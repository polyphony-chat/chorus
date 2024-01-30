// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{Snowflake, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
/// The identify payload for the webrtc stream;
/// Contains info to begin a webrtc connection;
/// See https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-websocket-connection-example-voice-identify-payload;
pub struct VoiceIdentify {
    server_id: Snowflake,
    user_id: Snowflake,
    session_id: String,
    token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Undocumented field, but is also in discord client comms
    video: Option<bool>,
}

impl WebSocketEvent for VoiceIdentify {}
