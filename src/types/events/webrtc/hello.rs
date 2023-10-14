use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Contains info on how often the client should send heartbeats to the server;
///
/// Differs from the normal hello data in that discord sends heartbeat interval as a float.
pub struct VoiceHelloData {
    /// How often a client should send heartbeats, in milliseconds
    pub heartbeat_interval: f64,
}

impl WebSocketEvent for VoiceHelloData {}
