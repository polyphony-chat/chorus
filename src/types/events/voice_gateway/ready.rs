use std::net::Ipv4Addr;

use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

use super::VoiceEncryptionMode;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// The voice gateway's ready event;
///
/// Gives the user info about the udp connection ip and port, srrc to use,
/// available encryption modes and other data.
///
/// Sent in response to an Identify event.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#ready-structure>
pub struct VoiceReady {
    /// See <https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpStreamStats/ssrc>
    pub ssrc: u32,
    pub ip: Ipv4Addr,
    pub port: u16,
    /// The available encryption modes for the udp connection
    pub modes: Vec<VoiceEncryptionMode>,
    #[serde(default)]
    pub experiments: Vec<String>,
    // TODO: Add video streams
    // Heartbeat interval is also sent, but is "an erroneous field and should be ignored. The correct heartbeat_interval value comes from the Hello payload."
}

impl Default for VoiceReady {
    fn default() -> Self {
        VoiceReady {
            ssrc: 1,
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            modes: Vec::new(),
            experiments: Vec::new(),
        }
    }
}

impl WebSocketEvent for VoiceReady {}
