// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{GuildMemberFlags, PermissionFlags, Shared};
use crate::types::{entities::PublicUser, Snowflake};

#[derive(Debug, Deserialize, Default, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Represents a participating user in a guild.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/guild#guild-member-object>
pub struct GuildMember {
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<PublicUser>>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub roles: Vec<Snowflake>,
    pub joined_at: DateTime<Utc>,
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: Option<GuildMemberFlags>,
    pub pending: Option<bool>,
    #[serde(default)]
    pub permissions: PermissionFlags,
    pub communication_disabled_until: Option<DateTime<Utc>>,
}
