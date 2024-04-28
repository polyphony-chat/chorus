// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use log::*;
#[cfg(not(target_arch = "wasm32"))]
use tokio::task;

use super::events::Events;
use super::*;
use super::{Sink, Stream};
use crate::types::{
    self, AutoModerationRule, AutoModerationRuleUpdate, Channel, ChannelCreate, ChannelDelete,
    ChannelUpdate, GatewayInvalidSession, GatewayReconnect, Guild, GuildRoleCreate,
    GuildRoleUpdate, JsonField, RoleObject, SourceUrlField, ThreadUpdate, UpdateMessage,
    WebSocketEvent,
};

#[derive(Debug)]
pub struct Gateway {
    events: Arc<Mutex<Events>>,
    heartbeat_handler: HeartbeatHandler,
    websocket_send: Arc<Mutex<Sink>>,
    websocket_receive: Stream,
    kill_send: tokio::sync::broadcast::Sender<()>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
    store: Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>,
    url: String,
}

impl Gateway {
    #[allow(clippy::new_ret_no_self)]
    pub async fn spawn(websocket_url: String) -> Result<GatewayHandle, GatewayError> {
        let (websocket_send, mut websocket_receive) =
            WebSocketBackend::connect(&websocket_url).await?;

        let shared_websocket_send = Arc::new(Mutex::new(websocket_send));

        // Create a shared broadcast channel for killing all gateway tasks
        let (kill_send, mut _kill_receive) = tokio::sync::broadcast::channel::<()>(16);

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        #[cfg(not(target_arch = "wasm32"))]
        let msg: GatewayMessage = websocket_receive.next().await.unwrap().unwrap().into();
        #[cfg(target_arch = "wasm32")]
        let msg: GatewayMessage = websocket_receive.next().await.unwrap().into();
        let gateway_payload: types::GatewayReceivePayload = serde_json::from_str(&msg.0).unwrap();

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
            kill_receive: kill_send.subscribe(),
            store: store.clone(),
            url: websocket_url.clone(),
        };

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        #[cfg(not(target_arch = "wasm32"))]
        task::spawn(async move {
            gateway.gateway_listen_task().await;
        });
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            gateway.gateway_listen_task().await;
        });

        Ok(GatewayHandle {
            url: websocket_url.clone(),
            events: shared_events,
            websocket_send: shared_websocket_send.clone(),
            kill_send: kill_send.clone(),
            store,
        })
    }

    /// The main gateway listener task;
    pub async fn gateway_listen_task(&mut self) {
        loop {
            let msg;

            tokio::select! {
                Ok(_) = self.kill_receive.recv() => {
                    log::trace!("GW: Closing listener task");
                    break;
                }
                message = self.websocket_receive.next() => {
                    msg = message;
                }
            }

            // PRETTYFYME: Remove inline conditional compiling
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(Ok(message)) = msg {
                self.handle_message(message.into()).await;
                continue;
            }
            #[cfg(target_arch = "wasm32")]
            if let Some(message) = msg {
                self.handle_message(message.into()).await;
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
        if msg.0.is_empty() {
            return;
        }

        let Ok(gateway_payload) = msg.payload() else {
            if let Some(error) = msg.error() {
                warn!("GW: Received error {:?}, connection will close..", error);
                self.close().await;
                self.events.lock().await.error.notify(error).await;
            } else {
                warn!(
                    "Message unrecognised: {:?}, please open an issue on the chorus github",
                    msg.0
                );
            }
            return;
        };

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
                                                    message.set_source_url(self.url.clone());
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
                trace!("GW: Received Reconnect");

                let reconnect = GatewayReconnect {};

                self.events
                    .lock()
                    .await
                    .session
                    .reconnect
                    .notify(reconnect)
                    .await;
            }
            GATEWAY_INVALID_SESSION => {
                trace!("GW: Received Invalid Session");

                let mut resumable: bool = false;

                if let Some(raw_value) = gateway_payload.event_data {
                    if let Ok(deserialized) = serde_json::from_str(raw_value.get()) {
                        resumable = deserialized;
                    } else {
                        warn!("Failed to parse part of INVALID_SESSION ('{}' as bool), assuming non-resumable", raw_value.get());
                    }
                } else {
                    warn!("Failed to parse part of INVALID_SESSION ('d' missing), assuming non-resumable");
                }

                let invalid_session = GatewayInvalidSession { resumable };

                self.events
                    .lock()
                    .await
                    .session
                    .invalid
                    .notify(invalid_session)
                    .await;
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
