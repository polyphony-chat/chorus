use serde::{Deserialize, Serialize};
use crate::events::{PresenceUpdate, WebSocketEvent};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: GatewayIdentifyConnectionProps,
    pub compress: Option<bool>,
    pub large_threshold: Option<i16>, //default: 50
    pub shard: Option<Vec<(i32, i32)>>,
    pub presence: Option<PresenceUpdate>,
    pub intents: i32,
}

impl WebSocketEvent for GatewayIdentifyPayload {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayIdentifyConnectionProps {
    pub os: String,
    pub browser: String,
    pub device: String,
}