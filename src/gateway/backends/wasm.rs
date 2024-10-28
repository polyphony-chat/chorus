// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use ws_stream_wasm::*;

use crate::gateway::{GatewayMessage, RawGatewayMessage};

#[derive(Debug, Clone, Copy)]
pub struct WasmBackend;

// These could be made into inherent associated types when that's stabilized
pub type WasmSink = SplitSink<WsStream, WsMessage>;
/// Note: this includes WsMeta so we can subscribe to events as well
pub type WasmStream = (SplitStream<WsStream>, WsMeta);

impl WasmBackend {
    pub async fn connect(
        websocket_url: &str,
    ) -> Result<(WasmSink, WasmStream), ws_stream_wasm::WsErr> {
        let (meta, websocket_stream) = WsMeta::connect(websocket_url, None).await?;

        let (sink, stream) = websocket_stream.split();

        Ok((sink, (stream, meta)))
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

impl From<RawGatewayMessage> for WsMessage {
    fn from(message: RawGatewayMessage) -> Self {
        match message {
            RawGatewayMessage::Text(text) => WsMessage::Text(text),
            RawGatewayMessage::Bytes(bytes) => WsMessage::Binary(bytes),
        }
    }
}

impl From<WsMessage> for RawGatewayMessage {
    fn from(value: WsMessage) -> Self {
        match value {
            WsMessage::Binary(bytes) => RawGatewayMessage::Bytes(bytes),
            WsMessage::Text(text) => RawGatewayMessage::Text(text),
        }
    }
}
