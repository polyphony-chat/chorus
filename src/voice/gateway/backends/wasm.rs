// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use ws_stream_wasm::*;

use crate::errors::VoiceGatewayError;
use crate::voice::gateway::VoiceGatewayMessage;

#[derive(Debug, Clone)]
pub struct WasmBackend;

// These could be made into inherent associated types when that's stabilized
pub type WasmSink = SplitSink<WsStream, WsMessage>;
pub type WasmStream = SplitStream<WsStream>;

impl WasmBackend {
    pub async fn connect(websocket_url: &str) -> Result<(WasmSink, WasmStream), VoiceGatewayError> {
        let (_, websocket_stream) = match WsMeta::connect(websocket_url, None).await {
            Ok(stream) => Ok(stream),
            Err(e) => Err(VoiceGatewayError::CannotConnect {
                error: e.to_string(),
            }),
        }?;

        Ok(websocket_stream.split())
    }
}

impl From<VoiceGatewayMessage> for WsMessage {
    fn from(message: VoiceGatewayMessage) -> Self {
        Self::Text(message.0)
    }
}

impl From<WsMessage> for VoiceGatewayMessage {
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
