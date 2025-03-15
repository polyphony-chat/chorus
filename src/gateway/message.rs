// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::string::FromUtf8Error;

use crate::types::{CloseCode, GatewayReceivePayload};

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines a communication received from the gateway, being either an optionally compressed
/// [RawGatewayMessage] or a [CloseCode].
///
/// Used only for a tungstenite gateway, since our underlying wasm backend handles close codes
/// differently.
pub(crate) enum GatewayCommunication {
    Message(RawGatewayMessage),
    Error(CloseCode),
}

impl From<RawGatewayMessage> for GatewayCommunication {
    fn from(value: RawGatewayMessage) -> Self {
        Self::Message(value)
    }
}

/// Defines a raw gateway message, being either string json or bytes
///
/// This is used as an intermediary type between types from different websocket implementations
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RawGatewayMessage {
    Text(String),
    Bytes(Vec<u8>),
}

impl RawGatewayMessage {
    /// Attempt to consume the message into a String, will try to convert binary to utf8
    pub fn into_text(self) -> Result<String, FromUtf8Error> {
        match self {
            RawGatewayMessage::Text(text) => Ok(text),
            RawGatewayMessage::Bytes(bytes) => String::from_utf8(bytes),
        }
    }

    /// Consume the message into bytes, will convert text to binary
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            RawGatewayMessage::Text(text) => text.as_bytes().to_vec(),
            RawGatewayMessage::Bytes(bytes) => bytes,
        }
    }
}

/// Represents a json message received from the gateway.
///
/// This will usually be a [GatewayReceivePayload].
///
/// This struct is used internally when handling messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GatewayMessage(pub String);

impl GatewayMessage {
    /// Parses the message as a payload;
    /// Returns a result of deserializing
    pub fn payload(&self) -> Result<GatewayReceivePayload, serde_json::Error> {
        serde_json::from_str(&self.0)
    }

    /// Create self from an uncompressed json [RawGatewayMessage]
    pub(crate) fn from_raw_json_message(
        message: RawGatewayMessage,
    ) -> Result<GatewayMessage, FromUtf8Error> {
        let text = message.into_text()?;
        Ok(GatewayMessage(text))
    }

    /// Attempt to create self by decompressing zlib-stream bytes
    // Thanks to <https://github.com/ByteAlex/zlib-stream-rs>, their
    // code helped a lot with the stream implementation
    pub(crate) fn from_zlib_stream_json_bytes(
        bytes: &[u8],
        inflate: &mut flate2::Decompress,
    ) -> Result<GatewayMessage, std::io::Error> {
        // Note: is there a better way to handle the size of this output buffer?
        //
        // This used to be 10, I measured it at 11.5, so a safe bet feels like 20
        //
        // ^ - This dude is naive. apparently not even 20x is okay. Measured at 47.9x!!!!
        // If it is >100x ever, I will literally explode
        //
        // About an hour later, you ^ will literally explode.
        // 133 vs 13994 -- 105.21805x ratio
        // Let's hope it doesn't go above 200??
        let mut output = Vec::with_capacity(bytes.len() * 200);
        let _status = inflate.decompress_vec(bytes, &mut output, flate2::FlushDecompress::Sync)?;

        output.shrink_to_fit();

        let string = String::from_utf8(output).unwrap();

        Ok(GatewayMessage(string))
    }

    /// Attempt to create self by decompressing a zlib-stream bytes raw message
    pub(crate) fn from_zlib_stream_json_message(
        message: RawGatewayMessage,
        inflate: &mut flate2::Decompress,
    ) -> Result<GatewayMessage, std::io::Error> {
        Self::from_zlib_stream_json_bytes(&message.into_bytes(), inflate)
    }
}
