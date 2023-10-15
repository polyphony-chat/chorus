use crate::types::{Snowflake, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Sent when another user disconnects from the voice server.
///
/// When received, the SSRC of the user should be discarded.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#other-client-disconnection>
pub struct VoiceClientDisconnection {
    pub user_id: Snowflake,
}

impl WebSocketEvent for VoiceClientDisconnection {}
