use futures_util::SinkExt;
use log::*;
use std::time::{self, Duration, Instant};
use tokio::sync::mpsc::{Receiver, Sender};

use safina_timer::sleep_until;
use tokio::task::{self, JoinHandle};

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
    /// The handle of the thread
    handle: JoinHandle<()>,
}

impl HeartbeatHandler {
    pub fn new(
        heartbeat_interval: Duration,
        websocket_tx: Arc<Mutex<WsSink>>,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> Self {
        let (send, receive) = tokio::sync::mpsc::channel(32);
        let kill_receive = kill_rc.resubscribe();

        let handle: JoinHandle<()> = task::spawn(async move {
            Self::heartbeat_task(websocket_tx, heartbeat_interval, receive, kill_receive).await;
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
        websocket_tx: Arc<Mutex<WsSink>>,
        heartbeat_interval: Duration,
        mut receive: Receiver<HeartbeatThreadCommunication>,
        mut kill_receive: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut last_heartbeat_timestamp: Instant = time::Instant::now();
        let mut last_heartbeat_acknowledged = true;
        let mut last_seq_number: Option<u64> = None;

        safina_timer::start_timer_thread();

        loop {
            if kill_receive.try_recv().is_ok() {
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

            if should_send {
                trace!("GW: Sending Heartbeat..");

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
