// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ws_stream_wasm::WsMessage;
use crate::voice::gateway::VoiceGatewayMessage;

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

