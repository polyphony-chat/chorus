// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{Snowflake, WebSocketEvent};
use chorus_macros::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy, WebSocketEvent)]
/// Sent when another user disconnects from the voice server.
///
/// When received, the SSRC of the user should be discarded.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#other-client-disconnection>
pub struct VoiceClientDisconnection {
    pub user_id: Snowflake,
}

