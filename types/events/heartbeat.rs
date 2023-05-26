use serde::{Deserialize, Serialize};
use crate::events::WebSocketEvent;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHeartbeat {
    pub op: u8,
    pub d: Option<u64>,
}

impl WebSocketEvent for GatewayHeartbeat {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHeartbeatAck {
    pub op: i32,
}

impl WebSocketEvent for GatewayHeartbeatAck {}