use serde::{Deserialize, Serialize};
use crate::events::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayResume {
    pub token: String,
    pub session_id: String,
    pub seq: String,
}

impl WebSocketEvent for GatewayResume {}