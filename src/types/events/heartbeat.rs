use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHeartbeat {
    pub op: u8,
    pub d: Option<u64>,
}

impl WebSocketEvent for GatewayHeartbeat {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct GatewayHeartbeatAck {
    pub op: i32,
}

impl WebSocketEvent for GatewayHeartbeatAck {}
