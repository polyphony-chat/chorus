use tokio::task::JoinHandle;
use ws_stream_wasm::*;

use super::*;

/// Handles sending heartbeats to the gateway in another thread
#[allow(dead_code)] // FIXME: Remove this, once used
#[derive(Debug)]
pub struct WasmHeartbeatHandler {
    /// How ofter heartbeats need to be sent at a minimum
    pub heartbeat_interval: Duration,
    /// The send channel for the heartbeat thread
    pub send: Sender<HeartbeatThreadCommunication>,
    /// The handle of the thread
    handle: JoinHandle<()>,
}
