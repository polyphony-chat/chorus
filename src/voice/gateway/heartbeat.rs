use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures_util::SinkExt;
use log::*;
use safina_timer::sleep_until;
use tokio::sync::{mpsc::Sender, Mutex};

use crate::{
    gateway::heartbeat::HEARTBEAT_ACK_TIMEOUT,
    types::{VoiceGatewaySendPayload, VOICE_HEARTBEAT, VOICE_HEARTBEAT_ACK},
    voice::gateway::VoiceGatewayMessage,
};

use super::Sink;

/// Handles sending heartbeats to the voice gateway in another thread
#[allow(dead_code)] // FIXME: Remove this, once all fields of VoiceHeartbeatHandler are used
#[derive(Debug)]
pub(super) struct VoiceHeartbeatHandler {
    /// The heartbeat interval in milliseconds
    pub heartbeat_interval: Duration,
    /// The send channel for the heartbeat thread
    pub send: Sender<VoiceHeartbeatThreadCommunication>,
}

impl VoiceHeartbeatHandler {
    pub fn new(
        heartbeat_interval: Duration,
        starting_nonce: u64,
        websocket_tx: Arc<Mutex<Sink>>,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> Self {
        let (send, receive) = tokio::sync::mpsc::channel(32);
        let kill_receive = kill_rc.resubscribe();

        #[cfg(not(target_arch = "wasm32"))]
        tokio::task::spawn(async move {
            Self::heartbeat_task(
                websocket_tx,
                heartbeat_interval,
                starting_nonce,
                receive,
                kill_receive,
            )
            .await;
        });
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
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
        }
    }

    /// The main heartbeat task;
    ///
    /// Can be killed by the kill broadcast;
    /// If the websocket is closed, will die out next time it tries to send a heartbeat;
    pub async fn heartbeat_task(
        websocket_tx: Arc<Mutex<Sink>>,
        heartbeat_interval: Duration,
        starting_nonce: u64,
        mut receive: tokio::sync::mpsc::Receiver<VoiceHeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut last_heartbeat_timestamp: Instant = Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut nonce: u64 = starting_nonce;

        safina_timer::start_timer_thread();

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

                let msg = VoiceGatewayMessage(heartbeat_json);

                let send_result = websocket_tx.lock().await.send(msg.into()).await;
                if send_result.is_err() {
                    // We couldn't send, the websocket is broken
                    warn!("VGW: Couldnt send heartbeat, websocket seems broken");
                    break;
                }

                last_heartbeat_timestamp = Instant::now();
                last_heartbeat_acknowledged = false;
            }
        }
    }
}

/// Used for communications between the voice heartbeat and voice gateway thread.
/// Either signifies a nonce update, a heartbeat ACK or a Heartbeat request by the server
#[derive(Clone, Copy, Debug)]
pub(super) struct VoiceHeartbeatThreadCommunication {
    /// The opcode for the communication we received, if relevant
    pub(super) op_code: Option<u8>,
    /// The new nonce to use, if any
    pub(super) updated_nonce: Option<u64>,
}
