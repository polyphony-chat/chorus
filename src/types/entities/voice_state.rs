// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "client")]
use chorus_macros::Composite;

use crate::types::Shared;

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::types::{
    entities::{Guild, GuildMember},
    utils::Snowflake,
};

use super::option_arc_rwlock_ptr_eq;

/// The VoiceState struct. Note, that Discord does not have an `id` field for this, whereas Spacebar
/// does.
///
/// See <https://docs.spacebar.chat/routes/#cmp--schemas-voicestate>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "client", derive(Composite))]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub guild: Option<Guild>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub member: Option<Shared<GuildMember>>,
    /// Includes alphanumeric characters, not a snowflake
    pub session_id: String,
    pub token: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
    pub id: Option<Snowflake>, // Only exists on Spacebar
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for VoiceState {
    fn eq(&self, other: &Self) -> bool {
        self.guild_id == other.guild_id
            && self.guild == other.guild
            && self.channel_id == other.channel_id
            && self.user_id == other.user_id
            && option_arc_rwlock_ptr_eq(&self.member, &other.member)
            && self.session_id == other.session_id
            && self.token == other.token
            && self.deaf == other.deaf
            && self.mute == other.mute
            && self.self_deaf == other.self_deaf
            && self.self_mute == other.self_mute
            && self.self_stream == other.self_stream
            && self.self_video == other.self_video
            && self.suppress == other.suppress
            && self.request_to_speak_timestamp == other.request_to_speak_timestamp
            && self.id == other.id
    }
}

#[cfg(feature = "client")]
impl Updateable for VoiceState {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Snowflake {
        if let Some(id) = self.id {
            id // ID exists: Only the case for Spacebar Server impls
        } else {
            self.user_id // ID doesn't exist: Discord does not have the ID field - ID is void
        }
    }
}
