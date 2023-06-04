use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct GatewayResume {
    pub token: String,
    pub session_id: String,
    pub seq: String,
}

impl WebSocketEvent for GatewayResume {}
