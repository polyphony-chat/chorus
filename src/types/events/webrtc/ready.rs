use std::net::Ipv4Addr;

use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// The ready event for the webrtc stream;
/// Used to give info after the identify event;
/// See https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection-example-voice-ready-payload;
pub struct VoiceReady {
    ssrc: i32,
    ip: Ipv4Addr,
    port: u32,
    modes: Vec<String>,
    // Heartbeat interval is also sent, but is "an erroneous field and should be ignored. The correct heartbeat_interval value comes from the Hello payload."
}

impl Default for VoiceReady {
    fn default() -> Self {
        VoiceReady {
            ssrc: 1,
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            modes: Vec::new(),
        }
    }
}

impl WebSocketEvent for VoiceReady {}
