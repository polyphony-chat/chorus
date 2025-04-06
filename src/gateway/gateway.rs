// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;

use flate2::Decompress;
use futures_util::{SinkExt, StreamExt};
use log::*;
use pubserve::Publisher;
#[cfg(not(target_arch = "wasm32"))]
use tokio::task;

use super::events::Events;
use super::*;
use super::{Sink, Stream};
use crate::types::{
    self, AutoModerationRule, AutoModerationRuleUpdate, Channel, ChannelCreate, ChannelDelete,
    ChannelUpdate, CloseCode, GatewayInvalidSession, GatewayReconnect, Guild, GuildRoleCreate,
    GuildRoleUpdate, JsonField, Opcode, RoleObject, SourceUrlField, ThreadUpdate, UpdateMessage,
    WebSocketEvent,
};

// Needed to observe close codes
#[cfg(target_arch = "wasm32")]
use pharos::Observable;

/// Tells us we have received enough of the buffer to decompress it
const ZLIB_SUFFIX: [u8; 4] = [0, 0, 255, 255];

#[derive(Debug)]
pub struct Gateway {
    events: Arc<Mutex<Events>>,
    heartbeat_handler: HeartbeatHandler,
    websocket_send: Arc<Mutex<Sink>>,
    websocket_receive: Stream,
    kill_send: tokio::sync::broadcast::Sender<()>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
    store: Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>,
    /// Url which was used to initialize the gateway
    url: String,
    /// Options which were used to initialize the gateway
    options: GatewayOptions,
    zlib_inflate: Option<flate2::Decompress>,
    zlib_buffer: Option<Vec<u8>>,
}

impl Gateway {
    #[allow(clippy::new_ret_no_self)]
    /// Creates / opens a new gateway connection.
    ///
    /// # Note
    /// The websocket url should begin with the prefix wss:// or ws:// (for unsecure connections)
    pub async fn spawn(
        websocket_url: &str,
        options: GatewayOptions,
    ) -> Result<GatewayHandle, GatewayError> {
        let url = options.add_to_url(websocket_url);

        debug!("GW: Connecting to {}", url);

        let (websocket_send, mut websocket_receive) = match WebSocketBackend::connect(&url).await {
            Ok(streams) => streams,
            Err(e) => {
                return Err(GatewayError::CannotConnect {
                    error: format!("{:?}", e),
                });
            }
        };

        let shared_websocket_send = Arc::new(Mutex::new(websocket_send));

        // Create a shared broadcast channel for killing all gateway tasks
        let (kill_send, mut _kill_receive) = tokio::sync::broadcast::channel::<()>(16);

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        #[cfg(not(target_arch = "wasm32"))]
        let received: RawGatewayMessage = {
            // Note: The tungstenite backend handles close codes as messages, while the ws_stream_wasm one handles them differently.
            //
            // Hence why wasm receives straight RawGatewayMessages, and tungstenite receives
            // GatewayCommunications.
            let communication: GatewayCommunication =
                websocket_receive.next().await.unwrap().unwrap().into();

            match communication {
                GatewayCommunication::Message(message) => message,
                GatewayCommunication::Error(error) => return Err(error.into()),
            }
        };
        #[cfg(target_arch = "wasm32")]
        let received: RawGatewayMessage = websocket_receive.0.next().await.unwrap().into();

        let message: GatewayMessage;

        let zlib_buffer;
        let zlib_inflate;

        match options.transport_compression {
            GatewayTransportCompression::None => {
                zlib_buffer = None;
                zlib_inflate = None;
                message = GatewayMessage::from_raw_json_message(received).unwrap();
            }
            GatewayTransportCompression::ZLibStream => {
                zlib_buffer = Some(Vec::new());
                let mut inflate = Decompress::new(true);

                message =
                    GatewayMessage::from_zlib_stream_json_message(received, &mut inflate).unwrap();

                zlib_inflate = Some(inflate);
            }
        }

        let gateway_payload: types::GatewayReceivePayload =
            serde_json::from_str(&message.0).unwrap();

        if gateway_payload.op_code != (Opcode::Hello as u8) {
            warn!(
                "GW: Received a non-hello opcode ({}) on gateway init",
                gateway_payload.op_code
            );
            return Err(GatewayError::NonHelloOnInitiate {
                opcode: gateway_payload.op_code,
            });
        }

        debug!("GW: Received Hello");

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
            url: url.clone(),
            options,
            zlib_inflate,
            zlib_buffer,
        };

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        #[cfg(not(target_arch = "wasm32"))]
        task::spawn(async move {
            gateway.gateway_listen_task_tungstenite().await;
        });
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            gateway.gateway_listen_task_wasm().await;
        });

        Ok(GatewayHandle {
            url: url.clone(),
            events: shared_events,
            websocket_send: shared_websocket_send.clone(),
            kill_send: kill_send.clone(),
            store,
        })
    }

    /// The main gateway listener task for a tungstenite based gateway;
    #[cfg(not(target_arch = "wasm32"))]
    async fn gateway_listen_task_tungstenite(&mut self) {
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

            // Note: The tungstenite backend handles close codes as messages, while the ws_stream_wasm one handles them differently.
            //
            // Hence why wasm receives straight RawGatewayMessages, and tungstenite receives
            // GatewayCommunications.
            if let Some(Ok(message)) = msg {
                let communication: GatewayCommunication = message.into();

                match communication {
                    GatewayCommunication::Message(raw_message) => {
                        self.handle_raw_message(raw_message).await
                    }
                    GatewayCommunication::Error(close_code) => {
                        self.handle_close_code(close_code).await
                    }
                }

                continue;
            }

            // We couldn't receive the next message or it was an error, something is wrong with the websocket, close
            warn!("GW: Websocket is broken, stopping gateway");
            break;
        }
    }

    /// The main gateway listener task for a wasm based gateway;
    ///
    /// Wasm handles close codes and events differently, and so we must change the listener logic a
    /// bit
    #[cfg(target_arch = "wasm32")]
    async fn gateway_listen_task_wasm(&mut self) {
        // Initiate the close event listener
        let mut close_events = self
            .websocket_receive
            .1
            .observe(pharos::Filter::Pointer(ws_stream_wasm::WsEvent::is_closed).into())
            .await
            .unwrap();

        loop {
            let msg;

            tokio::select! {
                 Ok(_) = self.kill_receive.recv() => {
                      log::trace!("GW: Closing listener task");
                      break;
                 }
                 message = self.websocket_receive.0.next() => {
                      msg = message;
                 }
                 maybe_event = close_events.next() => {
                      if let Some(event) = maybe_event {
                              match event {
                                    ws_stream_wasm::WsEvent::Closed(closed_event) => {
                                        let close_code = CloseCode::try_from(closed_event.code).unwrap_or(CloseCode::UnknownError);
                                        self.handle_close_code(close_code).await;
                                        break;
                                    }
                                    _ => unreachable!() // Should be impossible, we filtered close events
                              }
                      }
                      continue;
                }
            }

            // Note: The tungstenite backend handles close codes as messages, while the ws_stream_wasm one handles them as a seperate receiver.
            //
            // Hence why wasm receives RawGatewayMessages, and tungstenite receives
            // GatewayCommunications.
            if let Some(message) = msg {
                self.handle_raw_message(message.into()).await;
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

    /// Handles receiving a [CloseCode].
    ///
    /// Closes the connection and publishes an error event.
    async fn handle_close_code(&mut self, code: CloseCode) {
        let error = GatewayError::from(code);

        warn!("GW: Received error {:?}, connection will close..", error);
        self.close().await;
        self.events.lock().await.error.publish(error).await;
    }

    /// Deserializes and updates a dispatched event, when we already know its type;
    /// (Called for every event in handle_message)
    #[allow(dead_code)] // TODO: Remove this allow annotation
    async fn handle_event<'a, T: WebSocketEvent + serde::Deserialize<'a>>(
        data: &'a str,
        event: &mut Publisher<T>,
    ) -> Result<(), serde_json::Error> {
        let data_deserialize_result: Result<T, serde_json::Error> = serde_json::from_str(data);

        if data_deserialize_result.is_err() {
            return Err(data_deserialize_result.err().unwrap());
        }

        event.publish(data_deserialize_result.unwrap()).await;
        Ok(())
    }

    /// Takes a [RawGatewayMessage], converts it to [GatewayMessage] based
    /// of connection options and calls [Self::handle_message]
    async fn handle_raw_message(&mut self, raw_message: RawGatewayMessage) {
        let message;

        match self.options.transport_compression {
            GatewayTransportCompression::None => {
                message = GatewayMessage::from_raw_json_message(raw_message).unwrap()
            }
            GatewayTransportCompression::ZLibStream => {
                let message_bytes = raw_message.into_bytes();

                let can_decompress = message_bytes.len() > 4
                    && message_bytes[message_bytes.len() - 4..] == ZLIB_SUFFIX;

                let zlib_buffer = self.zlib_buffer.as_mut().unwrap();
                zlib_buffer.extend(message_bytes.clone());

                if !can_decompress {
                    return;
                }

                let zlib_buffer = self.zlib_buffer.as_ref().unwrap();
                let inflate = self.zlib_inflate.as_mut().unwrap();

                message =
                    GatewayMessage::from_zlib_stream_json_bytes(zlib_buffer, inflate).unwrap();
                self.zlib_buffer = Some(Vec::new());
            }
        };

        self.handle_message(message).await;
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    async fn handle_message(&mut self, msg: GatewayMessage) {
        if msg.0.is_empty() {
            return;
        }

        let Ok(gateway_payload) = msg.payload() else {
            warn!(
                "GW: Message unrecognised: {:?}, please open an issue on the chorus github",
                msg.0
            );
            return;
        };

        let op_code_res = Opcode::try_from(gateway_payload.op_code);

        if op_code_res.is_err() {
            warn!("Received unrecognized gateway op code ({})! Please open an issue on the chorus github so we can implement it", gateway_payload.op_code);
            trace!("Event data: {:?}", gateway_payload);
            return;
        }

        let op_code = op_code_res.unwrap();

        match op_code {
            // An event was dispatched, we need to look at the gateway event name t
            Opcode::Dispatch => {
                let Some(event_name) = gateway_payload.event_name else {
                    warn!("GW: Received dispatch without event_name");
                    return;
                };

                trace!("GW: Received {event_name}");

                macro_rules! handle {
                    ($($name:literal => $($path:ident).+ $( $message_type:ty: $update_type:ty)?),*) => {
                        match event_name.as_str() {
                            $($name => {
                                let event = &mut self.events.lock().await.$($path).+;
                                let json = gateway_payload.event_data.unwrap().get();
                                match serde_json::from_str(json) {
                                    Err(err) => {
                                        warn!("Failed to parse gateway event {event_name} ({err})");
                                        trace!("Event data: {json}");
                                    },
                                    Ok(message) => {
                                        $(
                                            let mut message: $message_type = message;
                                            let store = self.store.lock().await;
                                            let id = if message.id().is_some() {
                                                message.id().unwrap()
                                            } else {
                                                event.publish(message).await;
                                                return;
                                            };
                                            if let Some(to_update) = store.get(&id) {
                                                let object = to_update.clone();
                                                let mut inner_object = object.write().unwrap();
                                                if let Some(downcasted) = inner_object.downcast_mut::<$update_type>() {
                                                    message.set_source_url(self.url.clone());
                                                    message.update(downcasted);
                                                } else {
                                                    warn!("Received {} for {}, but it has been observed to be a different type!", $name, id)
                                                }
                                            }
                                        )?
                                        event.publish(message).await;
                                    }
                                }
                            },)*
                            "SESSIONS_REPLACE" => {
                                let json = gateway_payload.event_data.unwrap().get();
                                let result: Result<Vec<types::Session>, serde_json::Error> = serde_json::from_str(json);
                                match result {
                                    Err(err) => {
                                        warn!("Failed to parse gateway event {event_name} ({err})");
                                        trace!("Event data: {json}");
                                        return;
                                    }
                                    Ok(sessions) => {
                                        self.events.lock().await.session.replace.publish(
                                            types::SessionsReplace {sessions}
                                        ).await;
                                    }
                                }
                            },
                            _ => {
                                warn!("Received unrecognized gateway event ({event_name})! Please open an issue on the chorus github so we can implement it");
                                trace!("Event data: {}", gateway_payload.event_data.unwrap().get());
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
                    "AUTHENTICATOR_CREATE" => mfa.authenticator_create, // TODO
                    "AUTHENTICATOR_UPDATE" => mfa.authenticator_update, // TODO
                    "AUTHENTICATOR_DELETE" => mfa.authenticator_delete, // TODO
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
                    "LAST_MESSAGES" => message.last_messages,
                    "MESSAGE_CREATE" => message.create,
                    "MESSAGE_UPDATE" => message.update, // TODO
                    "MESSAGE_DELETE" => message.delete,
                    "MESSAGE_DELETE_BULK" => message.delete_bulk,
                    "MESSAGE_REACTION_ADD" => message.reaction_add, // TODO
                    "MESSAGE_REACTION_REMOVE" => message.reaction_remove, // TODO
                    "MESSAGE_REACTION_REMOVE_ALL" => message.reaction_remove_all, // TODO
                    "MESSAGE_REACTION_REMOVE_EMOJI" => message.reaction_remove_emoji, // TODO
                    "RECENT_MENTION_DELETE" => message.recent_mention_delete,
                    "MESSAGE_ACK" => message.ack,
                    "PRESENCE_UPDATE" => user.presence_update, // TODO
                          "RESUMED" => session.resumed,
                    "RELATIONSHIP_ADD" => relationship.add,
                    "RELATIONSHIP_REMOVE" => relationship.remove,
                    "STAGE_INSTANCE_CREATE" => stage_instance.create,
                    "STAGE_INSTANCE_UPDATE" => stage_instance.update, // TODO
                    "STAGE_INSTANCE_DELETE" => stage_instance.delete,
                    "TYPING_START" => user.typing_start,
                    "USER_UPDATE" => user.update, // TODO
                    "USER_CONNECTIONS_UPDATE" => user.connections_update, // TODO
                    "USER_NOTE_UPDATE" => user.note_update,
                    "USER_GUILD_SETTINGS_UPDATE" => user.guild_settings_update,
                    "VOICE_STATE_UPDATE" => voice.state_update, // TODO
                    "VOICE_SERVER_UPDATE" => voice.server_update,
                    "WEBHOOKS_UPDATE" => webhooks.update
                );
            }
            // We received a heartbeat from the server
            // "Discord may send the app a Heartbeat (opcode 1) event, in which case the app should send a Heartbeat event immediately."
            Opcode::Heartbeat => {
                trace!("GW: Received Heartbeat // Heartbeat Request");

                // Tell the heartbeat handler it should send a heartbeat right away
                let heartbeat_communication = HeartbeatThreadCommunication {
                    sequence_number: gateway_payload.sequence_number,
                    op_code: Some(Opcode::Heartbeat),
                };

                self.heartbeat_handler
                    .send
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
            Opcode::HeartbeatAck => {
                trace!("GW: Received Heartbeat ACK");

                // Tell the heartbeat handler we received an ack
                let heartbeat_communication = HeartbeatThreadCommunication {
                    sequence_number: gateway_payload.sequence_number,
                    op_code: Some(Opcode::HeartbeatAck),
                };

                self.heartbeat_handler
                    .send
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
            Opcode::Reconnect => {
                trace!("GW: Received Reconnect");

                let reconnect = GatewayReconnect {};

                self.events
                    .lock()
                    .await
                    .session
                    .reconnect
                    .publish(reconnect)
                    .await;
            }
            Opcode::InvalidSession => {
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
                    .publish(invalid_session)
                    .await;
            }
            // Starts our heartbeat
            // We should have already handled this
            Opcode::Hello => {
                warn!("Received hello when it was unexpected");
            }
            _ => {
                warn!(
                    "Received unexpected opcode ({}) for current state. This might be due to a faulty server implementation, but you can open an issue on the chorus github anyway",
                    gateway_payload.op_code
                );
            }
        }

        // If we we received a sequence number we should let the heartbeat thread know
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
