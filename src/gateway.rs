use crate::errors::GatewayError;
use crate::gateway::events::Events;
use crate::types::{
    self, AutoModerationRule, AutoModerationRuleUpdate, Channel, ChannelCreate, ChannelUpdate,
    Composite, Guild, GuildRoleCreate, GuildRoleUpdate, JsonField, RoleObject, Snowflake,
    UpdateMessage, WebSocketEvent,
};
use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::sleep_until;

use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use log::{info, trace, warn};
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::Instant;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async_tls_with_config, Connector, WebSocketStream};

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

/// The amount of time we wait for a heartbeat ack before resending our heartbeat in ms
const HEARTBEAT_ACK_TIMEOUT: u64 = 2000;

/// Represents a messsage received from the gateway. This will be either a [types::GatewayReceivePayload], containing events, or a [GatewayError].
/// This struct is used internally when handling messages.
#[derive(Clone, Debug)]
pub struct GatewayMessage {
    /// The message we received from the server
    message: tokio_tungstenite::tungstenite::Message,
}

impl GatewayMessage {
    /// Creates self from a tungstenite message
    pub fn from_tungstenite_message(message: tokio_tungstenite::tungstenite::Message) -> Self {
        Self { message }
    }

    /// Parses the message as an error;
    /// Returns the error if succesfully parsed, None if the message isn't an error
    pub fn error(&self) -> Option<GatewayError> {
        let content = self.message.to_string();

        // Some error strings have dots on the end, which we don't care about
        let processed_content = content.to_lowercase().replace('.', "");

        match processed_content.as_str() {
            "unknown error" | "4000" => Some(GatewayError::Unknown),
            "unknown opcode" | "4001" => Some(GatewayError::UnknownOpcode),
            "decode error" | "error while decoding payload" | "4002" => Some(GatewayError::Decode),
            "not authenticated" | "4003" => Some(GatewayError::NotAuthenticated),
            "authentication failed" | "4004" => Some(GatewayError::AuthenticationFailed),
            "already authenticated" | "4005" => Some(GatewayError::AlreadyAuthenticated),
            "invalid seq" | "4007" => Some(GatewayError::InvalidSequenceNumber),
            "rate limited" | "4008" => Some(GatewayError::RateLimited),
            "session timed out" | "4009" => Some(GatewayError::SessionTimedOut),
            "invalid shard" | "4010" => Some(GatewayError::InvalidShard),
            "sharding required" | "4011" => Some(GatewayError::ShardingRequired),
            "invalid api version" | "4012" => Some(GatewayError::InvalidAPIVersion),
            "invalid intent(s)" | "invalid intent" | "4013" => Some(GatewayError::InvalidIntents),
            "disallowed intent(s)" | "disallowed intents" | "4014" => {
                Some(GatewayError::DisallowedIntents)
            }
            _ => None,
        }
    }

    /// Returns whether or not the message is an error
    pub fn is_error(&self) -> bool {
        self.error().is_some()
    }

    /// Parses the message as a payload;
    /// Returns a result of deserializing
    pub fn payload(&self) -> Result<types::GatewayReceivePayload, serde_json::Error> {
        return serde_json::from_str(self.message.to_text().unwrap());
    }

    /// Returns whether or not the message is a payload
    pub fn is_payload(&self) -> bool {
        // close messages are never payloads, payloads are only text messages
        if self.message.is_close() | !self.message.is_text() {
            return false;
        }

        return self.payload().is_ok();
    }

    /// Returns whether or not the message is empty
    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }
}

pub type ObservableObject = dyn Send + Sync + Any;

/// Represents a handle to a Gateway connection. A Gateway connection will create observable
/// [`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
/// implemented types with the trait [`WebSocketEvent`]
/// Using this handle you can also send Gateway Events directly.
#[derive(Debug)]
pub struct GatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<Events>>,
    pub websocket_send: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
    pub handle: JoinHandle<()>,
    /// Tells gateway tasks to close
    kill_send: tokio::sync::broadcast::Sender<()>,
    pub(crate) store: Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>,
}

/// An entity type which is supposed to be updateable via the Gateway. This is implemented for all such types chorus supports, implementing it for your own types is likely a mistake.
pub trait Updateable: 'static + Send + Sync {
    fn id(&self) -> Snowflake;
}

impl GatewayHandle {
    /// Sends json to the gateway with an opcode
    async fn send_json_event(&self, op_code: u8, to_send: serde_json::Value) {
        let gateway_payload = types::GatewaySendPayload {
            op_code,
            event_data: Some(to_send),
            sequence_number: None,
        };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();

        let message = tokio_tungstenite::tungstenite::Message::text(payload_json);

        self.websocket_send
            .lock()
            .await
            .send(message)
            .await
            .unwrap();
    }

    pub async fn observe<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Arc<RwLock<T>>,
    ) -> Arc<RwLock<T>> {
        let mut store = self.store.lock().await;
        let id = object.read().unwrap().id();
        if let Some(channel) = store.get(&id) {
            let object = channel.clone();
            drop(store);
            object
                .read()
                .unwrap()
                .downcast_ref::<T>()
                .unwrap_or_else(|| {
                    panic!(
                        "Snowflake {} already exists in the store, but it is not of type T.",
                        id
                    )
                });
            let ptr = Arc::into_raw(object.clone());
            // SAFETY:
            // - We have just checked that the typeid of the `dyn Any ...` matches that of `T`.
            // - This operation doesn't read or write any shared data, and thus cannot cause a data race
            // - The reference count is not being modified
            let downcasted = unsafe { Arc::from_raw(ptr as *const RwLock<T>).clone() };
            let object = downcasted.read().unwrap().clone();

            let watched_object = object.watch_whole(self).await;
            *downcasted.write().unwrap() = watched_object;
            downcasted
        } else {
            let id = object.read().unwrap().id();
            let object = object.read().unwrap().clone();
            let object = object.clone().watch_whole(self).await;
            let wrapped = Arc::new(RwLock::new(object));
            store.insert(id, wrapped.clone());
            wrapped
        }
    }

    /// Recursively observes and updates all updateable fields on the struct T. Returns an object `T`
    /// with all of its observable fields being observed.
    pub async fn observe_and_into_inner<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Arc<RwLock<T>>,
    ) -> T {
        let channel = self.observe(object.clone()).await;
        let object = channel.read().unwrap().clone();
        object
    }

    /// Sends an identify event to the gateway
    pub async fn send_identify(&self, to_send: types::GatewayIdentifyPayload) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Identify..");

        self.send_json_event(GATEWAY_IDENTIFY, to_send_value).await;
    }

    /// Sends a resume event to the gateway
    pub async fn send_resume(&self, to_send: types::GatewayResume) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Resume..");

        self.send_json_event(GATEWAY_RESUME, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    pub async fn send_update_presence(&self, to_send: types::UpdatePresence) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Update Presence..");

        self.send_json_event(GATEWAY_UPDATE_PRESENCE, to_send_value)
            .await;
    }

    /// Sends a request guild members to the server
    pub async fn send_request_guild_members(&self, to_send: types::GatewayRequestGuildMembers) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Request Guild Members..");

        self.send_json_event(GATEWAY_REQUEST_GUILD_MEMBERS, to_send_value)
            .await;
    }

    /// Sends an update voice state to the server
    pub async fn send_update_voice_state(&self, to_send: types::UpdateVoiceState) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Update Voice State..");

        self.send_json_event(GATEWAY_UPDATE_VOICE_STATE, to_send_value)
            .await;
    }

    /// Sends a call sync to the server
    pub async fn send_call_sync(&self, to_send: types::CallSync) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Call Sync..");

        self.send_json_event(GATEWAY_CALL_SYNC, to_send_value).await;
    }

    /// Sends a Lazy Request
    pub async fn send_lazy_request(&self, to_send: types::LazyRequest) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Lazy Request..");

        self.send_json_event(GATEWAY_LAZY_REQUEST, to_send_value)
            .await;
    }

    /// Closes the websocket connection and stops all gateway tasks;
    ///
    /// Esentially pulls the plug on the gateway, leaving it possible to resume;
    pub async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}

pub struct Gateway {
    events: Arc<Mutex<Events>>,
    heartbeat_handler: HeartbeatHandler,
    websocket_send: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
    websocket_receive: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    kill_send: tokio::sync::broadcast::Sender<()>,
    store: Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>,
}

impl Gateway {
    #[allow(clippy::new_ret_no_self)]
    pub async fn new(websocket_url: String) -> Result<GatewayHandle, GatewayError> {
        let (websocket_stream, _) = match connect_async_tls_with_config(
            &websocket_url,
            None,
            false,
            Some(Connector::NativeTls(
                TlsConnector::builder().build().unwrap(),
            )),
        )
        .await
        {
            Ok(websocket_stream) => websocket_stream,
            Err(e) => {
                return Err(GatewayError::CannotConnect {
                    error: e.to_string(),
                })
            }
        };

        let (websocket_send, mut websocket_receive) = websocket_stream.split();

        let shared_websocket_send = Arc::new(Mutex::new(websocket_send));

        // Create a shared broadcast channel for killing all gateway tasks
        let (kill_send, mut _kill_receive) = tokio::sync::broadcast::channel::<()>(16);

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        let msg = websocket_receive.next().await.unwrap().unwrap();
        let gateway_payload: types::GatewayReceivePayload =
            serde_json::from_str(msg.to_text().unwrap()).unwrap();

        if gateway_payload.op_code != GATEWAY_HELLO {
            return Err(GatewayError::NonHelloOnInitiate {
                opcode: gateway_payload.op_code,
            });
        }

        info!("GW: Received Hello");

        let gateway_hello: types::HelloData =
            serde_json::from_str(gateway_payload.event_data.unwrap().get()).unwrap();

        let events = Events::default();
        let shared_events = Arc::new(Mutex::new(events));

        let store = Arc::new(Mutex::new(HashMap::new()));

        let mut gateway = Gateway {
            events: shared_events.clone(),
            heartbeat_handler: HeartbeatHandler::new(
                Duration::from_millis(gateway_hello.heartbeat_interval),
                shared_websocket_send.clone(),
                kill_send.subscribe(),
            ),
            websocket_send: shared_websocket_send.clone(),
            websocket_receive,
            kill_send: kill_send.clone(),
            store: store.clone(),
        };

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        let handle: JoinHandle<()> = task::spawn(async move {
            gateway.gateway_listen_task().await;
        });

        Ok(GatewayHandle {
            url: websocket_url.clone(),
            events: shared_events,
            websocket_send: shared_websocket_send.clone(),
            handle,
            kill_send: kill_send.clone(),
            store,
        })
    }

    /// The main gateway listener task;
    ///
    /// Can only be stopped by closing the websocket, cannot be made to listen for kill
    pub async fn gateway_listen_task(&mut self) {
        loop {
            let msg = self.websocket_receive.next().await;

            // This if chain can be much better but if let is unstable on stable rust
            if let Some(Ok(message)) = msg {
                self.handle_message(GatewayMessage::from_tungstenite_message(message))
                    .await;
                continue;
            }

            // We couldn't receive the next message or it was an error, something is wrong with the websocket, close
            warn!("GW: Websocket is broken, stopping gateway");
            break;
        }
    }

    /// Closes the websocket connection and stops all tasks
    async fn close(&mut self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }

    /// Deserializes and updates a dispatched event, when we already know its type;
    /// (Called for every event in handle_message)
    #[allow(dead_code)] // TODO: Remove this allow annotation
    async fn handle_event<'a, T: WebSocketEvent + serde::Deserialize<'a>>(
        data: &'a str,
        event: &mut GatewayEvent<T>,
    ) -> Result<(), serde_json::Error> {
        let data_deserialize_result: Result<T, serde_json::Error> = serde_json::from_str(data);

        if data_deserialize_result.is_err() {
            return Err(data_deserialize_result.err().unwrap());
        }

        event.notify(data_deserialize_result.unwrap()).await;
        Ok(())
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    pub async fn handle_message(&mut self, msg: GatewayMessage) {
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

        // Todo: handle errors in a good way, maybe observers like events?
        if msg.is_error() {
            warn!("GW: Received error, connection will close..");

            let _error = msg.error();

            self.close().await;
            return;
        }

        let gateway_payload = msg.payload().unwrap();

        // See https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
        match gateway_payload.op_code {
            // An event was dispatched, we need to look at the gateway event name t
            GATEWAY_DISPATCH => {
                let Some(event_name) = gateway_payload.event_name else {
                    warn!("Gateway dispatch op without event_name");
                    return
                };

                trace!("Gateway: Received {event_name}");

                macro_rules! handle {
                    ($($name:literal => $($path:ident).+ $( $message_type:ty: $update_type:ty)?),*) => {
                        match event_name.as_str() {
                            $($name => {
                                let event = &mut self.events.lock().await.$($path).+;
                                let json = gateway_payload.event_data.unwrap().get();
                                match serde_json::from_str(json) {
                                    Err(err) => warn!("Failed to parse gateway event {event_name} ({err})"),
                                    Ok(message) => {
                                        $(
                                            let mut message: $message_type = message;
                                            let store = self.store.lock().await;
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
                                        self.events.lock().await.session.replace.notify(
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
                    "CHANNEL_CREATE" => channel.create, // Could be processed if handle_message returned a Result<(), SomeError>, as channel.guild_id is Option
                    "CHANNEL_UPDATE" => channel.update ChannelUpdate: Channel,
                    "CHANNEL_UNREAD_UPDATE" => channel.unread_update,
                    "CHANNEL_DELETE" => channel.delete, // Same as CHANNEL_CREATE
                    "CHANNEL_PINS_UPDATE" => channel.pins_update,
                    "CALL_CREATE" => call.create,
                    "CALL_UPDATE" => call.update,
                    "CALL_DELETE" => call.delete,
                    "THREAD_CREATE" => thread.create, // TODO
                    "THREAD_UPDATE" => thread.update, // TODO
                    "THREAD_DELETE" => thread.delete, // TODO
                    "THREAD_LIST_SYNC" => thread.list_sync, // TODO
                    "THREAD_MEMBER_UPDATE" => thread.member_update, // TODO
                    "THREAD_MEMBERS_UPDATE" => thread.members_update, // TODO
                    "GUILD_CREATE" => guild.create, // TODO
                    "GUILD_UPDATE" => guild.update, // TODO
                    "GUILD_DELETE" => guild.delete, // TODO
                    "GUILD_AUDIT_LOG_ENTRY_CREATE" => guild.audit_log_entry_create,
                    "GUILD_BAN_ADD" => guild.ban_add,
                    "GUILD_BAN_REMOVE" => guild.ban_remove,
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

                self.heartbeat_handler
                    .send
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

                self.heartbeat_handler
                    .send
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
                let error = GatewayError::UnexpectedOpcodeReceived {
                    opcode: gateway_payload.op_code,
                };
                Err::<(), GatewayError>(error).unwrap();
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

            self.heartbeat_handler
                .send
                .send(heartbeat_communication)
                .await
                .unwrap();
        }
    }
}

/// Handles sending heartbeats to the gateway in another thread
#[allow(dead_code)] // FIXME: Remove this, once HeartbeatHandler is used
struct HeartbeatHandler {
    /// How ofter heartbeats need to be sent at a minimum
    pub heartbeat_interval: Duration,
    /// The send channel for the heartbeat thread
    pub send: Sender<HeartbeatThreadCommunication>,
    /// The handle of the thread
    handle: JoinHandle<()>,
}

impl HeartbeatHandler {
    pub fn new(
        heartbeat_interval: Duration,
        websocket_tx: Arc<
            Mutex<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tokio_tungstenite::tungstenite::Message,
                >,
            >,
        >,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> HeartbeatHandler {
        let (send, receive) = tokio::sync::mpsc::channel(32);
        let kill_receive = kill_rc.resubscribe();

        let handle: JoinHandle<()> = task::spawn(async move {
            HeartbeatHandler::heartbeat_task(
                websocket_tx,
                heartbeat_interval,
                receive,
                kill_receive,
            )
            .await;
        });

        Self {
            heartbeat_interval,
            send,
            handle,
        }
    }

    /// The main heartbeat task;
    ///
    /// Can be killed by the kill broadcast;
    /// If the websocket is closed, will die out next time it tries to send a heartbeat;
    pub async fn heartbeat_task(
        websocket_tx: Arc<
            Mutex<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tokio_tungstenite::tungstenite::Message,
                >,
            >,
        >,
        heartbeat_interval: Duration,
        mut receive: tokio::sync::mpsc::Receiver<HeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut last_heartbeat_timestamp: Instant = time::Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut last_seq_number: Option<u64> = None;

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

                let send_result = websocket_tx.lock().await.send(msg).await;
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

/// Used for communications between the heartbeat and gateway thread.
/// Either signifies a sequence number update, a heartbeat ACK or a Heartbeat request by the server
#[derive(Clone, Copy, Debug)]
struct HeartbeatThreadCommunication {
    /// The opcode for the communication we received, if relevant
    op_code: Option<u8>,
    /// The sequence number we got from discord, if any
    sequence_number: Option<u64>,
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

mod events {
    use super::*;

    #[derive(Default, Debug)]
    pub struct Events {
        pub application: Application,
        pub auto_moderation: AutoModeration,
        pub session: Session,
        pub message: Message,
        pub user: User,
        pub relationship: Relationship,
        pub channel: Channel,
        pub thread: Thread,
        pub guild: Guild,
        pub invite: Invite,
        pub integration: Integration,
        pub interaction: Interaction,
        pub stage_instance: StageInstance,
        pub call: Call,
        pub voice: Voice,
        pub webhooks: Webhooks,
        pub gateway_identify_payload: GatewayEvent<types::GatewayIdentifyPayload>,
        pub gateway_resume: GatewayEvent<types::GatewayResume>,
    }

    #[derive(Default, Debug)]
    pub struct Application {
        pub command_permissions_update: GatewayEvent<types::ApplicationCommandPermissionsUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct AutoModeration {
        pub rule_create: GatewayEvent<types::AutoModerationRuleCreate>,
        pub rule_update: GatewayEvent<types::AutoModerationRuleUpdate>,
        pub rule_delete: GatewayEvent<types::AutoModerationRuleDelete>,
        pub action_execution: GatewayEvent<types::AutoModerationActionExecution>,
    }

    #[derive(Default, Debug)]
    pub struct Session {
        pub ready: GatewayEvent<types::GatewayReady>,
        pub ready_supplemental: GatewayEvent<types::GatewayReadySupplemental>,
        pub replace: GatewayEvent<types::SessionsReplace>,
    }

    #[derive(Default, Debug)]
    pub struct StageInstance {
        pub create: GatewayEvent<types::StageInstanceCreate>,
        pub update: GatewayEvent<types::StageInstanceUpdate>,
        pub delete: GatewayEvent<types::StageInstanceDelete>,
    }

    #[derive(Default, Debug)]
    pub struct Message {
        pub create: GatewayEvent<types::MessageCreate>,
        pub update: GatewayEvent<types::MessageUpdate>,
        pub delete: GatewayEvent<types::MessageDelete>,
        pub delete_bulk: GatewayEvent<types::MessageDeleteBulk>,
        pub reaction_add: GatewayEvent<types::MessageReactionAdd>,
        pub reaction_remove: GatewayEvent<types::MessageReactionRemove>,
        pub reaction_remove_all: GatewayEvent<types::MessageReactionRemoveAll>,
        pub reaction_remove_emoji: GatewayEvent<types::MessageReactionRemoveEmoji>,
        pub ack: GatewayEvent<types::MessageACK>,
    }

    #[derive(Default, Debug)]
    pub struct User {
        pub update: GatewayEvent<types::UserUpdate>,
        pub guild_settings_update: GatewayEvent<types::UserGuildSettingsUpdate>,
        pub presence_update: GatewayEvent<types::PresenceUpdate>,
        pub typing_start: GatewayEvent<types::TypingStartEvent>,
    }

    #[derive(Default, Debug)]
    pub struct Relationship {
        pub add: GatewayEvent<types::RelationshipAdd>,
        pub remove: GatewayEvent<types::RelationshipRemove>,
    }

    #[derive(Default, Debug)]
    pub struct Channel {
        pub create: GatewayEvent<types::ChannelCreate>,
        pub update: GatewayEvent<types::ChannelUpdate>,
        pub unread_update: GatewayEvent<types::ChannelUnreadUpdate>,
        pub delete: GatewayEvent<types::ChannelDelete>,
        pub pins_update: GatewayEvent<types::ChannelPinsUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct Thread {
        pub create: GatewayEvent<types::ThreadCreate>,
        pub update: GatewayEvent<types::ThreadUpdate>,
        pub delete: GatewayEvent<types::ThreadDelete>,
        pub list_sync: GatewayEvent<types::ThreadListSync>,
        pub member_update: GatewayEvent<types::ThreadMemberUpdate>,
        pub members_update: GatewayEvent<types::ThreadMembersUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct Guild {
        pub create: GatewayEvent<types::GuildCreate>,
        pub update: GatewayEvent<types::GuildUpdate>,
        pub delete: GatewayEvent<types::GuildDelete>,
        pub audit_log_entry_create: GatewayEvent<types::GuildAuditLogEntryCreate>,
        pub ban_add: GatewayEvent<types::GuildBanAdd>,
        pub ban_remove: GatewayEvent<types::GuildBanRemove>,
        pub emojis_update: GatewayEvent<types::GuildEmojisUpdate>,
        pub stickers_update: GatewayEvent<types::GuildStickersUpdate>,
        pub integrations_update: GatewayEvent<types::GuildIntegrationsUpdate>,
        pub member_add: GatewayEvent<types::GuildMemberAdd>,
        pub member_remove: GatewayEvent<types::GuildMemberRemove>,
        pub member_update: GatewayEvent<types::GuildMemberUpdate>,
        pub members_chunk: GatewayEvent<types::GuildMembersChunk>,
        pub role_create: GatewayEvent<types::GuildRoleCreate>,
        pub role_update: GatewayEvent<types::GuildRoleUpdate>,
        pub role_delete: GatewayEvent<types::GuildRoleDelete>,
        pub role_scheduled_event_create: GatewayEvent<types::GuildScheduledEventCreate>,
        pub role_scheduled_event_update: GatewayEvent<types::GuildScheduledEventUpdate>,
        pub role_scheduled_event_delete: GatewayEvent<types::GuildScheduledEventDelete>,
        pub role_scheduled_event_user_add: GatewayEvent<types::GuildScheduledEventUserAdd>,
        pub role_scheduled_event_user_remove: GatewayEvent<types::GuildScheduledEventUserRemove>,
        pub passive_update_v1: GatewayEvent<types::PassiveUpdateV1>,
    }

    #[derive(Default, Debug)]
    pub struct Invite {
        pub create: GatewayEvent<types::InviteCreate>,
        pub delete: GatewayEvent<types::InviteDelete>,
    }

    #[derive(Default, Debug)]
    pub struct Integration {
        pub create: GatewayEvent<types::IntegrationCreate>,
        pub update: GatewayEvent<types::IntegrationUpdate>,
        pub delete: GatewayEvent<types::IntegrationDelete>,
    }

    #[derive(Default, Debug)]
    pub struct Interaction {
        pub create: GatewayEvent<types::InteractionCreate>,
    }

    #[derive(Default, Debug)]
    pub struct Call {
        pub create: GatewayEvent<types::CallCreate>,
        pub update: GatewayEvent<types::CallUpdate>,
        pub delete: GatewayEvent<types::CallDelete>,
    }

    #[derive(Default, Debug)]
    pub struct Voice {
        pub state_update: GatewayEvent<types::VoiceStateUpdate>,
        pub server_update: GatewayEvent<types::VoiceServerUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct Webhooks {
        pub update: GatewayEvent<types::WebhooksUpdate>,
    }
}

#[cfg(test)]
mod example {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

    #[derive(Debug)]
    struct Consumer {
        _name: String,
        events_received: AtomicI32,
    }

    #[async_trait]
    impl Observer<types::GatewayResume> for Consumer {
        async fn update(&self, _data: &types::GatewayResume) {
            self.events_received.fetch_add(1, Relaxed);
        }
    }

    #[tokio::test]
    async fn test_observer_behavior() {
        let mut event = GatewayEvent::default();

        let new_data = types::GatewayResume {
            token: "token_3276ha37am3".to_string(),
            session_id: "89346671230".to_string(),
            seq: "3".to_string(),
        };

        let consumer = Arc::new(Consumer {
            _name: "first".into(),
            events_received: 0.into(),
        });
        event.subscribe(consumer.clone());

        let second_consumer = Arc::new(Consumer {
            _name: "second".into(),
            events_received: 0.into(),
        });
        event.subscribe(second_consumer.clone());

        event.notify(new_data.clone()).await;
        event.unsubscribe(&*consumer);
        event.notify(new_data).await;

        assert_eq!(consumer.events_received.load(Relaxed), 1);
        assert_eq!(second_consumer.events_received.load(Relaxed), 2);
    }
}
