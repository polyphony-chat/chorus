use crate::errors::ObserverError;
use crate::gateway::events::Events;
use crate::types;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use native_tls::TlsConnector;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
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

#[derive(Debug)]
/**
Represents a handle to a Gateway connection. A Gateway connection will create observable
[`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
implemented [Types] with the trait [`WebSocketEvent`]
Using this handle you can also send Gateway Events directly.
*/
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

    /// Sends an identify event to the gateway
    pub async fn send_identify(&self, to_send: types::GatewayIdentifyPayload) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Identify..");

        self.send_json_event(GATEWAY_IDENTIFY, to_send_value).await;
    }

    /// Sends a resume event to the gateway
    pub async fn send_resume(&self, to_send: types::GatewayResume) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Resume..");

        self.send_json_event(GATEWAY_RESUME, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    pub async fn send_update_presence(&self, to_send: types::PresenceUpdate) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Presence Update..");

        self.send_json_event(GATEWAY_UPDATE_PRESENCE, to_send_value)
            .await;
    }

    /// Sends a request guild members to the server
    pub async fn send_request_guild_members(&self, to_send: types::GatewayRequestGuildMembers) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Request Guild Members..");

        self.send_json_event(GATEWAY_REQUEST_GUILD_MEMBERS, to_send_value)
            .await;
    }

    /// Sends an update voice state to the server
    pub async fn send_update_voice_state(&self, to_send: types::UpdateVoiceState) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Update Voice State..");

        self.send_json_event(GATEWAY_UPDATE_VOICE_STATE, to_send_value)
            .await;
    }

    /// Sends a call sync to the server
    pub async fn send_call_sync(&self, to_send: types::CallSync) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Call Sync..");

        self.send_json_event(GATEWAY_CALL_SYNC, to_send_value).await;
    }

    /// Sends a Lazy Request
    pub async fn send_lazy_request(&self, to_send: types::LazyRequest) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Lazy Request..");

        self.send_json_event(GATEWAY_LAZY_REQUEST, to_send_value)
            .await;
    }
}

pub struct Gateway {
    pub events: Arc<Mutex<Events>>,
    heartbeat_handler: Option<HeartbeatHandler>,
    pub websocket_send: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
}

impl Gateway {
    pub async fn new(
        websocket_url: String,
    ) -> Result<GatewayHandle, tokio_tungstenite::tungstenite::Error> {
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
            Err(e) => return Err(e),
        };

        let (gateway_send, mut gateway_receive) = websocket_stream.split();

        let shared_gateway_send = Arc::new(Mutex::new(gateway_send));

        let mut gateway = Gateway {
            events: Arc::new(Mutex::new(Events::default())),
            heartbeat_handler: None,
            websocket_send: shared_gateway_send.clone(),
        };

        let shared_events = gateway.events.clone();

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        let msg = gateway_receive.next().await.unwrap().unwrap();
        let gateway_payload: types::GatewayReceivePayload =
            serde_json::from_str(msg.to_text().unwrap()).unwrap();

        if gateway_payload.op_code != GATEWAY_HELLO {
            println!("Received non hello on gateway init, what is happening?");
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                tokio_tungstenite::tungstenite::error::ProtocolError::InvalidOpcode(
                    gateway_payload.op_code,
                ),
            ));
        }

        println!("GW: Received Hello");

        let gateway_hello: types::HelloData =
            serde_json::from_str(gateway_payload.event_data.unwrap().get()).unwrap();
        gateway.heartbeat_handler = Some(HeartbeatHandler::new(
            gateway_hello.heartbeat_interval,
            shared_gateway_send.clone(),
        ));

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        let handle: JoinHandle<()> = task::spawn(async move {
            loop {
                let msg = gateway_receive.next().await;
                if msg.as_ref().is_some() {
                    let msg_unwrapped = msg.unwrap().unwrap();
                    gateway.handle_event(msg_unwrapped).await;
                };
            }
        });

        return Ok(GatewayHandle {
            url: websocket_url.clone(),
            events: shared_events,
            websocket_send: shared_gateway_send.clone(),
            handle,
        });
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    pub async fn handle_event(&mut self, msg: tokio_tungstenite::tungstenite::Message) {
        if msg.to_string() == String::new() {
            return;
        }

        let gateway_payload: types::GatewayReceivePayload =
            serde_json::from_str(msg.to_text().unwrap()).unwrap();

        // See https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
        match gateway_payload.op_code {
            // An event was dispatched, we need to look at the gateway event name t
            GATEWAY_DISPATCH => {
                let gateway_payload_t = gateway_payload.clone().event_name.unwrap();

                println!("GW: Received {}..", gateway_payload_t);

                //println!("Event data dump: {}", gateway_payload.d.clone().unwrap().get());

                // See https://discord.com/developers/docs/topics/gateway-events#receive-events
                // "Some" of these are undocumented
                match gateway_payload_t.as_str() {
                    "READY" => {
                        let new_data: types::GatewayReady =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .session
                            .ready
                            .update_data(new_data)
                            .await;
                    }
                    "READY_SUPPLEMENTAL" => {
                        let new_data: types::GatewayReadySupplemental =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .session
                            .ready_supplemental
                            .update_data(new_data)
                            .await;
                    }
                    "RESUMED" => {}
                    "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {
                        let new_data: types::ApplicationCommandPermissionsUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .application
                            .command_permissions_update
                            .update_data(new_data)
                            .await;
                    }
                    "AUTO_MODERATION_RULE_CREATE" => {
                        let new_data: types::AutoModerationRuleCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .auto_moderation
                            .rule_create
                            .update_data(new_data)
                            .await;
                    }
                    "AUTO_MODERATION_RULE_UPDATE" => {
                        let new_data: types::AutoModerationRuleUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .auto_moderation
                            .rule_update
                            .update_data(new_data)
                            .await;
                    }
                    "AUTO_MODERATION_RULE_DELETE" => {
                        let new_data: types::AutoModerationRuleDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .auto_moderation
                            .rule_delete
                            .update_data(new_data)
                            .await;
                    }
                    "AUTO_MODERATION_ACTION_EXECUTION" => {
                        let new_data: types::AutoModerationActionExecution =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .auto_moderation
                            .action_execution
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_CREATE" => {
                        let new_data: types::ChannelCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .channel
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_UPDATE" => {
                        let new_data: types::ChannelUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .channel
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_UNREAD_UPDATE" => {
                        let new_data: types::ChannelUnreadUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .channel
                            .unread_update
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_DELETE" => {
                        let new_data: types::ChannelDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .channel
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_PINS_UPDATE" => {
                        let new_data: types::ChannelPinsUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .channel
                            .pins_update
                            .update_data(new_data)
                            .await;
                    }
                    "CALL_CREATE" => {
                        let new_data: types::CallCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .call
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "CALL_UPDATE" => {
                        let new_data: types::CallUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .call
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "CALL_DELETE" => {
                        let new_data: types::CallDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .call
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_CREATE" => {
                        let new_data: types::ThreadCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_UPDATE" => {
                        let new_data: types::ThreadUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_DELETE" => {
                        let new_data: types::ThreadDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_LIST_SYNC" => {
                        let new_data: types::ThreadListSync =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .list_sync
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_MEMBER_UPDATE" => {
                        let new_data: types::ThreadMemberUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .member_update
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_MEMBERS_UPDATE" => {
                        let new_data: types::ThreadMembersUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .members_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_CREATE" => {
                        let new_data: types::GuildCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_UPDATE" => {
                        let new_data: types::GuildUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_DELETE" => {
                        let new_data: types::GuildDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
                        let new_data: types::GuildAuditLogEntryCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .audit_log_entry_create
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_BAN_ADD" => {
                        let new_data: types::GuildBanAdd =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .ban_add
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_BAN_REMOVE" => {
                        let new_data: types::GuildBanRemove =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .ban_remove
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_EMOJIS_UPDATE" => {
                        let new_data: types::GuildEmojisUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .emojis_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_STICKERS_UPDATE" => {
                        let new_data: types::GuildStickersUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .stickers_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_INTEGRATIONS_UPDATE" => {
                        let new_data: types::GuildIntegrationsUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .integrations_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_MEMBER_ADD" => {
                        let new_data: types::GuildMemberAdd =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .member_add
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_MEMBER_REMOVE" => {
                        let new_data: types::GuildMemberRemove =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .member_remove
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_MEMBER_UPDATE" => {
                        let new_data: types::GuildMemberUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .member_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_MEMBERS_CHUNK" => {
                        let new_data: types::GuildMembersChunk =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .members_chunk
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_ROLE_CREATE" => {
                        let new_data: types::GuildRoleCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_create
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_ROLE_UPDATE" => {
                        let new_data: types::GuildRoleUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_ROLE_DELETE" => {
                        let new_data: types::GuildRoleDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_delete
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_SCHEDULED_EVENT_CREATE" => {
                        let new_data: types::GuildScheduledEventCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_scheduled_event_create
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_SCHEDULED_EVENT_UPDATE" => {
                        let new_data: types::GuildScheduledEventUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_scheduled_event_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_SCHEDULED_EVENT_DELETE" => {
                        let new_data: types::GuildScheduledEventDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_scheduled_event_delete
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_SCHEDULED_EVENT_USER_ADD" => {
                        let new_data: types::GuildScheduledEventUserAdd =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_scheduled_event_user_add
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
                        let new_data: types::GuildScheduledEventUserRemove =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .role_scheduled_event_user_remove
                            .update_data(new_data)
                            .await;
                    }
                    "PASSIVE_UPDATE_V1" => {
                        let new_data: types::PassiveUpdateV1 =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .passive_update_v1
                            .update_data(new_data)
                            .await;
                    }
                    "INTEGRATION_CREATE" => {
                        let new_data: types::IntegrationCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .integration
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "INTEGRATION_UPDATE" => {
                        let new_data: types::IntegrationUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .integration
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "INTEGRATION_DELETE" => {
                        let new_data: types::IntegrationDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .integration
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "INTERACTION_CREATE" => {
                        let new_data: types::InteractionCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .interaction
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "INVITE_CREATE" => {
                        let new_data: types::InviteCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .invite
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "INVITE_DELETE" => {
                        let new_data: types::InviteDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .invite
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_CREATE" => {
                        let new_data: types::MessageCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_UPDATE" => {
                        let new_data: types::MessageUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_DELETE" => {
                        let new_data: types::MessageDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_DELETE_BULK" => {
                        let new_data: types::MessageDeleteBulk =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .delete_bulk
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_ADD" => {
                        let new_data: types::MessageReactionAdd =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_add
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_REMOVE" => {
                        let new_data: types::MessageReactionRemove =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_remove
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_REMOVE_ALL" => {
                        let new_data: types::MessageReactionRemoveAll =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_remove_all
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_REMOVE_EMOJI" => {
                        let new_data: types::MessageReactionRemoveEmoji =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_remove_emoji
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_ACK" => {
                        let new_data: types::MessageACK =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .ack
                            .update_data(new_data)
                            .await;
                    }
                    "PRESENCE_UPDATE" => {
                        let new_data: types::PresenceUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .user
                            .presence_update
                            .update_data(new_data)
                            .await;
                    }
                    "RELATIONSHIP_ADD" => {
                        let new_data: types::RelationshipAdd =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .relationship
                            .add
                            .update_data(new_data)
                            .await;
                    }
                    "RELATIONSHIP_REMOVE" => {
                        let new_data: types::RelationshipRemove =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .relationship
                            .remove
                            .update_data(new_data)
                            .await;
                    }
                    "STAGE_INSTANCE_CREATE" => {
                        let new_data: types::StageInstanceCreate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .stage_instance
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "STAGE_INSTANCE_UPDATE" => {
                        let new_data: types::StageInstanceUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .stage_instance
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "STAGE_INSTANCE_DELETE" => {
                        let new_data: types::StageInstanceDelete =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .stage_instance
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "SESSIONS_REPLACE" => {
                        let sessions: Vec<types::Session> =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        let new_data = types::SessionsReplace { sessions };
                        self.events
                            .lock()
                            .await
                            .session
                            .replace
                            .update_data(new_data)
                            .await;
                    }
                    "TYPING_START" => {
                        let new_data: types::TypingStartEvent =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .user
                            .typing_start_event
                            .update_data(new_data)
                            .await;
                    }
                    "USER_UPDATE" => {
                        let new_data: types::UserUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .user
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "USER_GUILD_SETTINGS_UPDATE" => {
                        let new_data: types::UserGuildSettingsUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .user
                            .guild_settings_update
                            .update_data(new_data)
                            .await;
                    }
                    "VOICE_STATE_UPDATE" => {
                        let new_data: types::VoiceStateUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .voice
                            .state_update
                            .update_data(new_data)
                            .await;
                    }
                    "VOICE_SERVER_UPDATE" => {
                        let new_data: types::VoiceServerUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .voice
                            .server_update
                            .update_data(new_data)
                            .await;
                    }
                    "WEBHOOKS_UPDATE" => {
                        let new_data: types::WebhooksUpdate =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get())
                                .unwrap();
                        self.events
                            .lock()
                            .await
                            .webhooks
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    _ => {
                        println!("Received unrecognized gateway event ({})! Please open an issue on the chorus github so we can implement it", &gateway_payload_t);
                    }
                }
            }
            // We received a heartbeat from the server
            GATEWAY_HEARTBEAT => {}
            GATEWAY_RECONNECT => {
                todo!()
            }
            GATEWAY_INVALID_SESSION => {
                todo!()
            }
            // Starts our heartbeat
            // We should have already handled this in gateway init
            GATEWAY_HELLO => {
                panic!("Received hello when it was unexpected");
            }
            GATEWAY_HEARTBEAT_ACK => {
                println!("GW: Received Heartbeat ACK");
            }
            GATEWAY_IDENTIFY
            | GATEWAY_UPDATE_PRESENCE
            | GATEWAY_UPDATE_VOICE_STATE
            | GATEWAY_RESUME
            | GATEWAY_REQUEST_GUILD_MEMBERS
            | GATEWAY_CALL_SYNC
            | GATEWAY_LAZY_REQUEST => {
                panic!(
                    "Received gateway op code that's meant to be sent, not received ({})",
                    gateway_payload.op_code
                )
            }
            _ => {
                println!("Received unrecognized gateway op code ({})! Please open an issue on the chorus github so we can implement it", gateway_payload.op_code);
            }
        }

        // If we have an active heartbeat thread and we received a seq number we should let it know
        if gateway_payload.sequence_number.is_some() {
            if self.heartbeat_handler.is_some() {
                let heartbeat_communication = HeartbeatThreadCommunication {
                    op_code: gateway_payload.op_code,
                    sequence_number: gateway_payload.sequence_number.unwrap(),
                };

                self.heartbeat_handler
                    .as_mut()
                    .unwrap()
                    .send
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
        }
    }
}

/**
Handles sending heartbeats to the gateway in another thread
*/
struct HeartbeatHandler {
    /// The heartbeat interval in milliseconds
    pub heartbeat_interval: u128,
    /// The send channel for the heartbeat thread
    pub send: Sender<HeartbeatThreadCommunication>,
    /// The handle of the thread
    handle: JoinHandle<()>,
}

impl HeartbeatHandler {
    pub fn new(
        heartbeat_interval: u128,
        websocket_tx: Arc<
            Mutex<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tokio_tungstenite::tungstenite::Message,
                >,
            >,
        >,
    ) -> HeartbeatHandler {
        let (send, mut receive) = mpsc::channel(32);

        let handle: JoinHandle<()> = task::spawn(async move {
            let mut last_heartbeat_timestamp: Instant = time::Instant::now();
            let mut last_seq_number: Option<u64> = None;

            loop {
                // If we received a seq number update, use that as the last seq number
                let received_communication: Result<HeartbeatThreadCommunication, TryRecvError> =
                    receive.try_recv();
                if received_communication.is_ok() {
                    last_seq_number = Some(received_communication.unwrap().sequence_number);
                }

                let should_send =
                    last_heartbeat_timestamp.elapsed().as_millis() >= heartbeat_interval;

                if should_send {
                    println!("GW: Sending Heartbeat..");

                    let heartbeat = types::GatewayHeartbeat {
                        op: GATEWAY_HEARTBEAT,
                        d: last_seq_number,
                    };

                    let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                    let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                    websocket_tx.lock().await.send(msg).await.unwrap();

                    last_heartbeat_timestamp = time::Instant::now();
                }
            }
        });

        Self {
            heartbeat_interval,
            send,
            handle,
        }
    }
}

/**
Used to communicate with the main thread.
Either signifies a sequence number update or a received heartbeat ack
*/
#[derive(Clone, Copy, Debug)]
struct HeartbeatThreadCommunication {
    /// The opcode for the communication we received
    op_code: u8,
    /// The sequence number we got from discord
    sequence_number: u64,
}

/**
Trait which defines the behavior of an Observer. An Observer is an object which is subscribed to
an Observable. The Observer is notified when the Observable's data changes.
In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
 */
pub trait Observer<T: types::WebSocketEvent>: std::fmt::Debug {
    fn update(&self, data: &T);
}

/** GatewayEvent is a wrapper around a WebSocketEvent. It is used to notify the observers of a
change in the WebSocketEvent. GatewayEvents are observable.
*/

#[derive(Default, Debug)]
pub struct GatewayEvent<T: types::WebSocketEvent> {
    observers: Vec<Arc<Mutex<dyn Observer<T> + Sync + Send>>>,
    pub event_data: T,
    pub is_observed: bool,
}

impl<T: types::WebSocketEvent> GatewayEvent<T> {
    fn new(event_data: T) -> Self {
        Self {
            is_observed: false,
            observers: Vec::new(),
            event_data,
        }
    }

    /**
    Returns true if the GatewayEvent is observed by at least one Observer.
    */
    pub fn is_observed(&self) -> bool {
        self.is_observed
    }

    /**
    Subscribes an Observer to the GatewayEvent. Returns an error if the GatewayEvent is already
    observed.
    # Errors
    Returns an error if the GatewayEvent is already observed.
    Error type: [`ObserverError::AlreadySubscribedError`]
    */
    pub fn subscribe(
        &mut self,
        observable: Arc<Mutex<dyn Observer<T> + Sync + Send>>,
    ) -> Result<(), ObserverError> {
        if self.is_observed {
            return Err(ObserverError::AlreadySubscribedError);
        }
        self.is_observed = true;
        self.observers.push(observable);
        Ok(())
    }

    /**
    Unsubscribes an Observer from the GatewayEvent.
    */
    pub fn unsubscribe(&mut self, observable: Arc<Mutex<dyn Observer<T> + Sync + Send>>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        // The usage of the debug format to compare the generic T of observers is quite stupid, but the only thing to compare between them is T and if T == T they are the same
        // anddd there is no way to do that without using format
        self.observers
            .retain(|obs| !(format!("{:?}", obs) == format!("{:?}", &observable)));
        self.is_observed = !self.observers.is_empty();
    }

    /**
    Updates the GatewayEvent's data and notifies the observers.
    */
    async fn update_data(&mut self, new_event_data: T) {
        self.event_data = new_event_data;
        self.notify().await;
    }

    /**
    Notifies the observers of the GatewayEvent.
    */
    async fn notify(&self) {
        for observer in &self.observers {
            observer.lock().await.update(&self.event_data);
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
        pub typing_start_event: GatewayEvent<types::TypingStartEvent>,
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

    #[derive(Debug)]
    struct Consumer;
    impl Observer<types::GatewayResume> for Consumer {
        fn update(&self, data: &types::GatewayResume) {
            println!("{}", data.token)
        }
    }

    #[tokio::test]
    async fn test_observer_behavior() {
        let mut event = GatewayEvent::new(types::GatewayResume {
            token: "start".to_string(),
            session_id: "start".to_string(),
            seq: "start".to_string(),
        });

        let new_data = types::GatewayResume {
            token: "token_3276ha37am3".to_string(),
            session_id: "89346671230".to_string(),
            seq: "3".to_string(),
        };

        let consumer = Consumer;
        let arc_mut_consumer = Arc::new(Mutex::new(consumer));

        event.subscribe(arc_mut_consumer.clone());

        event.notify().await;

        event.update_data(new_data).await;

        let second_consumer = Consumer;
        let arc_mut_second_consumer = Arc::new(Mutex::new(second_consumer));

        match event.subscribe(arc_mut_second_consumer.clone()) {
            None => assert!(false),
            Some(err) => println!("You cannot subscribe twice: {}", err),
        }

        event.unsubscribe(arc_mut_consumer.clone());

        match event.subscribe(arc_mut_second_consumer.clone()) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_gateway_establish() {
        let _gateway = Gateway::new("ws://localhost:3001/".to_string())
            .await
            .unwrap();
    }
}
