#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub mod default;
pub mod events;
pub mod message;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub mod wasm;

#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub use default::*;
pub use message::*;
use safina_timer::sleep_until;
use tokio::task::JoinHandle;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub use wasm::*;

use self::events::Events;
use crate::errors::GatewayError;
use crate::types::{
    self, AutoModerationRule, AutoModerationRuleUpdate, Channel, ChannelCreate, ChannelDelete,
    ChannelUpdate, Composite, Guild, GuildRoleCreate, GuildRoleUpdate, JsonField, RoleObject,
    Snowflake, SourceUrlField, ThreadUpdate, UpdateMessage, WebSocketEvent,
};

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{self, Duration, Instant};

use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::Sink;
use futures_util::SinkExt;
use log::{info, trace, warn};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

pub type GatewayStore = Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>;

/// The amount of time we wait for a heartbeat ack before resending our heartbeat in ms
const HEARTBEAT_ACK_TIMEOUT: u64 = 2000;

// Gateway opcodes
/// Opcode received when the server dispatches a [crate::types::WebSocketEvent]
const GATEWAY_DISPATCH: u8 = 0;
/// Opcode sent when sending a heartbeat
const GATEWAY_HEARTBEAT: u8 = 1;
/// Opcode sent to initiate a session
///
/// See [types::GatewayIdentifyPayload]
const GATEWAY_IDENTIFY: u8 = 2;
/// Opcode sent to update our presence
///
/// See [types::GatewayUpdatePresence]
const GATEWAY_UPDATE_PRESENCE: u8 = 3;
/// Opcode sent to update our state in vc
///
/// Like muting, deafening, leaving, joining..
///
/// See [types::UpdateVoiceState]
const GATEWAY_UPDATE_VOICE_STATE: u8 = 4;
/// Opcode sent to resume a session
///
/// See [types::GatewayResume]
const GATEWAY_RESUME: u8 = 6;
/// Opcode received to tell the client to reconnect
const GATEWAY_RECONNECT: u8 = 7;
/// Opcode sent to request guild member data
///
/// See [types::GatewayRequestGuildMembers]
const GATEWAY_REQUEST_GUILD_MEMBERS: u8 = 8;
/// Opcode received to tell the client their token / session is invalid
const GATEWAY_INVALID_SESSION: u8 = 9;
/// Opcode received when initially connecting to the gateway, starts our heartbeat
///
/// See [types::HelloData]
const GATEWAY_HELLO: u8 = 10;
/// Opcode received to acknowledge a heartbeat
const GATEWAY_HEARTBEAT_ACK: u8 = 11;
/// Opcode sent to get the voice state of users in a given DM/group channel
///
/// See [types::CallSync]
const GATEWAY_CALL_SYNC: u8 = 13;
/// Opcode sent to get data for a server (Lazy Loading request)
///
/// Sent by the official client when switching to a server
///
/// See [types::LazyRequest]
const GATEWAY_LAZY_REQUEST: u8 = 14;

pub trait MessageCapable: From<tokio_tungstenite::tungstenite::Message> {
    fn as_string(&self) -> Option<String>;
    fn as_bytes(&self) -> Option<Vec<u8>>;
    fn is_empty(&self) -> bool;
}

pub type ObservableObject = dyn Send + Sync + Any;

/// Used for communications between the heartbeat and gateway thread.
/// Either signifies a sequence number update, a heartbeat ACK or a Heartbeat request by the server
#[derive(Clone, Copy, Debug)]
pub struct HeartbeatThreadCommunication {
    /// The opcode for the communication we received, if relevant
    pub op_code: Option<u8>,
    /// The sequence number we got from discord, if any
    pub sequence_number: Option<u64>,
}

/// An entity type which is supposed to be updateable via the Gateway. This is implemented for all such types chorus supports, implementing it for your own types is likely a mistake.
pub trait Updateable: 'static + Send + Sync {
    fn id(&self) -> Snowflake;
}

/// Trait which defines the behavior of an Observer. An Observer is an object which is subscribed to
/// an Observable. The Observer is notified when the Observable's data changes.
/// In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
/// Note that `Debug` is used to tell `Observer`s apart when unsubscribing.
#[async_trait]
pub trait Observer<T>: Sync + Send + std::fmt::Debug {
    async fn update(&self, data: &T);
}

/// GatewayEvent is a wrapper around a WebSocketEvent. It is used to notify the observers of a
/// change in the WebSocketEvent. GatewayEvents are observable.
#[derive(Default, Debug)]
pub struct GatewayEvent<T: WebSocketEvent> {
    observers: Vec<Arc<dyn Observer<T>>>,
}

impl<T: WebSocketEvent> GatewayEvent<T> {
    /// Returns true if the GatewayEvent is observed by at least one Observer.
    pub fn is_observed(&self) -> bool {
        !self.observers.is_empty()
    }

    /// Subscribes an Observer to the GatewayEvent.
    pub fn subscribe(&mut self, observable: Arc<dyn Observer<T>>) {
        self.observers.push(observable);
    }

    /// Unsubscribes an Observer from the GatewayEvent.
    pub fn unsubscribe(&mut self, observable: &dyn Observer<T>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        // The usage of the debug format to compare the generic T of observers is quite stupid, but the only thing to compare between them is T and if T == T they are the same
        // anddd there is no way to do that without using format
        let to_remove = format!("{:?}", observable);
        self.observers
            .retain(|obs| format!("{:?}", obs) != to_remove);
    }

    /// Notifies the observers of the GatewayEvent.
    async fn notify(&self, new_event_data: T) {
        for observer in &self.observers {
            observer.update(&new_event_data).await;
        }
    }
}

#[async_trait]
pub trait GatewayCapable<T, S>
where
    T: MessageCapable + Send + 'static,
    S: Sink<T> + Send,
{
    fn get_events(&self) -> Arc<Mutex<Events>>;
    fn get_websocket_send(&self) -> Arc<Mutex<SplitSink<S, T>>>;
    fn get_store(&self) -> GatewayStore;
    fn get_url(&self) -> String;
    fn get_heartbeat_handler(&self) -> &HeartbeatHandler;
    /// Returns a Result with a matching impl of [`GatewayHandleCapable`], or a [`GatewayError`]
    ///
    /// DOCUMENTME: Explain what this method has to do to be a good get_handle() impl, or link to such documentation
    async fn spawn<G: GatewayHandleCapable<T, S>>(websocket_url: String)
        -> Result<G, GatewayError>;
    async fn close(&mut self);
    /// This handles a message as a websocket event and updates its events along with the events' observers
    async fn handle_message(&mut self, msg: GatewayMessage) {
        if msg.is_empty() {
            return;
        }

        if !msg.is_error() && !msg.is_payload() {
            warn!(
                "Message unrecognised: {:?}, please open an issue on the chorus github",
                msg.message.to_string()
            );
            return;
        }

        if msg.is_error() {
            let error = msg.error().unwrap();

            warn!("GW: Received error {:?}, connection will close..", error);

            self.close().await;

            let events = self.get_events();
            let events = events.lock().await;

            events.error.notify(error).await;

            return;
        }

        let gateway_payload = msg.payload().unwrap();
        println!("gateway payload: {:#?}", &gateway_payload);

        // See https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
        match gateway_payload.op_code {
            // An event was dispatched, we need to look at the gateway event name t
            GATEWAY_DISPATCH => {
                let Some(event_name) = gateway_payload.event_name else {
                    warn!("Gateway dispatch op without event_name");
                    return;
                };

                trace!("Gateway: Received {event_name}");

                macro_rules! handle {
                    ($($name:literal => $($path:ident).+ $( $message_type:ty: $update_type:ty)?),*) => {
                        match event_name.as_str() {
                            $($name => {
                                let events = self.get_events();
                                let event = &mut events.lock().await.$($path).+;
                                let json = gateway_payload.event_data.unwrap().get();
                                match serde_json::from_str(json) {
                                    Err(err) => warn!("Failed to parse gateway event {event_name} ({err})"),
                                    Ok(message) => {
                                        $(
                                            let mut message: $message_type = message;
                                            let store = self.get_store();
                                            let store = store.lock().await;
                                            let id = if message.id().is_some() {
                                                message.id().unwrap()
                                            } else {
                                                event.notify(message).await;
                                                return;
                                            };
                                            if let Some(to_update) = store.get(&id) {
                                                let object = to_update.clone();
                                                let inner_object = object.read().unwrap();
                                                if let Some(_) = inner_object.downcast_ref::<$update_type>() {
                                                    let ptr = Arc::into_raw(object.clone());
                                                    // SAFETY:
                                                    // - We have just checked that the typeid of the `dyn Any ...` matches that of `T`.
                                                    // - This operation doesn't read or write any shared data, and thus cannot cause a data race
                                                    // - The reference count is not being modified
                                                    let downcasted = unsafe { Arc::from_raw(ptr as *const RwLock<$update_type>).clone() };
                                                    drop(inner_object);
                                                    message.set_json(json.to_string());
                                                    message.set_source_url(self.get_url().clone());
                                                    message.update(downcasted.clone());
                                                } else {
                                                    warn!("Received {} for {}, but it has been observed to be a different type!", $name, id)
                                                }
                                            }
                                        )?
                                        event.notify(message).await;
                                    }
                                }
                            },)*
                            "RESUMED" => (),
                            "SESSIONS_REPLACE" => {
                                let result: Result<Vec<types::Session>, serde_json::Error> =
                                    serde_json::from_str(gateway_payload.event_data.unwrap().get());
                                match result {
                                    Err(err) => {
                                        warn!(
                                            "Failed to parse gateway event {} ({})",
                                            event_name,
                                            err
                                        );
                                        return;
                                    }
                                    Ok(sessions) => {
                                        let events = self.get_events();
                                        let events = events.lock().await;
                                        events.session.replace.notify(
                                            types::SessionsReplace {sessions}
                                        ).await;
                                    }
                                }
                            },
                            _ => {
                                warn!("Received unrecognized gateway event ({event_name})! Please open an issue on the chorus github so we can implement it");
                            }
                        }
                    };
                }

                // See https://discord.com/developers/docs/topics/gateway-events#receive-events
                // "Some" of these are undocumented
                handle!(
                    "READY" => session.ready,
                    "READY_SUPPLEMENTAL" => session.ready_supplemental,
                    "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => application.command_permissions_update,
                    "AUTO_MODERATION_RULE_CREATE" =>auto_moderation.rule_create,
                    "AUTO_MODERATION_RULE_UPDATE" =>auto_moderation.rule_update AutoModerationRuleUpdate: AutoModerationRule,
                    "AUTO_MODERATION_RULE_DELETE" => auto_moderation.rule_delete,
                    "AUTO_MODERATION_ACTION_EXECUTION" => auto_moderation.action_execution,
                    "CHANNEL_CREATE" => channel.create ChannelCreate: Guild,
                    "CHANNEL_UPDATE" => channel.update ChannelUpdate: Channel,
                    "CHANNEL_UNREAD_UPDATE" => channel.unread_update,
                    "CHANNEL_DELETE" => channel.delete ChannelDelete: Guild,
                    "CHANNEL_PINS_UPDATE" => channel.pins_update,
                    "CALL_CREATE" => call.create,
                    "CALL_UPDATE" => call.update,
                    "CALL_DELETE" => call.delete,
                    "THREAD_CREATE" => thread.create, // TODO
                    "THREAD_UPDATE" => thread.update ThreadUpdate: Channel,
                    "THREAD_DELETE" => thread.delete, // TODO
                    "THREAD_LIST_SYNC" => thread.list_sync, // TODO
                    "THREAD_MEMBER_UPDATE" => thread.member_update, // TODO
                    "THREAD_MEMBERS_UPDATE" => thread.members_update, // TODO
                    "GUILD_CREATE" => guild.create, // TODO
                    "GUILD_UPDATE" => guild.update, // TODO
                    "GUILD_DELETE" => guild.delete, // TODO
                    "GUILD_AUDIT_LOG_ENTRY_CREATE" => guild.audit_log_entry_create,
                    "GUILD_BAN_ADD" => guild.ban_add, // TODO
                    "GUILD_BAN_REMOVE" => guild.ban_remove, // TODO
                    "GUILD_EMOJIS_UPDATE" => guild.emojis_update, // TODO
                    "GUILD_STICKERS_UPDATE" => guild.stickers_update, // TODO
                    "GUILD_INTEGRATIONS_UPDATE" => guild.integrations_update,
                    "GUILD_MEMBER_ADD" => guild.member_add,
                    "GUILD_MEMBER_REMOVE" => guild.member_remove,
                    "GUILD_MEMBER_UPDATE" => guild.member_update, // TODO
                    "GUILD_MEMBERS_CHUNK" => guild.members_chunk, // TODO
                    "GUILD_ROLE_CREATE" => guild.role_create GuildRoleCreate: Guild,
                    "GUILD_ROLE_UPDATE" => guild.role_update GuildRoleUpdate: RoleObject,
                    "GUILD_ROLE_DELETE" => guild.role_delete, // TODO
                    "GUILD_SCHEDULED_EVENT_CREATE" => guild.role_scheduled_event_create, // TODO
                    "GUILD_SCHEDULED_EVENT_UPDATE" => guild.role_scheduled_event_update, // TODO
                    "GUILD_SCHEDULED_EVENT_DELETE" => guild.role_scheduled_event_delete, // TODO
                    "GUILD_SCHEDULED_EVENT_USER_ADD" => guild.role_scheduled_event_user_add,
                    "GUILD_SCHEDULED_EVENT_USER_REMOVE" => guild.role_scheduled_event_user_remove,
                    "PASSIVE_UPDATE_V1" => guild.passive_update_v1, // TODO
                    "INTEGRATION_CREATE" => integration.create, // TODO
                    "INTEGRATION_UPDATE" => integration.update, // TODO
                    "INTEGRATION_DELETE" => integration.delete, // TODO
                    "INTERACTION_CREATE" => interaction.create, // TODO
                    "INVITE_CREATE" => invite.create, // TODO
                    "INVITE_DELETE" => invite.delete, // TODO
                    "MESSAGE_CREATE" => message.create,
                    "MESSAGE_UPDATE" => message.update, // TODO
                    "MESSAGE_DELETE" => message.delete,
                    "MESSAGE_DELETE_BULK" => message.delete_bulk,
                    "MESSAGE_REACTION_ADD" => message.reaction_add, // TODO
                    "MESSAGE_REACTION_REMOVE" => message.reaction_remove, // TODO
                    "MESSAGE_REACTION_REMOVE_ALL" => message.reaction_remove_all, // TODO
                    "MESSAGE_REACTION_REMOVE_EMOJI" => message.reaction_remove_emoji, // TODO
                    "MESSAGE_ACK" => message.ack,
                    "PRESENCE_UPDATE" => user.presence_update, // TODO
                    "RELATIONSHIP_ADD" => relationship.add,
                    "RELATIONSHIP_REMOVE" => relationship.remove,
                    "STAGE_INSTANCE_CREATE" => stage_instance.create,
                    "STAGE_INSTANCE_UPDATE" => stage_instance.update, // TODO
                    "STAGE_INSTANCE_DELETE" => stage_instance.delete,
                    "TYPING_START" => user.typing_start,
                    "USER_UPDATE" => user.update, // TODO
                    "USER_GUILD_SETTINGS_UPDATE" => user.guild_settings_update,
                    "VOICE_STATE_UPDATE" => voice.state_update, // TODO
                    "VOICE_SERVER_UPDATE" => voice.server_update,
                    "WEBHOOKS_UPDATE" => webhooks.update
                );
            }
            // We received a heartbeat from the server
            // "Discord may send the app a Heartbeat (opcode 1) event, in which case the app should send a Heartbeat event immediately."
            GATEWAY_HEARTBEAT => {
                trace!("GW: Received Heartbeat // Heartbeat Request");

                // Tell the heartbeat handler it should send a heartbeat right away

                let heartbeat_communication = HeartbeatThreadCommunication {
                    sequence_number: gateway_payload.sequence_number,
                    op_code: Some(GATEWAY_HEARTBEAT),
                };

                let heartbeat_thread_communicator = self.get_heartbeat_handler().get_send();

                heartbeat_thread_communicator
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
            GATEWAY_RECONNECT => {
                todo!()
            }
            GATEWAY_INVALID_SESSION => {
                todo!()
            }
            // Starts our heartbeat
            // We should have already handled this in gateway init
            GATEWAY_HELLO => {
                warn!("Received hello when it was unexpected");
            }
            GATEWAY_HEARTBEAT_ACK => {
                trace!("GW: Received Heartbeat ACK");

                // Tell the heartbeat handler we received an ack

                let heartbeat_communication = HeartbeatThreadCommunication {
                    sequence_number: gateway_payload.sequence_number,
                    op_code: Some(GATEWAY_HEARTBEAT_ACK),
                };

                let heartbeat_handler = self.get_heartbeat_handler();
                let heartbeat_thread_communicator = heartbeat_handler.get_send();

                heartbeat_thread_communicator
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
            GATEWAY_IDENTIFY
            | GATEWAY_UPDATE_PRESENCE
            | GATEWAY_UPDATE_VOICE_STATE
            | GATEWAY_RESUME
            | GATEWAY_REQUEST_GUILD_MEMBERS
            | GATEWAY_CALL_SYNC
            | GATEWAY_LAZY_REQUEST => {
                info!(
                    "Received unexpected opcode ({}) for current state. This might be due to a faulty server implementation and is likely not the fault of chorus.",
                    gateway_payload.op_code
                );
            }
            _ => {
                warn!("Received unrecognized gateway op code ({})! Please open an issue on the chorus github so we can implement it", gateway_payload.op_code);
            }
        }

        // If we we received a seq number we should let it know
        if let Some(seq_num) = gateway_payload.sequence_number {
            let heartbeat_communication = HeartbeatThreadCommunication {
                sequence_number: Some(seq_num),
                // Op code is irrelevant here
                op_code: None,
            };

            let heartbeat_handler = self.get_heartbeat_handler();
            let heartbeat_thread_communicator = heartbeat_handler.get_send();
            heartbeat_thread_communicator
                .send(heartbeat_communication)
                .await
                .unwrap();
        }
    }
}

/// Handles sending heartbeats to the gateway in another thread
#[allow(dead_code)] // FIXME: Remove this, once HeartbeatHandler is used
#[derive(Debug)]
pub struct HeartbeatHandler {
    /// How ofter heartbeats need to be sent at a minimum
    pub heartbeat_interval: Duration,
    /// The send channel for the heartbeat thread
    pub send: Sender<HeartbeatThreadCommunication>,
    /// The handle of the thread
    handle: JoinHandle<()>,
}

impl HeartbeatHandler {
    pub async fn heartbeat_task<T: MessageCapable + Send + 'static, S: Sink<T> + Send>(
        websocket_tx: Arc<Mutex<SplitSink<S, T>>>,
        heartbeat_interval: Duration,
        mut receive: tokio::sync::mpsc::Receiver<HeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut last_heartbeat_timestamp: Instant = time::Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut last_seq_number: Option<u64> = None;
        safina_timer::start_timer_thread();

        loop {
            if kill_receive.try_recv().is_ok() {
                trace!("GW: Closing heartbeat task");
                break;
            }

            let timeout = if last_heartbeat_acknowledged {
                heartbeat_interval
            } else {
                // If the server hasn't acknowledged our heartbeat we should resend it
                Duration::from_millis(HEARTBEAT_ACK_TIMEOUT)
            };

            let mut should_send = false;

            tokio::select! {
                () = sleep_until(last_heartbeat_timestamp + timeout) => {
                    should_send = true;
                }
                Some(communication) = receive.recv() => {
                    // If we received a seq number update, use that as the last seq number
                    if communication.sequence_number.is_some() {
                        last_seq_number = communication.sequence_number;
                    }

                    if let Some(op_code) = communication.op_code {
                        match op_code {
                            GATEWAY_HEARTBEAT => {
                                // As per the api docs, if the server sends us a Heartbeat, that means we need to respond with a heartbeat immediately
                                should_send = true;
                            }
                            GATEWAY_HEARTBEAT_ACK => {
                                // The server received our heartbeat
                                last_heartbeat_acknowledged = true;
                            }
                            _ => {}
                        }
                    }
                }
            }

            if should_send {
                trace!("GW: Sending Heartbeat..");

                let heartbeat = types::GatewayHeartbeat {
                    op: GATEWAY_HEARTBEAT,
                    d: last_seq_number,
                };

                let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                let send_result = websocket_tx.lock().await.send(msg.into()).await;
                if send_result.is_err() {
                    // We couldn't send, the websocket is broken
                    warn!("GW: Couldnt send heartbeat, websocket seems broken");
                    break;
                }

                last_heartbeat_timestamp = time::Instant::now();
                last_heartbeat_acknowledged = false;
            }
        }
    }
}

#[async_trait(?Send)]
pub trait GatewayHandleCapable<T, S>
where
    T: MessageCapable + Send + 'static,
    S: Sink<T>,
{
    fn new(
        url: String,
        events: Arc<Mutex<Events>>,
        websocket_send: Arc<Mutex<SplitSink<S, T>>>,
        kill_send: tokio::sync::broadcast::Sender<()>,
        store: GatewayStore,
    ) -> Self;

    /// Sends json to the gateway with an opcode
    async fn send_json_event(&self, op_code: u8, to_send: serde_json::Value);

    /// Observes an Item `<T: Updateable>`, which will update itself, if new information about this
    /// item arrives on the corresponding Gateway Thread
    async fn observe<U: Updateable + Clone + std::fmt::Debug + Composite<U> + Send + Sync>(
        &self,
        object: Arc<RwLock<U>>,
    ) -> Arc<RwLock<U>>;

    /// Recursively observes and updates all updateable fields on the struct T. Returns an object `T`
    /// with all of its observable fields being observed.
    async fn observe_and_into_inner<U: Updateable + Clone + std::fmt::Debug + Composite<U>>(
        &self,
        object: Arc<RwLock<U>>,
    ) -> U {
        let channel = self.observe(object.clone()).await;
        let object = channel.read().unwrap().clone();
        object
    }

    /// Sends an identify event to the gateway
    async fn send_identify(&self, to_send: types::GatewayIdentifyPayload) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Identify..");

        self.send_json_event(GATEWAY_IDENTIFY, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    async fn send_update_presence(&self, to_send: types::UpdatePresence) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Update Presence..");

        self.send_json_event(GATEWAY_UPDATE_PRESENCE, to_send_value)
            .await;
    }

    /// Sends a resume event to the gateway
    async fn send_resume(&self, to_send: types::GatewayResume) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Resume..");

        self.send_json_event(GATEWAY_RESUME, to_send_value).await;
    }

    /// Sends a request guild members to the server
    async fn send_request_guild_members(&self, to_send: types::GatewayRequestGuildMembers) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Request Guild Members..");

        self.send_json_event(GATEWAY_REQUEST_GUILD_MEMBERS, to_send_value)
            .await;
    }

    /// Sends an update voice state to the server
    async fn send_update_voice_state(&self, to_send: types::UpdateVoiceState) {
        let to_send_value = serde_json::to_value(to_send).unwrap();

        trace!("GW: Sending Update Voice State..");

        self.send_json_event(GATEWAY_UPDATE_VOICE_STATE, to_send_value)
            .await;
    }

    /// Sends a call sync to the server
    async fn send_call_sync(&self, to_send: types::CallSync) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Call Sync..");

        self.send_json_event(GATEWAY_CALL_SYNC, to_send_value).await;
    }

    /// Sends a Lazy Request
    async fn send_lazy_request(&self, to_send: types::LazyRequest) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Lazy Request..");

        self.send_json_event(GATEWAY_LAZY_REQUEST, to_send_value)
            .await;
    }

    /// Closes the websocket connection and stops all gateway tasks;
    ///
    /// Esentially pulls the plug on the gateway, leaving it possible to resume;
    async fn close(&self);
}

#[async_trait]
// TODO: Make me not a trait!!
pub trait HeartbeatHandlerCapable<T: MessageCapable + Send + 'static, S: Sink<T>> {
    fn get_send(&self) -> &Sender<HeartbeatThreadCommunication>;
    fn get_heartbeat_interval(&self) -> Duration;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        heartbeat_interval: Duration,
        websocket_tx: Arc<Mutex<SplitSink<S, T>>>,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> HeartbeatHandler;
}
