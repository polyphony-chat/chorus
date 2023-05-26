use serde::{Deserialize, Serialize};
use crate::events::WebSocketEvent;

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