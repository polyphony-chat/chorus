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
pub type WasmSink = SplitSink<WsStream, WsMessage>;
pub type WasmStream = SplitStream<WsStream>;

impl WasmBackend {
    pub async fn connect(
        websocket_url: &str,
    ) -> Result<(WasmSink, WasmStream), crate::errors::GatewayError> {
        let (_, websocket_stream) = match WsMeta::connect(websocket_url, None).await {
            Ok(stream) => Ok(stream),
            Err(e) => Err(GatewayError::CannotConnect {
                error: e.to_string(),
            }),
        }?;

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
        match value {
            WsMessage::Text(text) => Self(text),
            WsMessage::Binary(bin) => {
                let mut text = String::new();
                let _ = bin.iter().map(|v| text.push_str(&v.to_string()));
                Self(text)
            }
        }
    }
}
