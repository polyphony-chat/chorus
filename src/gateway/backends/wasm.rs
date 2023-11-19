use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use ws_stream_wasm::*;

use crate::errors::GatewayError;
use crate::gateway::GatewayMessage;

#[derive(Debug, Clone)]
pub struct WasmBackend;

// These could be made into inherent associated types when that's stabilized
pub type WasmSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>;
pub type WasmStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

impl WasmBackend {
    pub async fn connect(
        websocket_url: &str,
    ) -> Result<(WasmSink, WasmStream), crate::errors::GatewayError> {
        let (websocket_stream, _) = match WsMeta::connect();
        {
            Ok(websocket_stream) => websocket_stream,
            Err(e) => {
                return Err(GatewayError::CannotConnect {
                    error: e.to_string(),
                })
            }
        };

        Ok(websocket_stream.split())
    }
}

impl From<GatewayMessage> for WsMessage {
    fn from(message: GatewayMessage) -> Self {
        Self::Text(message.0)
    }
}

impl From<WsMessage> for GatewayMessage {
    fn from(value: WsMessage) -> Self {
        Self(value.to_string())
    }
}
