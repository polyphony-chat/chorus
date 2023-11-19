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

#[async_trait(?Send)]
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

    async fn send_json_event(&self, op_code: u8, to_send: serde_json::Value) {
        self.send_json_event(op_code, to_send).await
    }

    async fn observe<U: Updateable + Clone + std::fmt::Debug + Composite<U> + Send + Sync>(
        &self,
        object: Arc<RwLock<U>>,
    ) -> Arc<RwLock<U>> {
        self.observe(object).await
    }

    async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}
