// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::voice::gateway::VoiceGatewayMessage;

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
