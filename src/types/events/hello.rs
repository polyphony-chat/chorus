use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Received on gateway init, tells the client how often to send heartbeats;
pub struct GatewayHello {
    pub op: i32,
    pub d: HelloData,
}

impl WebSocketEvent for GatewayHello {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Contains info on how often the client should send heartbeats to the server;
pub struct HelloData {
    /// How often a client should send heartbeats, in milliseconds
    // u128 because std used u128s for milliseconds
    pub heartbeat_interval: u128,
}

impl WebSocketEvent for HelloData {}
