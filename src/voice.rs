use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::StreamExt;
use log::{debug, info, trace, warn};
use native_tls::TlsConnector;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tokio::time::{self, sleep_until};
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async_tls_with_config, Connector, WebSocketStream};

use crate::errors::VoiceGatewayError;
use crate::gateway::{GatewayEvent, HEARTBEAT_ACK_TIMEOUT};
use crate::types::{
    self, SelectProtocol, Speaking, VoiceGatewayReceivePayload, VoiceGatewaySendPayload,
    VoiceIdentify, WebSocketEvent, VOICE_BACKEND_VERSION, VOICE_HEARTBEAT, VOICE_HEARTBEAT_ACK,
    VOICE_HELLO, VOICE_IDENTIFY, VOICE_READY, VOICE_RESUME, VOICE_SELECT_PROTOCOL,
    VOICE_SESSION_DESCRIPTION, VOICE_SPEAKING,
};

/// Represents a messsage received from the webrtc socket. This will be either a [GatewayReceivePayload], containing webrtc events, or a [WebrtcError].
/// This struct is used internally when handling messages.
#[derive(Clone, Debug)]
pub struct VoiceGatewayMesssage {
    /// The message we received from the server
    message: tokio_tungstenite::tungstenite::Message,
}

impl VoiceGatewayMesssage {
    /// Creates self from a tungstenite message
    pub fn from_tungstenite_message(message: tokio_tungstenite::tungstenite::Message) -> Self {
        Self { message }
    }

    /// Parses the message as an error;
    /// Returns the error if succesfully parsed, None if the message isn't an error
    pub fn error(&self) -> Option<VoiceGatewayError> {
        let content = self.message.to_string();

        // Some error strings have dots on the end, which we don't care about
        let processed_content = content.to_lowercase().replace('.', "");

        match processed_content.as_str() {
            "unknown opcode" | "4001" => Some(VoiceGatewayError::UnknownOpcodeError),
            "decode error" | "failed to decode payload" | "4002" => {
                Some(VoiceGatewayError::FailedToDecodePayloadError)
            }
            "not authenticated" | "4003" => Some(VoiceGatewayError::NotAuthenticatedError),
            "authentication failed" | "4004" => Some(VoiceGatewayError::AuthenticationFailedError),
            "already authenticated" | "4005" => Some(VoiceGatewayError::AlreadyAuthenticatedError),
            "session no longer valid" | "4006" => {
                Some(VoiceGatewayError::SessionNoLongerValidError)
            }
            "session timeout" | "4009" => Some(VoiceGatewayError::SessionTimeoutError),
            "server not found" | "4011" => Some(VoiceGatewayError::ServerNotFoundError),
            "unknown protocol" | "4012" => Some(VoiceGatewayError::UnknownProtocolError),
            "disconnected" | "4014" => Some(VoiceGatewayError::DisconnectedError),
            "voice server crashed" | "4015" => Some(VoiceGatewayError::VoiceServerCrashedError),
            "unknown encryption mode" | "4016" => {
                Some(VoiceGatewayError::UnknownEncryptionModeError)
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
    pub fn payload(&self) -> Result<VoiceGatewayReceivePayload, serde_json::Error> {
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

/// Represents a handle to a Voice Gateway connection.
/// Using this handle you can send Gateway Events directly.
#[derive(Debug)]
pub struct VoiceGatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<voice_events::VoiceEvents>>,
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

impl VoiceGatewayHandle {
    /// Sends json to the gateway with an opcode
    async fn send_json(&self, op_code: u8, to_send: serde_json::Value) {
        let gateway_payload = VoiceGatewaySendPayload {
            op_code,
            data: to_send,
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

    /// Sends a voice identify event to the gateway
    pub async fn send_identify(&self, to_send: VoiceIdentify) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending Identify..");

        self.send_json(VOICE_IDENTIFY, to_send_value).await;
    }

    /// Sends a select protocol event to the gateway
    pub async fn send_select_protocol(&self, to_send: SelectProtocol) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending Select Protocol");

        self.send_json(VOICE_SELECT_PROTOCOL, to_send_value).await;
    }

    /// Sends a speaking event to the gateway
    pub async fn send_speaking(&self, to_send: Speaking) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending Speaking");

        self.send_json(VOICE_SPEAKING, to_send_value).await;
    }

    /// Sends a voice backend version request to the gateway
    pub async fn send_voice_backend_version_request(&self) {
        let data_empty_object = json!("{}");

        trace!("VGW: Requesting voice backend version");

        self.send_json(VOICE_BACKEND_VERSION, data_empty_object)
            .await;
    }

    /// Closes the websocket connection and stops all gateway tasks;
    ///
    /// Esentially pulls the plug on the voice gateway, leaving it possible to resume;
    pub async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}
pub struct VoiceGateway {
    events: Arc<Mutex<voice_events::VoiceEvents>>,
    heartbeat_handler: VoiceHeartbeatHandler,
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
}

impl VoiceGateway {
    #[allow(clippy::new_ret_no_self)]
    pub async fn new(websocket_url: String) -> Result<VoiceGatewayHandle, VoiceGatewayError> {
        // Append the needed things to the websocket url
        let processed_url = format!("wss://{}?v=4", websocket_url);

        let (websocket_stream, _) = match connect_async_tls_with_config(
            &processed_url,
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
                return Err(VoiceGatewayError::CannotConnect {
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
        let gateway_payload: VoiceGatewayReceivePayload =
            serde_json::from_str(msg.to_text().unwrap()).unwrap();

        if gateway_payload.op_code != VOICE_HELLO {
            return Err(VoiceGatewayError::NonHelloOnInitiate {
                opcode: gateway_payload.op_code,
            });
        }

        info!("VGW: Received Hello");

        // The hello data is the same on voice and normal gateway
        let gateway_hello: types::HelloData =
            serde_json::from_str(gateway_payload.data.get()).unwrap();

        let voice_events = voice_events::VoiceEvents::default();
        let shared_events = Arc::new(Mutex::new(voice_events));

        let mut gateway = VoiceGateway {
            events: shared_events.clone(),
            heartbeat_handler: VoiceHeartbeatHandler::new(
                Duration::from_millis(gateway_hello.heartbeat_interval),
                1, // to:do actually compute nonce
                shared_websocket_send.clone(),
                kill_send.subscribe(),
            ),
            websocket_send: shared_websocket_send.clone(),
            websocket_receive,
            kill_send: kill_send.clone(),
        };

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        let handle: JoinHandle<()> = tokio::spawn(async move {
            gateway.gateway_listen_task().await;
        });

        Ok(VoiceGatewayHandle {
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

            if let Some(Ok(message)) = msg {
                self.handle_message(VoiceGatewayMesssage::from_tungstenite_message(message))
                    .await;
                continue;
            }

            // We couldn't receive the next message or it was an error, something is wrong with the websocket, close
            warn!("VGW: Websocket is broken, stopping gateway");
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

        event.notify(data_deserialize_result.unwrap()).await;
        Ok(())
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    pub async fn handle_message(&mut self, msg: VoiceGatewayMesssage) {
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

        // To:do: handle errors in a good way, maybe observers like events?
        if msg.is_error() {
            warn!("VGW: Received error, connection will close..");

            let _error = msg.error();

            self.close().await;
            return;
        }

        let gateway_payload = msg.payload().unwrap();

        match gateway_payload.op_code {
            VOICE_READY => {
                let event = &mut self.events.lock().await.voice_ready;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!("Failed to parse VOICE_READY ({})", result.err().unwrap());
                    return;
                }
            }
            VOICE_SESSION_DESCRIPTION => {
                let event = &mut self.events.lock().await.session_description;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_SELECT_PROTOCOL ({})",
                        result.err().unwrap()
                    );
                    return;
                }
            }
            // We received a heartbeat from the server
            // "Discord may send the app a Heartbeat (opcode 1) event, in which case the app should send a Heartbeat event immediately."
            VOICE_HEARTBEAT => {
                trace!("VGW: Received Heartbeat // Heartbeat Request");

                // Tell the heartbeat handler it should send a heartbeat right away
                let heartbeat_communication = VoiceHeartbeatThreadCommunication {
                    updated_nonce: None,
                    op_code: Some(VOICE_HEARTBEAT),
                };

                self.heartbeat_handler
                    .send
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
            VOICE_HEARTBEAT_ACK => {
                debug!("VGW: Received Heartbeat ACK");

                // Tell the heartbeat handler we received an ack

                let heartbeat_communication = VoiceHeartbeatThreadCommunication {
                    updated_nonce: None,
                    op_code: Some(VOICE_HEARTBEAT_ACK),
                };

                self.heartbeat_handler
                    .send
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
            VOICE_IDENTIFY | VOICE_SELECT_PROTOCOL | VOICE_RESUME => {
                let error = VoiceGatewayError::UnexpectedOpcodeReceived {
                    opcode: gateway_payload.op_code,
                };
                Err::<(), VoiceGatewayError>(error).unwrap();
            }
            _ => {
                warn!("Received unrecognized voice gateway op code ({})! Please open an issue on the chorus github so we can implement it", gateway_payload.op_code);
            }
        }
    }
}

/// Handles sending heartbeats to the voice gateway in another thread
#[allow(dead_code)] // FIXME: Remove this, once all fields of VoiceHeartbeatHandler are used
struct VoiceHeartbeatHandler {
    /// The heartbeat interval in milliseconds
    pub heartbeat_interval: Duration,
    /// The send channel for the heartbeat thread
    pub send: Sender<VoiceHeartbeatThreadCommunication>,
    /// The handle of the thread
    handle: JoinHandle<()>,
}

impl VoiceHeartbeatHandler {
    pub fn new(
        heartbeat_interval: Duration,
        starting_nonce: u64,
        websocket_tx: Arc<
            Mutex<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tokio_tungstenite::tungstenite::Message,
                >,
            >,
        >,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> Self {
        let (send, receive) = tokio::sync::mpsc::channel(32);
        let kill_receive = kill_rc.resubscribe();

        let handle: JoinHandle<()> = tokio::spawn(async move {
            Self::heartbeat_task(
                websocket_tx,
                heartbeat_interval,
                starting_nonce,
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
        starting_nonce: u64,
        mut receive: tokio::sync::mpsc::Receiver<VoiceHeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut last_heartbeat_timestamp: Instant = time::Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut nonce: u64 = starting_nonce;

        loop {
            if kill_receive.try_recv().is_ok() {
                trace!("VGW: Closing heartbeat task");
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

                    // If we received a nonce update, use that nonce now
                if communication.updated_nonce.is_some() {
                    nonce = communication.updated_nonce.unwrap();
                }

                    if let Some(op_code) = communication.op_code {
                        match op_code {
                            VOICE_HEARTBEAT => {
                                // As per the api docs, if the server sends us a Heartbeat, that means we need to respond with a heartbeat immediately
                                should_send = true;
                            }
                            VOICE_HEARTBEAT_ACK => {
                                // The server received our heartbeat
                                last_heartbeat_acknowledged = true;
                            }
                            _ => {}
                        }
                    }
                }
            }

            if should_send {
                trace!("VGW: Sending Heartbeat..");

                let heartbeat = VoiceGatewaySendPayload {
                    op_code: VOICE_HEARTBEAT,
                    data: nonce.into(),
                };

                let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                let send_result = websocket_tx.lock().await.send(msg).await;
                if send_result.is_err() {
                    // We couldn't send, the websocket is broken
                    warn!("VGW: Couldnt send heartbeat, websocket seems broken");
                    break;
                }

                last_heartbeat_timestamp = time::Instant::now();
                last_heartbeat_acknowledged = false;
            }
        }
    }
}

/// Used for communications between the voice heartbeat and voice gateway thread.
/// Either signifies a nonce update, a heartbeat ACK or a Heartbeat request by the server
#[derive(Clone, Copy, Debug)]
struct VoiceHeartbeatThreadCommunication {
    /// The opcode for the communication we received, if relevant
    op_code: Option<u8>,
    /// The new nonce to use, if any
    updated_nonce: Option<u64>,
}

mod voice_events {
    use crate::types::{SessionDescription, VoiceReady};

    use super::*;

    #[derive(Default, Debug)]
    pub struct VoiceEvents {
        pub voice_ready: GatewayEvent<VoiceReady>,
        pub session_description: GatewayEvent<SessionDescription>,
    }
}
