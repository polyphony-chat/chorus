// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{entities::PublicUser, Snowflake};
use crate::types::{GuildMemberFlags, PermissionFlags, Shared};

use super::option_arc_rwlock_ptr_eq;

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

#[cfg(not(tarpaulin_include))]
impl PartialEq for GuildMember {
    fn eq(&self, other: &Self) -> bool {
        self.nick == other.nick
            && self.avatar == other.avatar
            && self.roles == other.roles
            && self.joined_at == other.joined_at
            && self.premium_since == other.premium_since
            && self.deaf == other.deaf
            && self.mute == other.mute
            && self.flags == other.flags
            && self.pending == other.pending
            && self.permissions == other.permissions
            && self.communication_disabled_until == other.communication_disabled_until
            && option_arc_rwlock_ptr_eq(&self.user, &other.user)
    }
}
