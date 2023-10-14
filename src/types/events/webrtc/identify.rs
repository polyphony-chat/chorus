use crate::types::{Snowflake, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
/// The identify payload for the webrtc stream;
///
/// Contains info to begin a webrtc connection;
///
/// See <https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection-example-voice-identify-payload>
pub struct VoiceIdentify {
    pub server_id: Snowflake,
    pub user_id: Snowflake,
    pub session_id: String,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Undocumented field, but is also in discord client comms
    pub video: Option<bool>,
}

impl WebSocketEvent for VoiceIdentify {}
