// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, Snowflake, VoiceState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, PartialEq, Eq, WebSocketEvent)]
///
/// Sent to the server to indicate an update of the voice state (leave voice channel, join voice channel, mute, deafen);
///
/// Not to be confused with [VoiceStateUpdate];
pub struct UpdateVoiceState {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#voice-state-update>;
///
/// Received from the server to indicate an update in a user's voice state (leave voice channel, join voice channel, mute, deafen, etc);
///
/// Not to be confused with [UpdateVoiceState];
pub struct VoiceStateUpdate {
    #[serde(flatten)]
    pub state: VoiceState,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#voice-server-update>;
///
/// Received to indicate which voice endpoint, token and guild_id to use;
pub struct VoiceServerUpdate {
    pub token: String,
    /// The guild this voice server update is for
    pub guild_id: Option<Snowflake>,
    /// The private channel this voice server update is for
    pub channel_id: Option<Snowflake>,
    pub endpoint: Option<String>,
}

