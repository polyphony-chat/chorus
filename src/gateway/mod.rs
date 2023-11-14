pub mod gateway;
pub mod handle;
pub mod heartbeat;
pub mod message;

pub use gateway::*;
pub use handle::*;
use heartbeat::*;
pub use message::*;
use tokio_tungstenite::tungstenite::Message;

use crate::errors::GatewayError;
use crate::types::{self, Snowflake, WebSocketEvent};

use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::sleep_until;

use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::{Sink, StreamExt};
use futures_util::{SinkExt, Stream};
use log::{info, trace, warn};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::Instant;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async_tls_with_config, Connector, WebSocketStream};

use self::event::Events;

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

pub type ObservableObject = dyn Send + Sync + Any;

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

#[allow(clippy::type_complexity)]
#[async_trait]
pub trait GatewayCapable<R, S, G>
where
    R: Stream,
    S: Sink<Message>,
    G: GatewayHandleCapable<R, S>,
{
    fn get_events(&self) -> Arc<Mutex<Events>>;
    fn get_websocket_send(&self) -> Arc<Mutex<SplitSink<S, Message>>>;
    fn get_store(&self) -> GatewayStore;
    fn get_url(&self) -> String;
    /// Returns a Result with a matching impl of [`GatewayHandleCapable`], or a [`GatewayError`]
    ///
    /// DOCUMENTME: Explain what this method has to do to be a good get_handle() impl, or link to such documentation
    async fn get_handle(websocket_url: String) -> Result<G, GatewayError>;
    async fn close(&mut self);
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

            self.get_events().lock().await.error.notify(error).await;

            return;
        }

        let gateway_payload = msg.payload().unwrap();

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
                                let event = &mut self.get_events().lock().await.$($path).+;
                                let json = gateway_payload.event_data.unwrap().get();
                                match serde_json::from_str(json) {
                                    Err(err) => warn!("Failed to parse gateway event {event_name} ({err})"),
                                    Ok(message) => {
                                        $(
                                            let mut message: $message_type = message;
                                            let store = self.get_store().lock().await;
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

            self.heartbeat_handler
                .send
                .send(heartbeat_communication)
                .await
                .unwrap();
        }
    }
}

pub trait GatewayHandleCapable<R, S>
where
    R: Stream,
    S: Sink<Message>,
{
}

#[cfg(test)]
mod test {
    use crate::types;

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
