use crate::types::{Snowflake, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
/// The identify payload for the webrtc stream;
///
/// Contains info to begin a webrtc connection;
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#identify-structure>
pub struct VoiceIdentify {
    /// The ID of the guild or the private channel being connected to
    pub server_id: Snowflake,
    pub user_id: Snowflake,
    pub session_id: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    // TODO: Add video streams
}

impl WebSocketEvent for VoiceIdentify {}
