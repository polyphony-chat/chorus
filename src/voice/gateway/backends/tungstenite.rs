// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::VoiceCloseCode;
use crate::voice::gateway::{VoiceGatewayCommunication, VoiceGatewayMessage};

impl From<VoiceGatewayMessage> for tokio_tungstenite::tungstenite::Message {
    fn from(message: VoiceGatewayMessage) -> Self {
        Self::Text(message.0)
    }
}

impl From<tokio_tungstenite::tungstenite::Message> for VoiceGatewayMessage {
    fn from(value: tokio_tungstenite::tungstenite::Message) -> Self {
        Self(value.to_string())
    }
}

impl From<tokio_tungstenite::tungstenite::Message> for VoiceGatewayCommunication {
    fn from(value: tokio_tungstenite::tungstenite::Message) -> Self {
        match value {
            tokio_tungstenite::tungstenite::Message::Text(text) => {
                VoiceGatewayCommunication::Message(VoiceGatewayMessage(text))
            }
            tokio_tungstenite::tungstenite::Message::Close(close_frame) => {
                if close_frame.is_none() {
                    // Note: there is no unknown error. This case shouldn't happen, so I'm just
                    // going to delegate it to this error
                    return VoiceGatewayCommunication::Error(VoiceCloseCode::FailedToDecodePayload);
                }

                let close_code = u16::from(close_frame.unwrap().code);

                VoiceGatewayCommunication::Error(
                    VoiceCloseCode::try_from(close_code)
                        .unwrap_or(VoiceCloseCode::FailedToDecodePayload),
                )
            }
            _ => VoiceGatewayCommunication::Error(VoiceCloseCode::FailedToDecodePayload),
        }
    }
}
