use std::str::Utf8Error;

use crate::types;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GatewayMessageData {
    Text(String),
    Binary(Vec<u8>),
}

impl From<tokio_tungstenite::tungstenite::Message> for GatewayMessageData {
    fn from(value: tokio_tungstenite::tungstenite::Message) -> Self {
        match value {
            Message::Text(string) => Self::Text(string),
            Message::Binary(data) | Message::Ping(data) | Message::Pong(data) => Self::Binary(data),
            Message::Close(data) => {
                if let Some(data) = data {
                    Self::Text(data.code.to_string())
                } else {
                    Self::Text(String::new())
                }
            }
            Message::Frame(data) => Self::Binary(data.into_data()),
        }
    }
}

impl From<ws_stream_wasm::WsMessage> for GatewayMessageData {
    fn from(value: ws_stream_wasm::WsMessage) -> Self {
        match value {
            ws_stream_wasm::WsMessage::Text(string) => Self::Text(string),
            ws_stream_wasm::WsMessage::Binary(data) => Self::Binary(data),
        }
    }
}

impl From<String> for GatewayMessageData {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl GatewayMessageData {
    pub fn to_text(&self) -> Result<&str, Utf8Error> {
        match *self {
            GatewayMessageData::Text(ref text) => Ok(text),
            GatewayMessageData::Binary(ref data) => Ok(std::str::from_utf8(data)?),
        }
    }
}

/// Represents a messsage received from the gateway. This will be either a [types::GatewayReceivePayload], containing events, or a [GatewayError].
/// This struct is used internally when handling messages.
#[derive(Clone, Debug)]
pub struct GatewayMessage {
    /// The message we received from the server
    pub(crate) message: tokio_tungstenite::tungstenite::Message,
}

impl GatewayMessage {
    /// Creates self from a tungstenite message
    pub fn from_tungstenite_message(message: tokio_tungstenite::tungstenite::Message) -> Self {
        Self { message }
    }

    /// Parses the message as an error;
    /// Returns the error if succesfully parsed, None if the message isn't an error
    pub fn error(&self) -> Option<GatewayError> {
        let content = self.message.to_string();

        // Some error strings have dots on the end, which we don't care about
        let processed_content = content.to_lowercase().replace('.', "");

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

    /// Returns whether or not the message is an error
    pub fn is_error(&self) -> bool {
        self.error().is_some()
    }

    /// Parses the message as a payload;
    /// Returns a result of deserializing
    pub fn payload(&self) -> Result<types::GatewayReceivePayload, serde_json::Error> {
        return serde_json::from_str(self.message.to_text().unwrap());
    }

    /// Returns whether or not the message is a payload
    pub fn is_payload(&self) -> bool {
        // close messages are never payloads, payloads are only text messages
        if self.message.is_close() | !self.message.is_text() {
            return false;
        }

        return self.payload().is_ok();
    }

    /// Returns whether or not the message is empty
    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }
}
