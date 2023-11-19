use std::sync::Arc;

use super::events::Events;
use super::*;
use pharos::*;
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

#[async_trait]
impl GatewayCapable<WsMessage, WsStream> for WasmGateway {
    fn get_events(&self) -> Arc<Mutex<Events>> {
        self.events.clone()
    }

    fn get_websocket_send(&self) -> Arc<Mutex<SplitSink<WsStream, WsMessage>>> {
        self.websocket_send.clone()
    }

    fn get_store(&self) -> GatewayStore {
        self.store.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    async fn spawn<G: GatewayHandleCapable<WsMessage, WsStream>>(
        websocket_url: String,
    ) -> Result<G, GatewayError> {
        let (mut websocket_stream, _) = match WsMeta::connect(websocket_url, None).await {
            Ok(ws) => Ok(ws),
            Err(e) => Err(GatewayError::CannotConnect {
                error: e.to_string(),
            }),
        }?;

        let mut event = match websocket_stream
            .observe(ObserveConfig::channel(self, Channel::Unbounded))
            .await
        {
            Ok(ok) => Ok(ok),
            Err(e) => Err(GatewayError::CannotConnect { error: e }),
        }?;
    }
}
