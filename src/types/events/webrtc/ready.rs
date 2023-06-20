use std::net::Ipv4Addr;

use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

use super::WebrtcEncryptionMode;

#[derive(Debug, Deserialize, Serialize, Clone)]
/// The ready event for the webrtc stream;
/// Used to give info after the identify event;
/// See https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection-example-voice-ready-payload;
pub struct VoiceReady {
    /// See https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpStreamStats/ssrc
    ssrc: i32,
    ip: Ipv4Addr,
    port: u32,
    /// The available encryption modes for the webrtc connection
    modes: Vec<WebrtcEncryptionMode>,
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
