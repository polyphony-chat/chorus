use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
/// Received from the server to describe the backend version.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#voice-backend-version>
pub struct VoiceBackendVersion {
    /// The voice backend's version
    #[serde(rename = "voice")]
    pub voice_version: String,
    /// The WebRTC worker's version
    #[serde(rename = "rtc_worker")]
    pub rtc_worker_version: String,
}

impl WebSocketEvent for VoiceBackendVersion {}
