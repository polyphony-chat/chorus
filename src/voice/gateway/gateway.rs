// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{sync::Arc, time::Duration};

use log::*;

use tokio::sync::Mutex;

use futures_util::SinkExt;
use futures_util::StreamExt;

use crate::{
    errors::VoiceGatewayError,
    gateway::GatewayEvent,
    types::{
        VoiceGatewayReceivePayload, VoiceHelloData, WebSocketEvent, VOICE_BACKEND_VERSION,
        VOICE_CLIENT_CONNECT_FLAGS, VOICE_CLIENT_CONNECT_PLATFORM, VOICE_CLIENT_DISCONNECT,
        VOICE_HEARTBEAT, VOICE_HEARTBEAT_ACK, VOICE_HELLO, VOICE_IDENTIFY, VOICE_MEDIA_SINK_WANTS,
        VOICE_READY, VOICE_RESUME, VOICE_SELECT_PROTOCOL, VOICE_SESSION_DESCRIPTION,
        VOICE_SESSION_UPDATE, VOICE_SPEAKING, VOICE_SSRC_DEFINITION,
    },
    voice::gateway::{
        heartbeat::VoiceHeartbeatThreadCommunication, VoiceGatewayMessage, WebSocketBackend,
    },
};

use super::{
    events::VoiceEvents, heartbeat::VoiceHeartbeatHandler, Sink, Stream, VoiceGatewayHandle,
};

#[derive(Debug)]
pub struct VoiceGateway {
    events: Arc<Mutex<VoiceEvents>>,
    heartbeat_handler: VoiceHeartbeatHandler,
    websocket_send: Arc<Mutex<Sink>>,
    websocket_receive: Stream,
    kill_send: tokio::sync::broadcast::Sender<()>,
    kill_receive: tokio::sync::broadcast::Receiver<()>,
}

impl VoiceGateway {
    #[allow(clippy::new_ret_no_self)]
    pub async fn spawn(websocket_url: String) -> Result<VoiceGatewayHandle, VoiceGatewayError> {
        // Append the needed things to the websocket url
        let processed_url = format!("wss://{}/?v=7", websocket_url);
        trace!("Created voice socket url: {}", processed_url.clone());

        let (websocket_send, mut websocket_receive) =
            WebSocketBackend::connect(&processed_url).await?;

        let shared_websocket_send = Arc::new(Mutex::new(websocket_send));

        // Create a shared broadcast channel for killing all gateway tasks
        let (kill_send, mut _kill_receive) = tokio::sync::broadcast::channel::<()>(16);

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        #[cfg(not(target_arch = "wasm32"))]
        let msg: VoiceGatewayMessage = websocket_receive.next().await.unwrap().unwrap().into();
        #[cfg(target_arch = "wasm32")]
        let msg: VoiceGatewayMessage = websocket_receive.next().await.unwrap().into();
        let gateway_payload: VoiceGatewayReceivePayload = serde_json::from_str(&msg.0).unwrap();

        if gateway_payload.op_code != VOICE_HELLO {
            return Err(VoiceGatewayError::NonHelloOnInitiate {
                opcode: gateway_payload.op_code,
            });
        }

        info!("VGW: Received Hello");

        // The hello data for voice gateways is in float milliseconds, so we convert it to f64 seconds
        let gateway_hello: VoiceHelloData =
            serde_json::from_str(gateway_payload.data.get()).unwrap();
        let heartbeat_interval_seconds: f64 = gateway_hello.heartbeat_interval / 1000.0;

        let voice_events = VoiceEvents::default();
        let shared_events = Arc::new(Mutex::new(voice_events));

        let mut gateway = VoiceGateway {
            events: shared_events.clone(),
            heartbeat_handler: VoiceHeartbeatHandler::new(
                Duration::from_secs_f64(heartbeat_interval_seconds),
                1, // to:do actually compute nonce
                shared_websocket_send.clone(),
                kill_send.subscribe(),
            ),
            websocket_send: shared_websocket_send.clone(),
            websocket_receive,
            kill_send: kill_send.clone(),
            kill_receive: kill_send.subscribe(),
        };

        // Now we can continuously check for messages in a different task, since we aren't going to receive another hello
        #[cfg(not(target_arch = "wasm32"))]
        tokio::task::spawn(async move {
            gateway.gateway_listen_task().await;
        });
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            gateway.gateway_listen_task().await;
        });

        Ok(VoiceGatewayHandle {
            url: websocket_url.clone(),
            events: shared_events,
            websocket_send: shared_websocket_send.clone(),
            kill_send: kill_send.clone(),
        })
    }

    /// The main gateway listener task;
    pub async fn gateway_listen_task(&mut self) {
        loop {
            let msg;

            tokio::select! {
                Ok(_) = self.kill_receive.recv() => {
                    log::trace!("VGW: Closing listener task");
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
    pub async fn handle_message(&mut self, msg: VoiceGatewayMessage) {
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

        // See <https://discord.com/developers/docs/topics/voice-connections>
        match gateway_payload.op_code {
            VOICE_READY => {
                trace!("VGW: Received READY!");

                let event = &mut self.events.lock().await.voice_ready;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!("Failed to parse VOICE_READY ({})", result.err().unwrap());
                }
            }
            VOICE_BACKEND_VERSION => {
                trace!("VGW: Received Backend Version");

                let event = &mut self.events.lock().await.backend_version;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_BACKEND_VERSION ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_SESSION_DESCRIPTION => {
                trace!("VGW: Received Session Description");

                let event = &mut self.events.lock().await.session_description;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_SESSION_DESCRIPTION ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_SESSION_UPDATE => {
                trace!("VGW: Received Session Update");

                let event = &mut self.events.lock().await.session_update;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_SESSION_UPDATE ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_SPEAKING => {
                trace!("VGW: Received Speaking");

                let event = &mut self.events.lock().await.speaking;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!("Failed to parse VOICE_SPEAKING ({})", result.err().unwrap());
                }
            }
            VOICE_SSRC_DEFINITION => {
                trace!("VGW: Received Ssrc Definition");

                let event = &mut self.events.lock().await.ssrc_definition;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_SSRC_DEFINITION ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_CLIENT_DISCONNECT => {
                trace!("VGW: Received Client Disconnect");

                let event = &mut self.events.lock().await.client_disconnect;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_CLIENT_DISCONNECT ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_CLIENT_CONNECT_FLAGS => {
                trace!("VGW: Received Client Connect Flags");

                let event = &mut self.events.lock().await.client_connect_flags;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_CLIENT_CONNECT_FLAGS ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_CLIENT_CONNECT_PLATFORM => {
                trace!("VGW: Received Client Connect Platform");

                let event = &mut self.events.lock().await.client_connect_platform;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_CLIENT_CONNECT_PLATFORM ({})",
                        result.err().unwrap()
                    );
                }
            }
            VOICE_MEDIA_SINK_WANTS => {
                trace!("VGW: Received Media Sink Wants");

                let event = &mut self.events.lock().await.media_sink_wants;
                let result = VoiceGateway::handle_event(gateway_payload.data.get(), event).await;
                if result.is_err() {
                    warn!(
                        "Failed to parse VOICE_MEDIA_SINK_WANTS ({})",
                        result.err().unwrap()
                    );
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
                trace!("VGW: Received Heartbeat ACK");

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
                info!(
                    "VGW: Received unexpected opcode ({}) for current state. This might be due to a faulty server implementation and is likely not the fault of chorus.",
                    gateway_payload.op_code
                );
            }
            _ => {
                warn!("VGW: Received unrecognized voice gateway op code ({})! Please open an issue on the chorus github so we can implement it", gateway_payload.op_code);
            }
        }
    }
}
