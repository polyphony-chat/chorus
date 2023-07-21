use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

/// Received on gateway init, tells the client how often to send heartbeats;
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHello {
    pub op: i32,
    pub d: HelloData,
}

impl WebSocketEvent for GatewayHello {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Contains info on how often the client should send heartbeats to the server;
pub struct HelloData {
    /// How often a client should send heartbeats, in milliseconds
    pub heartbeat_interval: u64,
}

impl WebSocketEvent for HelloData {}
