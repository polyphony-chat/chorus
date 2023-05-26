use crate::types::events::{PresenceUpdate, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: GatewayIdentifyConnectionProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i16>, //default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<(i32, i32)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceUpdate>,
    // What is the difference between these two?
    // Intents is documented, capabilities is used in users
    // I wonder if these are interchangable..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<i32>,
}

impl GatewayIdentifyPayload {
    /// Creates an identify payload with the same default capabilities as the official client
    pub fn default_w_client_capabilities() -> Self {
        let mut def = Self::default();
        def.capabilities = Some(8189); // Default capabilities for a client
        def
    }

    /// Creates an identify payload with all possible capabilities
    pub fn default_w_all_capabilities() -> Self {
        let mut def = Self::default();
        def.capabilities = Some(i32::MAX); // Since discord uses bitwise for capabilities, this has almost every bit as 1, so all capabilities
        def
    }
}

impl WebSocketEvent for GatewayIdentifyPayload {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayIdentifyConnectionProps {
    pub os: String,
    pub browser: String,
    pub device: String,
}
