// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::string::FromUtf8Error;

use crate::types;

use super::*;

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
/// This will be either a [types::GatewayReceivePayload], containing events, or a [GatewayError].
/// This struct is used internally when handling messages.
#[derive(Clone, Debug)]
pub struct GatewayMessage(pub String);

impl GatewayMessage {
    /// Parses the message as an error;
    /// Returns the error if successfully parsed, None if the message isn't an error
    pub fn error(&self) -> Option<GatewayError> {
        // Some error strings have dots on the end, which we don't care about
        let processed_content = self.0.to_lowercase().replace('.', "");

        match processed_content.as_str() {
            "unknown error" | "4000" => Some(GatewayError::Unknown),
            "unknown opcode" | "4001" => Some(GatewayError::UnknownOpcode),
            "decode error" | "error while decoding payload" | "4002" => Some(GatewayError::Decode),
            "not authenticated" | "4003" => Some(GatewayError::NotAuthenticated),
            "authentication failed" | "4004" => Some(GatewayError::AuthenticationFailed),
            "already authenticated" | "4005" => Some(GatewayError::AlreadyAuthenticated),
            "invalid seq" | "4007" => Some(GatewayError::InvalidSequenceNumber),
            "rate limited" | "4008" => Some(GatewayError::RateLimited),
            "session timed out" | "4009" => Some(GatewayError::SessionTimedOut),
            "invalid shard" | "4010" => Some(GatewayError::InvalidShard),
            "sharding required" | "4011" => Some(GatewayError::ShardingRequired),
            "invalid api version" | "4012" => Some(GatewayError::InvalidAPIVersion),
            "invalid intent(s)" | "invalid intent" | "4013" => Some(GatewayError::InvalidIntents),
            "disallowed intent(s)" | "disallowed intents" | "4014" => {
                Some(GatewayError::DisallowedIntents)
            }
            _ => None,
        }
    }

    /// Parses the message as a payload;
    /// Returns a result of deserializing
    pub fn payload(&self) -> Result<types::GatewayReceivePayload, serde_json::Error> {
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
        let mut output = Vec::with_capacity(bytes.len() * 10);
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
