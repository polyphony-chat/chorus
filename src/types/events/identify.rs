// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, ClientProperties};
use serde::{Deserialize, Serialize};

use super::GatewayIdentifyPresenceUpdate;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, WebSocketEvent)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: ClientProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i16>,
    //default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<(i32, i32)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<GatewayIdentifyPresenceUpdate>,
    // What is the difference between these two?
    // Intents is documented, capabilities is used in users
    // I wonder if these are interchangeable...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<i32>,
}

impl Default for GatewayIdentifyPayload {
    fn default() -> Self {
        Self {
            token: String::new(),
            properties: ClientProperties::default(),
            compress: None,
            large_threshold: None,
            shard: None,
            presence: None,
            intents: None,
            capabilities: None,
        }
    }
}

impl GatewayIdentifyPayload {
    /// Uses the most common data along with client capabilities
    ///
    /// Basically pretends to be an official client on Windows 10
    pub fn common() -> Self {
        Self {
            properties: ClientProperties::default(),
            capabilities: Some(8189),
            ..Self::default()
        }
    }

    /// Creates an identify payload with the same default capabilities as the official client
    pub fn default_w_client_capabilities() -> Self {
        Self {
            capabilities: Some(8189), // Default capabilities for a client
            ..Self::default()
        }
    }

    /// Creates an identify payload with all possible capabilities
    pub fn default_w_all_capabilities() -> Self {
        Self {
            capabilities: Some(i32::MAX), // Since discord uses bitwise for capabilities, this has almost every bit as 1, so all capabilities
            ..Self::default()
        }
    }
}
