use futures_util::StreamExt;

use self::event::Events;
use super::*;
use crate::types::{self, WebSocketEvent};

#[derive(Debug)]
pub struct DefaultGateway {
    events: Arc<Mutex<Events>>,
    heartbeat_handler: DefaultHeartbeatHandler,
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
        DefaultGatewayHandle,
        DefaultHeartbeatHandler,
    > for DefaultGateway
{
    fn get_heartbeat_handler(&self) -> &DefaultHeartbeatHandler {
        &self.heartbeat_handler
    }

    #[allow(clippy::new_ret_no_self)]
    async fn get_handle(websocket_url: String) -> Result<DefaultGatewayHandle, GatewayError> {
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

        let mut gateway = DefaultGateway {
            events: shared_events.clone(),
            heartbeat_handler: DefaultHeartbeatHandler::new(
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

        Ok(DefaultGatewayHandle {
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

impl DefaultGateway {
    /// The main gateway listener task;
    ///
    /// Can only be stopped by closing the websocket, cannot be made to listen for kill
    pub async fn gateway_listen_task(&mut self) {
        loop {
            let msg = self.websocket_receive.next().await;

            // This if chain can be much better but if let is unstable on stable rust
            if let Some(Ok(message)) = msg {
                let _ = self
                    .handle_message(GatewayMessage::from_tungstenite_message(message))
                    .await;
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
