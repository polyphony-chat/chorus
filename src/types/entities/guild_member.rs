// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::errors::ChorusError;
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Additional information about a user participating in a guild.
///
/// # See also
/// - [Guild::get_members_supplemental](crate::types::Guild::get_members_supplemental)
/// - [Guild::search_members](crate::types::Guild::search_members)
///
/// [ChorusUser]: [crate::instance::ChorusUser]
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#supplemental-guild-member-object>
pub struct SupplementalGuildMember {
    /// The ID of the user this member represents
    ///
    /// Note: this field is only included by the
    /// [Guild::get_members_supplemental](crate::types::Guild::get_members_supplemental) endpoint
    pub user_id: Option<Snowflake>,
    /// The associated member object
    ///
    /// Note: this field is only included by the
    /// [Guild::search_members](crate::types::Guild::search_members) endpoint
    pub member: Option<GuildMember>,
    /// How the user joined the guild
    pub join_source_type: JoinSourceType,
    /// The invite code or vanity link used to join the guild
    pub source_invite_code: Option<String>,
    /// The id of the user who invited the user to the guild
    pub inviter_id: Option<Snowflake>,
    // TODO: Add integration_type, once it's documented as a strongly typed enum
}

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord,
)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// How a user joined a guild
///
/// Used in [SupplementalGuildMember]
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#join-source-type>
pub enum JoinSourceType {
    /// The user joined through an unknown source
    Unspecified = 0,
    /// The user was added by a bot using the guilds.join oath scope
    Bot = 1,
    /// The user was added by an integration (e.g. Twitch)
    Integration = 2,
    /// The user joined from discovery
    Discovery = 3,
    /// The user joined through a student hub
    Hub = 4,
    /// The user joined from an invite
    Invite = 5,
    /// The user joined from a vanity url
    VanityUrl = 6,
    /// The user was accepted into the guild after applying for membership
    ManualMemberVerification = 7,
}

impl TryFrom<u8> for JoinSourceType {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Bot),
            2 => Ok(Self::Integration),
            3 => Ok(Self::Discovery),
            4 => Ok(Self::Hub),
            5 => Ok(Self::Invite),
            6 => Ok(Self::VanityUrl),
            7 => Ok(Self::ManualMemberVerification),
            _ => Err(ChorusError::InvalidArguments {
                error: "Value is not a valid JoinSourceType".to_string(),
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for JoinSourceType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for JoinSourceType {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::from(*self as u8);
        sqlx_pg_uint.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for JoinSourceType {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::decode(value)?;
        JoinSourceType::try_from(sqlx_pg_uint.to_uint()).map_err(|e| e.into())
    }
}
