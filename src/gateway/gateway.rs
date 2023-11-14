use self::event::Events;
use super::*;
use crate::types::{
    self, AutoModerationRule, AutoModerationRuleUpdate, Channel, ChannelCreate, ChannelDelete,
    ChannelUpdate, Guild, GuildRoleCreate, GuildRoleUpdate, JsonField, RoleObject, SourceUrlField,
    ThreadUpdate, UpdateMessage, WebSocketEvent,
};

pub type GatewayStore = Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>;

#[derive(Debug)]
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
    store: GatewayStore,
    url: String,
}

#[async_trait]
impl
    GatewayCapable<
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        GatewayHandle,
    > for Gateway
{
    #[allow(clippy::new_ret_no_self)]
    async fn get_handle(websocket_url: String) -> Result<GatewayHandle, GatewayError> {
        let mut roots = rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs")
        {
            roots.add(&rustls::Certificate(cert.0)).unwrap();
        }
        let (websocket_stream, _) = match connect_async_tls_with_config(
            &websocket_url,
            None,
            false,
            Some(Connector::Rustls(
                rustls::ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(roots)
                    .with_no_client_auth()
                    .into(),
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
            url: websocket_url.clone(),
        };

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        task::spawn(async move {
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

    /// Closes the websocket connection and stops all tasks
    async fn close(&mut self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }

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

            self.events.lock().await.error.notify(error).await;

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

    fn get_events(&self) -> Arc<Mutex<Events>> {
        self.events.clone()
    }

    fn get_websocket_send(
        &self,
    ) -> Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>> {
        self.websocket_send.clone()
    }

    fn get_store(&self) -> GatewayStore {
        self.store.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }
}

impl Gateway {
    /// The main gateway listener task;
    ///
    /// Can only be stopped by closing the websocket, cannot be made to listen for kill
    pub async fn gateway_listen_task(&mut self) {
        loop {
            let msg = self.websocket_receive.next().await;

            // This if chain can be much better but if let is unstable on stable rust
            if let Some(Ok(message)) = msg {
                self.handle_message(GatewayMessage::from_tungstenite_message(message));
                continue;
            }

            // We couldn't receive the next message or it was an error, something is wrong with the websocket, close
            warn!("GW: Websocket is broken, stopping gateway");
            break;
        }
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
}

pub mod event {
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
        pub error: GatewayEvent<GatewayError>,
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
