use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GatewayResume {
    pub token: String,
    pub session_id: String,
    pub seq: String,
}

impl WebSocketEvent for GatewayResume {}
