use std::sync::Arc;

use super::events::Events;
use super::*;
use ws_stream_wasm::*;

use crate::types::{self, WebSocketEvent};
#[derive(Debug)]
pub struct WasmGateway {
    events: Arc<Mutex<Events>>,
    heartbeat_handler: HeartbeatHandler,
    websocket_send: Arc<Mutex<SplitSink<WsStream, WsMessage>>>,
    websocket_receive: SplitStream<WsStream>,
    store: GatewayStore,
    url: String,
}
