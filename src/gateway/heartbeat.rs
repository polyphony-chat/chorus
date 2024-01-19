use futures_util::SinkExt;
use log::*;
use std::time::{self, Duration, Instant};
use tokio::sync::mpsc::{Receiver, Sender};

use safina_timer::sleep_until;
#[cfg(not(target_arch = "wasm32"))]
use tokio::task;

use super::*;
use crate::types;

/// The amount of time we wait for a heartbeat ack before resending our heartbeat in ms
const HEARTBEAT_ACK_TIMEOUT: u64 = 2000;

/// Handles sending heartbeats to the gateway in another thread
#[allow(dead_code)] // FIXME: Remove this, once HeartbeatHandler is used
#[derive(Debug)]
pub(super) struct HeartbeatHandler {
    /// How ofter heartbeats need to be sent at a minimum
    pub heartbeat_interval: Duration,
    /// The send channel for the heartbeat thread
    pub send: Sender<HeartbeatThreadCommunication>,
}

impl HeartbeatHandler {
    pub fn new(
        heartbeat_interval: Duration,
        websocket_tx: Arc<Mutex<Sink>>,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> Self {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&String::from("HBH: Creating new self").into());

        let (send, receive) = tokio::sync::mpsc::channel(32);
        let kill_receive = kill_rc.resubscribe();

        #[cfg(not(target_arch = "wasm32"))]
        task::spawn(async move {
            Self::heartbeat_task(websocket_tx, heartbeat_interval, receive, kill_receive).await;
        });
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            Self::heartbeat_task(websocket_tx, heartbeat_interval, receive, kill_receive).await;
        });
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&String::from("HBH: Spawned task").into());

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
        mut receive: Receiver<HeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&String::from("HBH: Running task").into());
        let mut last_heartbeat_timestamp: Instant = time::Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut last_seq_number: Option<u64> = None;
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&String::from("HBH: Initialized variables").into());

        safina_timer::start_timer_thread();

        loop {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&String::from("HBH: L0").into());

            println!("HBH: L0");
            if kill_receive.try_recv().is_ok() {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&String::from("HBH: dying").into());

                println!("HBH: dying");
                trace!("GW: Closing heartbeat task");
                break;
            }

            let timeout = if last_heartbeat_acknowledged {
                heartbeat_interval
            } else {
                // If the server hasn't acknowledged our heartbeat we should resend it
                Duration::from_millis(HEARTBEAT_ACK_TIMEOUT)
            };

            let mut should_send = false;

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&String::from("HBH: L1").into());

            println!("HBH: L1");

            tokio::select! {
                () = sleep_until(last_heartbeat_timestamp + timeout) => {
                    should_send = true;
                }
                Some(communication) = receive.recv() => {
                    // If we received a seq number update, use that as the last seq number
                    if communication.sequence_number.is_some() {
                        last_seq_number = communication.sequence_number;
                    }

                    if let Some(op_code) = communication.op_code {
                        match op_code {
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
            }

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&String::from("HBH: L2").into());

            println!("HBH: L2");

            if should_send {
                trace!("GW: Sending Heartbeat..");

                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&String::from("HBH: L3, sending").into());

                println!("HBH: L3, sending");

                let heartbeat = types::GatewayHeartbeat {
                    op: GATEWAY_HEARTBEAT,
                    d: last_seq_number,
                };

                let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                let msg = GatewayMessage(heartbeat_json);

                let send_result = websocket_tx.lock().await.send(msg.into()).await;
                if send_result.is_err() {
                    // We couldn't send, the websocket is broken
                    warn!("GW: Couldnt send heartbeat, websocket seems broken");
                    break;
                }

                last_heartbeat_timestamp = time::Instant::now();
                last_heartbeat_acknowledged = false;

                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&String::from("HBH: L4, sending done").into());
            }
        }
    }
}

/// Used for communications between the heartbeat and gateway thread.
/// Either signifies a sequence number update, a heartbeat ACK or a Heartbeat request by the server
#[derive(Clone, Copy, Debug)]
pub(super) struct HeartbeatThreadCommunication {
    /// The opcode for the communication we received, if relevant
    pub(super) op_code: Option<u8>,
    /// The sequence number we got from discord, if any
    pub(super) sequence_number: Option<u64>,
}
