// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{errors::VoiceGatewayError, types::{VoiceGatewayReceivePayload, VoiceCloseCode}};

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines a communication received from the gateway, being either an optionally compressed
/// [RawGatewayMessage] or a [CloseCode].
///
/// Used only for a tungstenite gateway, since our underlying wasm backend handles close codes
/// differently.
pub(crate) enum VoiceGatewayCommunication {
    Message(VoiceGatewayMessage),
    Error(VoiceCloseCode),
}

impl From<VoiceGatewayMessage> for VoiceGatewayCommunication {
    fn from(value: VoiceGatewayMessage) -> Self {
        Self::Message(value)
    }
}

/// Represents a message received from the voice websocket connection.
///
/// This should be a [VoiceGatewayReceivePayload].
///
/// This struct is used internally when handling messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceGatewayMessage(pub String);

impl VoiceGatewayMessage {
    /// Parses the message as a payload;
    /// Returns a result of deserializing
    pub fn payload(&self) -> Result<VoiceGatewayReceivePayload, serde_json::Error> {
        serde_json::from_str(&self.0)
    }
}
