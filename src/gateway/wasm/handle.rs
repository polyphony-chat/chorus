use super::*;
use std::sync::Arc;

use crate::gateway::GatewayHandleCapable;
use ws_stream_wasm::*;

#[derive(Debug, Clone)]
pub struct WasmGatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<Events>>,
    pub websocket_send: Arc<Mutex<SplitSink<WsStream, WsMessage>>>,
    /// Tells gateway tasks to close
    pub(super) kill_send: tokio::sync::broadcast::Sender<()>,
    pub(crate) store: GatewayStore,
}

#[async_trait]
impl GatewayHandleCapable<WsMessage, WsStream> for WasmGatewayHandle {
    fn new(
        url: String,
        events: Arc<Mutex<Events>>,
        websocket_send: Arc<Mutex<SplitSink<WsStream, WsMessage>>>,
        kill_send: tokio::sync::broadcast::Sender<()>,
        store: GatewayStore,
    ) -> Self {
        Self {
            url,
            events,
            websocket_send,
            kill_send,
            store,
        }
    }
}
