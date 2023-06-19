use crate::errors::GatewayError;
use crate::errors::ObserverError;
use crate::gateway::events::Events;
use crate::types;
use crate::types::WebSocketEvent;
use std::sync::Arc;

use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use native_tls::TlsConnector;
use tokio::net::TcpStream;
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

/// The amount of time we wait for a heartbeat ack before resending our heartbeat in ms
const HEARTBEAT_ACK_TIMEOUT: u128 = 2000;

#[derive(Clone, Debug)]
/**
Represents a messsage received from the gateway. This will be either a [GatewayReceivePayload], containing events, or a [GatewayError].
This struct is used internally when handling messages.
*/
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
            "unknown error" | "4000" => Some(GatewayError::UnknownError),
            "unknown opcode" | "4001" => Some(GatewayError::UnknownOpcodeError),
            "decode error" | "error while decoding payload" | "4002" => {
                Some(GatewayError::DecodeError)
            }
            "not authenticated" | "4003" => Some(GatewayError::NotAuthenticatedError),
            "authentication failed" | "4004" => Some(GatewayError::AuthenticationFailedError),
            "already authenticated" | "4005" => Some(GatewayError::AlreadyAuthenticatedError),
            "invalid seq" | "4007" => Some(GatewayError::InvalidSequenceNumberError),
            "rate limited" | "4008" => Some(GatewayError::RateLimitedError),
            "session timed out" | "4009" => Some(GatewayError::SessionTimedOutError),
            "invalid shard" | "4010" => Some(GatewayError::InvalidShardError),
            "sharding required" | "4011" => Some(GatewayError::ShardingRequiredError),
            "invalid api version" | "4012" => Some(GatewayError::InvalidAPIVersionError),
            "invalid intent(s)" | "invalid intent" | "4013" => {
                Some(GatewayError::InvalidIntentsError)
            }
            "disallowed intent(s)" | "disallowed intents" | "4014" => {
                Some(GatewayError::DisallowedIntentsError)
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
    /// Tells gateway tasks to close
    kill_send: tokio::sync::broadcast::Sender<()>,
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
    pub async fn send_update_presence(&self, to_send: types::UpdatePresence) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Update Presence..");

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

    /// Closes the websocket connection and stops all gateway tasks;
    ///
    /// Esentially pulls the plug on the gateway, leaving it possible to resume;
    pub async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}

pub struct Gateway {
    pub events: Arc<Mutex<Events>>,
    heartbeat_handler: HeartbeatHandler,
    pub websocket_send: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
    pub websocket_receive: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    kill_send: tokio::sync::broadcast::Sender<()>,
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
                return Err(GatewayError::CannotConnectError {
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
            return Err(GatewayError::NonHelloOnInitiateError {
                opcode: gateway_payload.op_code,
            });
        }

        println!("GW: Received Hello");

        let gateway_hello: types::HelloData =
            serde_json::from_str(gateway_payload.event_data.unwrap().get()).unwrap();

        let events = Events::default();
        let shared_events = Arc::new(Mutex::new(events));

        let mut gateway = Gateway {
            events: shared_events.clone(),
            heartbeat_handler: HeartbeatHandler::new(
                gateway_hello.heartbeat_interval,
                shared_websocket_send.clone(),
                kill_send.subscribe(),
            ),
            websocket_send: shared_websocket_send.clone(),
            websocket_receive,
            kill_send: kill_send.clone(),
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
            println!("GW: Websocket is broken, stopping gateway");
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
    async fn handle_event<'a, T: WebSocketEvent + serde::Deserialize<'a>>(
        data: &'a str,
        event: &mut GatewayEvent<T>,
    ) -> Result<(), serde_json::Error> {
        let data_deserialize_result: Result<T, serde_json::Error> = serde_json::from_str(data);

        if data_deserialize_result.is_err() {
            return Err(data_deserialize_result.err().unwrap());
        }

        event.update_data(data_deserialize_result.unwrap()).await;
        Ok(())
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    pub async fn handle_message(&mut self, msg: GatewayMessage) {
        if msg.is_empty() {
            return;
        }

        if !msg.is_error() && !msg.is_payload() {
            println!(
                "Message unrecognised: {:?}, please open an issue on the chorus github",
                msg.message.to_string()
            );
            return;
        }

        // To:do: handle errors in a good way, maybe observers like events?
        if msg.is_error() {
            println!("GW: Received error, connection will close..");

            let error = msg.error();

            match error {
                _ => {}
            }

            self.close().await;
            return;
        }

        let gateway_payload = msg.payload().unwrap();

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
                        let event = &mut self.events.lock().await.session.ready;

                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;

                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "READY_SUPPLEMENTAL" => {
                        let event = &mut self.events.lock().await.session.ready_supplemental;

                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;

                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "RESUMED" => {}
                    "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {
                        let event = &mut self
                            .events
                            .lock()
                            .await
                            .application
                            .command_permissions_update;

                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;

                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "AUTO_MODERATION_RULE_CREATE" => {
                        let event = &mut self.events.lock().await.auto_moderation.rule_create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "AUTO_MODERATION_RULE_UPDATE" => {
                        let event = &mut self.events.lock().await.auto_moderation.rule_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "AUTO_MODERATION_RULE_DELETE" => {
                        let event = &mut self.events.lock().await.auto_moderation.rule_delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "AUTO_MODERATION_ACTION_EXECUTION" => {
                        let event = &mut self.events.lock().await.auto_moderation.action_execution;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CHANNEL_CREATE" => {
                        let event = &mut self.events.lock().await.channel.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CHANNEL_UPDATE" => {
                        let event = &mut self.events.lock().await.channel.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CHANNEL_UNREAD_UPDATE" => {
                        let event = &mut self.events.lock().await.channel.unread_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CHANNEL_DELETE" => {
                        let event = &mut self.events.lock().await.channel.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CHANNEL_PINS_UPDATE" => {
                        let event = &mut self.events.lock().await.channel.pins_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CALL_CREATE" => {
                        let event = &mut self.events.lock().await.call.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CALL_UPDATE" => {
                        let event = &mut self.events.lock().await.call.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "CALL_DELETE" => {
                        let event = &mut self.events.lock().await.call.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "THREAD_CREATE" => {
                        let event = &mut self.events.lock().await.thread.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "THREAD_UPDATE" => {
                        let event = &mut self.events.lock().await.thread.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "THREAD_DELETE" => {
                        let event = &mut self.events.lock().await.thread.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "THREAD_LIST_SYNC" => {
                        let event = &mut self.events.lock().await.thread.list_sync;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "THREAD_MEMBER_UPDATE" => {
                        let event = &mut self.events.lock().await.thread.member_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "THREAD_MEMBERS_UPDATE" => {
                        let event = &mut self.events.lock().await.thread.members_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_CREATE" => {
                        let event = &mut self.events.lock().await.guild.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_DELETE" => {
                        let event = &mut self.events.lock().await.guild.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_AUDIT_LOG_ENTRY_CREATE" => {
                        let event = &mut self.events.lock().await.guild.audit_log_entry_create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_BAN_ADD" => {
                        let event = &mut self.events.lock().await.guild.ban_add;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_BAN_REMOVE" => {
                        let event = &mut self.events.lock().await.guild.ban_remove;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_EMOJIS_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.emojis_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_STICKERS_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.stickers_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_INTEGRATIONS_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.integrations_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_MEMBER_ADD" => {
                        let event = &mut self.events.lock().await.guild.member_add;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_MEMBER_REMOVE" => {
                        let event = &mut self.events.lock().await.guild.member_remove;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_MEMBER_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.member_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_MEMBERS_CHUNK" => {
                        let event = &mut self.events.lock().await.guild.members_chunk;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_ROLE_CREATE" => {
                        let event = &mut self.events.lock().await.guild.role_create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_ROLE_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.role_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_ROLE_DELETE" => {
                        let event = &mut self.events.lock().await.guild.role_delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_SCHEDULED_EVENT_CREATE" => {
                        let event = &mut self.events.lock().await.guild.role_scheduled_event_create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_SCHEDULED_EVENT_UPDATE" => {
                        let event = &mut self.events.lock().await.guild.role_scheduled_event_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_SCHEDULED_EVENT_DELETE" => {
                        let event = &mut self.events.lock().await.guild.role_scheduled_event_delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_SCHEDULED_EVENT_USER_ADD" => {
                        let event =
                            &mut self.events.lock().await.guild.role_scheduled_event_user_add;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {
                        let event = &mut self
                            .events
                            .lock()
                            .await
                            .guild
                            .role_scheduled_event_user_remove;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "PASSIVE_UPDATE_V1" => {
                        let event = &mut self.events.lock().await.guild.passive_update_v1;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "INTEGRATION_CREATE" => {
                        let event = &mut self.events.lock().await.integration.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "INTEGRATION_UPDATE" => {
                        let event = &mut self.events.lock().await.integration.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "INTEGRATION_DELETE" => {
                        let event = &mut self.events.lock().await.integration.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "INTERACTION_CREATE" => {
                        let event = &mut self.events.lock().await.interaction.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "INVITE_CREATE" => {
                        let event = &mut self.events.lock().await.invite.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "INVITE_DELETE" => {
                        let event = &mut self.events.lock().await.invite.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_CREATE" => {
                        let event = &mut self.events.lock().await.message.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_UPDATE" => {
                        let event = &mut self.events.lock().await.message.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_DELETE" => {
                        let event = &mut self.events.lock().await.message.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_DELETE_BULK" => {
                        let event = &mut self.events.lock().await.message.delete_bulk;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_REACTION_ADD" => {
                        let event = &mut self.events.lock().await.message.reaction_add;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_REACTION_REMOVE" => {
                        let event = &mut self.events.lock().await.message.reaction_remove;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_REACTION_REMOVE_ALL" => {
                        let event = &mut self.events.lock().await.message.reaction_remove_all;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_REACTION_REMOVE_EMOJI" => {
                        let event = &mut self.events.lock().await.message.reaction_remove_emoji;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "MESSAGE_ACK" => {
                        let event = &mut self.events.lock().await.message.ack;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "PRESENCE_UPDATE" => {
                        let event = &mut self.events.lock().await.user.presence_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "RELATIONSHIP_ADD" => {
                        let event = &mut self.events.lock().await.relationship.add;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "RELATIONSHIP_REMOVE" => {
                        let event = &mut self.events.lock().await.relationship.remove;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "STAGE_INSTANCE_CREATE" => {
                        let event = &mut self.events.lock().await.stage_instance.create;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "STAGE_INSTANCE_UPDATE" => {
                        let event = &mut self.events.lock().await.stage_instance.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "STAGE_INSTANCE_DELETE" => {
                        let event = &mut self.events.lock().await.stage_instance.delete;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "SESSIONS_REPLACE" => {
                        let result: Result<Vec<types::Session>, serde_json::Error> =
                            serde_json::from_str(gateway_payload.event_data.unwrap().get());
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }

                        let data = types::SessionsReplace {
                            sessions: result.unwrap(),
                        };

                        self.events
                            .lock()
                            .await
                            .session
                            .replace
                            .update_data(data)
                            .await;
                    }
                    "USER_UPDATE" => {
                        let event = &mut self.events.lock().await.user.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "USER_GUILD_SETTINGS_UPDATE" => {
                        let event = &mut self.events.lock().await.user.guild_settings_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "VOICE_STATE_UPDATE" => {
                        let event = &mut self.events.lock().await.voice.state_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "VOICE_SERVER_UPDATE" => {
                        let event = &mut self.events.lock().await.voice.server_update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    "WEBHOOKS_UPDATE" => {
                        let event = &mut self.events.lock().await.webhooks.update;
                        let result =
                            Gateway::handle_event(gateway_payload.event_data.unwrap().get(), event)
                                .await;
                        if result.is_err() {
                            println!(
                                "Failed to parse gateway event {} ({})",
                                gateway_payload_t,
                                result.err().unwrap()
                            );
                            return;
                        }
                    }
                    _ => {
                        println!("Received unrecognized gateway event ({})! Please open an issue on the chorus github so we can implement it", &gateway_payload_t);
                    }
                }
            }
            // We received a heartbeat from the server
            // "Discord may send the app a Heartbeat (opcode 1) event, in which case the app should send a Heartbeat event immediately."
            GATEWAY_HEARTBEAT => {
                println!("GW: Received Heartbeat // Heartbeat Request");

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
                panic!("Received hello when it was unexpected");
            }
            GATEWAY_HEARTBEAT_ACK => {
                println!("GW: Received Heartbeat ACK");

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
                let error = GatewayError::UnexpectedOpcodeReceivedError {
                    opcode: gateway_payload.op_code,
                };
                Err::<(), GatewayError>(error).unwrap();
            }
            _ => {
                println!("Received unrecognized gateway op code ({})! Please open an issue on the chorus github so we can implement it", gateway_payload.op_code);
            }
        }

        // If we we received a seq number we should let it know
        if gateway_payload.sequence_number.is_some() {
            let heartbeat_communication = HeartbeatThreadCommunication {
                sequence_number: Some(gateway_payload.sequence_number.unwrap()),
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
        heartbeat_interval: u128,
        mut receive: tokio::sync::mpsc::Receiver<HeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut last_heartbeat_timestamp: Instant = time::Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut last_seq_number: Option<u64> = None;

        loop {
            let should_shutdown = kill_receive.try_recv().is_ok();
            if should_shutdown {
                break;
            }

            let mut should_send;

            let time_to_send = last_heartbeat_timestamp.elapsed().as_millis() >= heartbeat_interval;

            should_send = time_to_send;

            let received_communication: Result<HeartbeatThreadCommunication, TryRecvError> =
                receive.try_recv();
            if received_communication.is_ok() {
                let communication = received_communication.unwrap();

                // If we received a seq number update, use that as the last seq number
                if communication.sequence_number.is_some() {
                    last_seq_number = Some(communication.sequence_number.unwrap());
                }

                if communication.op_code.is_some() {
                    match communication.op_code.unwrap() {
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

            // If the server hasn't acknowledged our heartbeat we should resend it
            if !last_heartbeat_acknowledged
                && last_heartbeat_timestamp.elapsed().as_millis() > HEARTBEAT_ACK_TIMEOUT
            {
                should_send = true;
                println!("GW: Timed out waiting for a heartbeat ack, resending");
            }

            if should_send {
                println!("GW: Sending Heartbeat..");

                let heartbeat = types::GatewayHeartbeat {
                    op: GATEWAY_HEARTBEAT,
                    d: last_seq_number,
                };

                let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                let send_result = websocket_tx.lock().await.send(msg).await;
                if send_result.is_err() {
                    // We couldn't send, the websocket is broken
                    println!("GW: Couldnt send heartbeat, websocket seems broken");
                    break;
                }

                last_heartbeat_timestamp = time::Instant::now();
                last_heartbeat_acknowledged = false;
            }
        }
    }
}

/**
Used for communications between the heartbeat and gateway thread.
Either signifies a sequence number update, a heartbeat ACK or a Heartbeat request by the server
*/
#[derive(Clone, Copy, Debug)]
struct HeartbeatThreadCommunication {
    /// The opcode for the communication we received, if relevant
    op_code: Option<u8>,
    /// The sequence number we got from discord, if any
    sequence_number: Option<u64>,
}

/**
Trait which defines the behavior of an Observer. An Observer is an object which is subscribed to
an Observable. The Observer is notified when the Observable's data changes.
In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
 */
pub trait Observer<T: types::WebSocketEvent>: std::fmt::Debug {
    fn update(&mut self, data: &T);
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
            .retain(|obs| format!("{:?}", obs) != format!("{:?}", &observable));
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
            let mut observer_lock = observer.lock().await;
            observer_lock.update(&self.event_data);
            drop(observer_lock);
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
        fn update(&mut self, data: &types::GatewayResume) {
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

        event.subscribe(arc_mut_consumer.clone()).unwrap();

        event.notify().await;

        event.update_data(new_data).await;

        let second_consumer = Consumer;
        let arc_mut_second_consumer = Arc::new(Mutex::new(second_consumer));

        match event.subscribe(arc_mut_second_consumer.clone()).err() {
            None => panic!(),
            Some(err) => println!("You cannot subscribe twice: {}", err),
        }

        event.unsubscribe(arc_mut_consumer.clone());

        event.subscribe(arc_mut_second_consumer).unwrap();
    }
}
