use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHello {
    pub op: i32,
    pub d: HelloData,
}

impl WebSocketEvent for GatewayHello {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HelloData {
    pub heartbeat_interval: u128,
}

impl WebSocketEvent for HelloData {}
