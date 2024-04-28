// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types;

use super::*;

/// Represents a message received from the gateway. This will be either a [types::GatewayReceivePayload], containing events, or a [GatewayError].
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
}
