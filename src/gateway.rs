use std::sync::Arc;
use crate::api::types::*;
use crate::api::WebSocketEvent;
use crate::errors::ObserverError;
use crate::gateway::events::Events;
use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::SplitSink;
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time;
use tokio::time::Instant;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{WebSocketStream, Connector, connect_async_tls_with_config};

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
    pub websocket_tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>>>,
}

impl GatewayHandle {
    /// Sends json to the gateway with an opcode
    async fn send_json_event(&self, op: u8, to_send: serde_json::Value) {

        let gateway_payload = GatewaySendPayload { op, d: Some(to_send), s: None };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();

        let message = tokio_tungstenite::tungstenite::Message::text(payload_json);

        self.websocket_tx.lock().await.send(message).await.unwrap();
    }

    /// Sends an identify event to the gateway
    pub async fn send_identify(&self, to_send: GatewayIdentifyPayload) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Identify..");

        self.send_json_event(2, to_send_value).await;
    }

    /// Sends a resume event to the gateway
    pub async fn send_resume(&self, to_send: GatewayResume) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Resume..");

        self.send_json_event(6, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    pub async fn send_update_presence(&self, to_send: PresenceUpdate) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Presence Update..");

        self.send_json_event(3, to_send_value).await;
    }

    /// Sends a request guild members to the server
    pub async fn send_request_guild_members(&self, to_send: GatewayRequestGuildMembers) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Request Guild Members..");

        self.send_json_event(8, to_send_value).await;
    }

    /// Sends an update voice state to the server
    pub async fn send_update_voice_state(&self, to_send: UpdateVoiceState) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Update Voice State..");

        self.send_json_event(4, to_send_value).await;
    }

    /// Sends a call sync to the server
    pub async fn send_call_sync(&self, to_send: CallSync) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Call Sync..");

        self.send_json_event(13, to_send_value).await;
    }

    /// Sends a Lazy Request
    pub async fn send_lazy_request(&self, to_send: LazyRequest) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Lazy Request..");

        self.send_json_event(14, to_send_value).await;
    }
}

pub struct Gateway {
    pub events: Arc<Mutex<Events>>,
    heartbeat_handler: Option<HeartbeatHandler>,
    pub websocket_tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>>>
}

impl Gateway {
    pub async fn new(
        websocket_url: String,
    ) -> Result<GatewayHandle, tokio_tungstenite::tungstenite::Error> {

        let (ws_stream, _) = match connect_async_tls_with_config(
            &websocket_url,
            None,
            false,
            Some(Connector::NativeTls(
                TlsConnector::builder().build().unwrap(),
            )),
        )
        .await
        {
            Ok(ws_stream) => ws_stream,
            Err(e) => return Err(e),
        };

        let (ws_tx, mut ws_rx) = ws_stream.split();

        let shared_tx = Arc::new(Mutex::new(ws_tx));

        let mut gateway = Gateway { events: Arc::new(Mutex::new(Events::default())), heartbeat_handler: None, websocket_tx: shared_tx.clone() };

        let shared_events = gateway.events.clone();

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        let msg = ws_rx.next().await.unwrap().unwrap();
        let gateway_payload: GatewayReceivePayload = serde_json::from_str(msg.to_text().unwrap()).unwrap();

        if gateway_payload.op != 10 {
            println!("Recieved non hello on gateway init, what is happening?");
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(tokio_tungstenite::tungstenite::error::ProtocolError::InvalidOpcode(gateway_payload.op)))
        }

        println!("GW: Received Hello");

        let gateway_hello: HelloData = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
        gateway.heartbeat_handler = Some(HeartbeatHandler::new(gateway_hello.heartbeat_interval, shared_tx.clone()));

        // Now we can continously check for messages in a different task, since we aren't going to receive another hello
        task::spawn(async move {
            loop {
                let msg = ws_rx.next().await;
                if msg.as_ref().is_some() {
                    let msg_unwrapped = msg.unwrap().unwrap();
                    gateway.handle_event(msg_unwrapped).await;
                };
            }
        });

        return Ok(GatewayHandle {
            url: websocket_url.clone(),
            events: shared_events,
            websocket_tx: shared_tx.clone(),
        });
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    pub async fn handle_event(&mut self, msg: tokio_tungstenite::tungstenite::Message) {
        
        if msg.to_string() == String::new() {
            return;
        }

        let gateway_payload: GatewayReceivePayload = serde_json::from_str(msg.to_text().unwrap()).unwrap();

        // See https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
        match gateway_payload.op {
            // Dispatch
            // An event was dispatched, we need to look at the gateway event name t
            0 => {
                let gateway_payload_t = gateway_payload.clone().t.unwrap();

                println!("GW: Received {}..", gateway_payload_t);

                println!("Event data dump: {}", gateway_payload.d.clone().unwrap().get());
                
                // See https://discord.com/developers/docs/topics/gateway-events#receive-events
                // "Some" of these are uncodumented
                match gateway_payload_t.as_str() {
                    "READY" => {
                        let new_data: GatewayReady = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.session.ready.update_data(new_data).await;
                    },
                    "READY_SUPPLEMENTAL" => {
                        let new_data: GatewayReadySupplemental = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.session.ready_supplimental.update_data(new_data).await;
                    }
                    "RESUMED" => {}
                    "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {}
                    "AUTO_MODERATION_RULE_CREATE" => {}
                    "AUTO_MODERATION_RULE_UPDATE" => {}
                    "AUTO_MODERATION_RULE_DELETE" => {}
                    "AUTO_MODERATION_ACTION_EXECUTION" => {}
                    "CHANNEL_CREATE" => {
                        let new_data: ChannelCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.channel.create.update_data(new_data).await;
                    }
                    "CHANNEL_UPDATE" => {
                        let new_data: ChannelUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.channel.update.update_data(new_data).await;
                    }
                    "CHANNEL_UNREAD_UPDATE" => {
                        let new_data: ChannelUnreadUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.channel.unread_update.update_data(new_data).await;
                    }
                    "CHANNEL_DELETE" => {
                        let new_data: ChannelDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.channel.delete.update_data(new_data).await;
                    }
                    "CHANNEL_PINS_UPDATE" => {
                        let new_data: ChannelPinsUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.channel.pins_update.update_data(new_data).await;
                    }
                    "CALL_CREATE" => {
                        let new_data: CallCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.call.create.update_data(new_data).await;
                    },
                    "CALL_UPDATE" => {
                        let new_data: CallUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.call.update.update_data(new_data).await;
                    }
                    "CALL_DELETE" => {
                        let new_data: CallDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.call.delete.update_data(new_data).await;
                    }
                    "THREAD_CREATE" => {
                        let new_data: ThreadCreate =  serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.thread.create.update_data(new_data).await;
                    }
                    "THREAD_UPDATE" => {
                        let new_data: ThreadUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.thread.update.update_data(new_data).await;
                    }
                    "THREAD_DELETE" => {
                        let new_data: ThreadDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.thread.delete.update_data(new_data).await;
                    }
                    "THREAD_LIST_SYNC" => {
                        let new_data: ThreadListSync = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.thread.list_sync.update_data(new_data).await;
                    }
                    "THREAD_MEMBER_UPDATE" => {
                        let new_data: ThreadMemberUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.thread.member_update.update_data(new_data).await;
                    }
                    "THREAD_MEMBERS_UPDATE" => {
                        let new_data: ThreadMembersUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.thread.members_update.update_data(new_data).await;
                    }
                    "GUILD_CREATE" => {
                        let new_data: GuildCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.create.update_data(new_data).await;
                    }
                    "GUILD_UPDATE" => {
                        let new_data: GuildUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.update.update_data(new_data).await;
                    }
                    "GUILD_DELETE" => {
                        let new_data: GuildDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.delete.update_data(new_data).await;
                    }
                    "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
                        let new_data: GuildAuditLogEntryCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.audit_log_entry_create.update_data(new_data).await;
                    }
                    "GUILD_BAN_ADD" => {
                        let new_data: GuildBanAdd = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.ban_add.update_data(new_data).await;
                    }
                    "GUILD_BAN_REMOVE" => {
                        let new_data: GuildBanRemove = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.ban_remove.update_data(new_data).await;
                    }
                    "GUILD_EMOJIS_UPDATE" => {
                        let new_data: GuildEmojisUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.emojis_update.update_data(new_data).await;
                    }
                    "GUILD_STICKERS_UPDATE" => {
                        let new_data: GuildStickersUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.stickers_update.update_data(new_data).await;
                    }
                    "GUILD_INTEGRATIONS_UPDATE" => {
                        let new_data: GuildIntegrationsUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.integrations_update.update_data(new_data).await;
                    }
                    "GUILD_MEMBER_ADD" => {
                        let new_data: GuildMemberAdd = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.member_add.update_data(new_data).await;
                    }
                    "GUILD_MEMBER_REMOVE" => {
                        let new_data: GuildMemberRemove = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.member_remove.update_data(new_data).await;
                    }
                    "GUILD_MEMBER_UPDATE" => {
                        let new_data: GuildMemberUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.member_update.update_data(new_data).await;
                    }
                    "GUILD_MEMBERS_CHUNK" => {
                        let new_data: GuildMembersChunk = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.members_chunk.update_data(new_data).await;
                    }
                    "GUILD_ROLE_CREATE" => {
                        let new_data: GuildRoleCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_create.update_data(new_data).await;
                    }
                    "GUILD_ROLE_UPDATE" => {
                        let new_data: GuildRoleUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_update.update_data(new_data).await;
                    }
                    "GUILD_ROLE_DELETE" => {
                        let new_data: GuildRoleDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_delete.update_data(new_data).await;
                    }
                    "GUILD_SCHEDULED_EVENT_CREATE" => {
                        let new_data: GuildScheduledEventCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_scheduled_event_create.update_data(new_data).await;
                    }
                    "GUILD_SCHEDULED_EVENT_UPDATE" => {
                        let new_data: GuildScheduledEventUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_scheduled_event_update.update_data(new_data).await;
                    }
                    "GUILD_SCHEDULED_EVENT_DELETE" => {
                        let new_data: GuildScheduledEventDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_scheduled_event_delete.update_data(new_data).await;
                    }
                    "GUILD_SCHEDULED_EVENT_USER_ADD" => {
                        let new_data: GuildScheduledEventUserAdd = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_scheduled_event_user_add.update_data(new_data).await;
                    }
                    "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
                        let new_data: GuildScheduledEventUserRemove = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.role_scheduled_event_user_remove.update_data(new_data).await;
                    }
                    "PASSIVE_UPDATE_V1" => {
                        let new_data: PassiveUpdateV1 = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.guild.passive_update_v1.update_data(new_data).await;
                    }
                    "INTEGRATION_CREATE" => {
                        let new_data: IntegrationCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.integration.create.update_data(new_data).await;
                    }
                    "INTEGRATION_UPDATE" => {
                        let new_data: IntegrationUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.integration.update.update_data(new_data).await;
                    }
                    "INTEGRATION_DELETE" => {
                        let new_data: IntegrationDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.integration.delete.update_data(new_data).await;
                    }
                    "INTERACTION_CREATE" => {}
                    "INVITE_CREATE" => {
                        let new_data: InviteCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.invite.create.update_data(new_data).await;
                    }
                    "INVITE_DELETE" => {
                        let new_data: InviteDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.invite.delete.update_data(new_data).await;
                    }
                    "MESSAGE_CREATE" => {
                        let new_data: MessageCreate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.create.update_data(new_data).await;
                    }
                    "MESSAGE_UPDATE" => {
                        let new_data: MessageUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.update.update_data(new_data).await;
                    }
                    "MESSAGE_DELETE" => {
                        let new_data: MessageDelete = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.delete.update_data(new_data).await;
                    }
                    "MESSAGE_DELETE_BULK" => {
                        let new_data: MessageDeleteBulk = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.delete_bulk.update_data(new_data).await;
                    }
                    "MESSAGE_REACTION_ADD" => {
                        let new_data: MessageReactionAdd = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.reaction_add.update_data(new_data).await;
                    }
                    "MESSAGE_REACTION_REMOVE" => {
                        let new_data: MessageReactionRemove = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.reaction_remove.update_data(new_data).await;
                    }
                    "MESSAGE_REACTION_REMOVE_ALL" => {
                        let new_data: MessageReactionRemoveAll = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.reaction_remove_all.update_data(new_data).await;
                    }
                    "MESSAGE_REACTION_REMOVE_EMOJI" => {
                        let new_data: MessageReactionRemoveEmoji= serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.reaction_remove_emoji.update_data(new_data).await;
                    },
                    "MESSAGE_ACK" => {
                        let new_data: MessageACK = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.message.ack.update_data(new_data).await;
                    }
                    "PRESENCE_UPDATE" => {
                        let new_data: PresenceUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.user.presence_update.update_data(new_data).await;
                    }
                    "STAGE_INSTANCE_CREATE" => {}
                    "STAGE_INSTANCE_UPDATE" => {}
                    "STAGE_INSTANCE_DELETE" => {}
                    "SESSIONS_REPLACE" => {
                        let sessions: Vec<Session> = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        let new_data = SessionsReplace {sessions};
                        self.events.lock().await.session.replace.update_data(new_data).await;
                    }
                    "TYPING_START" => {
                        let new_data: TypingStartEvent = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.user.typing_start_event.update_data(new_data).await;
                    }
                    "USER_UPDATE" => {
                        let new_data: UserUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.user.update.update_data(new_data).await;
                    }
                    "VOICE_STATE_UPDATE" => {
                        let new_data: VoiceStateUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.voice.state_update.update_data(new_data).await;
                    }
                    "VOICE_SERVER_UPDATE" => {
                        let new_data: VoiceServerUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.voice.server_update.update_data(new_data).await;
                    }
                    "WEBHOOKS_UPDATE" => {
                        let new_data: WebhooksUpdate = serde_json::from_str(gateway_payload.d.unwrap().get()).unwrap();
                        self.events.lock().await.webhooks.update.update_data(new_data).await;
                    }
                    _ => {
                        //panic!("Invalid gateway event ({})", &gateway_payload_t)
                        println!("New gateway event ({})", &gateway_payload_t);
                    }
                }
            }
            // Heartbeat
            // We received a heartbeat from the server
            1 => {}
            // Reconnect
            7 => {todo!()}
            // Invalid Session
            9 => {todo!()}
            // Hello
            // Starts our heartbeat
            // We should have already handled this in gateway init
            10 => {
                panic!("Recieved hello when it was unexpected");
            }
            // Heartbeat ACK
            11 => {
                println!("GW: Received Heartbeat ACK");
            }
            2 | 3 | 4 | 6 | 8 => {panic!("Received Gateway op code that's meant to be sent, not received ({})", gateway_payload.op)}
            _ => {println!("Received new Gateway op code ({})", gateway_payload.op);}
        }

        // If we have an active heartbeat thread and we received a seq number we should let it know
        if gateway_payload.s.is_some() {
            if self.heartbeat_handler.is_some() {

                let heartbeat_communication = HeartbeatThreadCommunication { op: gateway_payload.op, d: gateway_payload.s.unwrap() };

                self.heartbeat_handler.as_mut().unwrap().tx.send(heartbeat_communication).await.unwrap();
            }
        }
    }
}

/**
Handles sending heartbeats to the gateway in another thread
*/
struct HeartbeatHandler {
    /// The heartbeat interval in milliseconds
    heartbeat_interval: u128,
    tx: Sender<HeartbeatThreadCommunication>,
}

impl HeartbeatHandler {
    pub fn new(heartbeat_interval: u128, websocket_tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>>>) -> HeartbeatHandler {
        let (tx, mut rx) = mpsc::channel(32);

        task::spawn(async move {
            let mut last_heartbeat: Instant = time::Instant::now();
            let mut last_seq_number: Option<u64> = None;

            loop {

                // If we received a seq number update, use that as the last seq number
                let hb_communication: Result<HeartbeatThreadCommunication, TryRecvError> = rx.try_recv();
                if hb_communication.is_ok() {
                    last_seq_number = Some(hb_communication.unwrap().d);
                }

                if last_heartbeat.elapsed().as_millis() > heartbeat_interval {

                    println!("GW: Sending Heartbeat..");

                    let heartbeat = GatewayHeartbeat {
                        op: 1,
                        d: last_seq_number
                    };

                    let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                    let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                    websocket_tx.lock().await
                    .send(msg)
                    .await
                    .unwrap();

                    last_heartbeat = time::Instant::now();
                }
            }
        });

        Self { heartbeat_interval, tx }
    }
}

/**
Used to communicate with the main thread.
Either signifies a sequence number update or a received heartbeat ack
*/
#[derive(Clone, Copy, Debug)]
struct HeartbeatThreadCommunication {
    /// An opcode for the communication we received
    op: u8,
    /// The sequence number we got from discord
    d: u64
}

/**
Trait which defines the behaviour of an Observer. An Observer is an object which is subscribed to
an Observable. The Observer is notified when the Observable's data changes.
In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
 */
pub trait Observer<T: WebSocketEvent>: std::fmt::Debug {
    fn update(&self, data: &T);
}

/** GatewayEvent is a wrapper around a WebSocketEvent. It is used to notify the observers of a
change in the WebSocketEvent. GatewayEvents are observable.
*/

#[derive(Default, Debug)]
pub struct GatewayEvent<T: WebSocketEvent> {
    observers: Vec<Arc<Mutex<dyn Observer<T> + Sync + Send>>>,
    pub event_data: T,
    pub is_observed: bool,
}

impl<T: WebSocketEvent> GatewayEvent<T> {
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
    pub fn subscribe(&mut self, observable: Arc<Mutex<dyn Observer<T> + Sync + Send>>) -> Option<ObserverError> {
        if self.is_observed {
            return Some(ObserverError::AlreadySubscribedError);
        }
        self.is_observed = true;
        self.observers.push(observable);
        None
    }

    /**
    Unsubscribes an Observer from the GatewayEvent.
    */
    pub fn unsubscribe(&mut self, observable: Arc<Mutex<dyn Observer<T> + Sync + Send>>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        // The usage of the debug format to compare the generic T of observers is quite stupid, but the only thing to compare between them is T and if T == T they are the same
        // anddd there is no way to do that without using format
        self.observers.retain(|obs| !(format!("{:?}", obs) == format!("{:?}", &observable)));
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
        pub session: Session,
        pub message: Message,
        pub user: User,
        pub channel: Channel,
        pub thread: Thread,
        pub guild: Guild,
        pub invite: Invite,
        pub integration: Integration,
        pub call: Call,
        pub voice: Voice,
        pub webhooks: Webhooks,
        pub gateway_identify_payload: GatewayEvent<GatewayIdentifyPayload>,
        pub gateway_resume: GatewayEvent<GatewayResume>,
    }

    #[derive(Default, Debug)]
    pub struct Session {
        pub ready: GatewayEvent<GatewayReady>,
        pub ready_supplimental: GatewayEvent<GatewayReadySupplemental>,
        pub replace: GatewayEvent<SessionsReplace>
    }

    #[derive(Default, Debug)]
    pub struct Message {
        pub create: GatewayEvent<MessageCreate>,
        pub update: GatewayEvent<MessageUpdate>,
        pub delete: GatewayEvent<MessageDelete>,
        pub delete_bulk: GatewayEvent<MessageDeleteBulk>,
        pub reaction_add: GatewayEvent<MessageReactionAdd>,
        pub reaction_remove: GatewayEvent<MessageReactionRemove>,
        pub reaction_remove_all: GatewayEvent<MessageReactionRemoveAll>,
        pub reaction_remove_emoji: GatewayEvent<MessageReactionRemoveEmoji>,
        pub ack: GatewayEvent<MessageACK>
    }

    #[derive(Default, Debug)]
    pub struct User {
        pub update: GatewayEvent<UserUpdate>,
        pub presence_update: GatewayEvent<PresenceUpdate>,
        pub typing_start_event: GatewayEvent<TypingStartEvent>,
    }

    #[derive(Default, Debug)]
    pub struct Channel {
        pub create: GatewayEvent<ChannelCreate>,
        pub update: GatewayEvent<ChannelUpdate>,
        pub unread_update: GatewayEvent<ChannelUnreadUpdate>,
        pub delete: GatewayEvent<ChannelDelete>,
        pub pins_update: GatewayEvent<ChannelPinsUpdate>
    }

    #[derive(Default, Debug)]
    pub struct Thread {
        pub create: GatewayEvent<ThreadCreate>,
        pub update: GatewayEvent<ThreadUpdate>,
        pub delete: GatewayEvent<ThreadDelete>,
        pub list_sync: GatewayEvent<ThreadListSync>,
        pub member_update: GatewayEvent<ThreadMemberUpdate>,
        pub members_update: GatewayEvent<ThreadMembersUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct Guild {
        pub create: GatewayEvent<GuildCreate>,
        pub update: GatewayEvent<GuildUpdate>,
        pub delete: GatewayEvent<GuildDelete>,
        pub audit_log_entry_create: GatewayEvent<GuildAuditLogEntryCreate>,
        pub ban_add: GatewayEvent<GuildBanAdd>,
        pub ban_remove: GatewayEvent<GuildBanRemove>,
        pub emojis_update: GatewayEvent<GuildEmojisUpdate>,
        pub stickers_update: GatewayEvent<GuildStickersUpdate>,
        pub integrations_update: GatewayEvent<GuildIntegrationsUpdate>,
        pub member_add: GatewayEvent<GuildMemberAdd>,
        pub member_remove: GatewayEvent<GuildMemberRemove>,
        pub member_update: GatewayEvent<GuildMemberUpdate>,
        pub members_chunk: GatewayEvent<GuildMembersChunk>,
        pub role_create: GatewayEvent<GuildRoleCreate>,
        pub role_update: GatewayEvent<GuildRoleUpdate>,
        pub role_delete: GatewayEvent<GuildRoleDelete>,
        pub role_scheduled_event_create: GatewayEvent<GuildScheduledEventCreate>,
        pub role_scheduled_event_update: GatewayEvent<GuildScheduledEventUpdate>,
        pub role_scheduled_event_delete: GatewayEvent<GuildScheduledEventDelete>,
        pub role_scheduled_event_user_add: GatewayEvent<GuildScheduledEventUserAdd>,
        pub role_scheduled_event_user_remove: GatewayEvent<GuildScheduledEventUserRemove>,
        pub passive_update_v1: GatewayEvent<PassiveUpdateV1>,
    }

    #[derive(Default, Debug)]
    pub struct Invite {
        pub create: GatewayEvent<InviteCreate>,
        pub delete: GatewayEvent<InviteDelete>
    }

    #[derive(Default, Debug)]
    pub struct Integration {
        pub create: GatewayEvent<IntegrationCreate>,
        pub update: GatewayEvent<IntegrationUpdate>,
        pub delete: GatewayEvent<IntegrationDelete>
    }

    #[derive(Default, Debug)]
    pub struct Call {
        pub create: GatewayEvent<CallCreate>,
        pub update: GatewayEvent<CallUpdate>,
        pub delete: GatewayEvent<CallDelete>
    }

    #[derive(Default, Debug)]
    pub struct Voice {
        pub state_update: GatewayEvent<VoiceStateUpdate>,
        pub server_update: GatewayEvent<VoiceServerUpdate>
    }

    #[derive(Default, Debug)]
    pub struct Webhooks {
        pub update: GatewayEvent<WebhooksUpdate>,
    }
}

#[cfg(test)]
mod example {
    use super::*;
    use crate::api::types::GatewayResume;

    #[derive(Debug)]
    struct Consumer;
    impl Observer<GatewayResume> for Consumer {
        fn update(&self, data: &GatewayResume) {
            println!("{}", data.token)
        }
    }

    #[tokio::test]
    async fn test_observer_behaviour() {
        let mut event = GatewayEvent::new(GatewayResume {
            token: "start".to_string(),
            session_id: "start".to_string(),
            seq: "start".to_string(),
        });

        let new_data = GatewayResume {
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
