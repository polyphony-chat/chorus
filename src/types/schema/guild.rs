// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

use crate::errors::ChorusError;
use crate::types::entities::Channel;
use crate::types::types::guild_configuration::GuildFeatures;
use crate::types::{
    Emoji, ExplicitContentFilterLevel, GenericSearchQueryWithLimit, GuildMember, JoinSourceType,
    MFALevel, MessageNotificationLevel, RoleObject, Snowflake, Sticker, StickerFormatType,
    SupplementalGuildMember, SystemChannelFlags, ThemeColors, VerificationLevel,
    WelcomeScreenChannel,
};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create a Guild.
///
/// # Reference
/// See <https://docs.spacebar.chat/routes/#cmp--schemas-guildcreateschema> and <https://docs.discord.sex/resources/guild#create-guild>
pub struct GuildCreateSchema {
    /// The name of the guild (2-100 characters, excluding trailing and leading whitespace)
    pub name: Option<String>,

    /// The main voice region ID of the guild
    ///
    /// Note: this field is deprecated
    pub region: Option<String>,

    /// See <https://docs.discord.sex/reference#cdn-data>
    pub icon: Option<String>,

    /// Note: this field is not implemented yet on Spacebar, see <https://github.com/spacebarchat/server/issues/1251>
    pub verification_level: Option<VerificationLevel>,

    /// The default [MessageNotificationLevel] for members of the guild
    ///
    /// Note: this field is not implemented yet on Spacebar, see <https://github.com/spacebarchat/server/issues/1251>
    pub default_message_notifications: Option<MessageNotificationLevel>,

    /// Whose messages are scanned for explicit content
    ///
    /// Note: this field is not implemented yet on Spacebar, see <https://github.com/spacebarchat/server/issues/1251>
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,

    /// Roles in the new guild
    ///
    /// The first member of the array is used to change properties of the guild's @everyone role.
    ///
    /// The id field within each role object is a placeholder, which will be replaced by the server
    ///
    /// Note: this field is not implemented yet on Spacebar, see <https://github.com/spacebarchat/server/issues/1251>
    pub roles: Option<Vec<RoleObject>>,

    /// Channels in the new guild
    ///
    /// When set, none of the default channels are created, and the position field is always
    /// ignored
    ///
    /// The id field within each channel object is a placeholder, which will be replaced by the
    /// server
    pub channels: Option<Vec<Channel>>,

    /// Whether the new guild will only be accessible for instance staff
    ///
    /// Can only be set by instance staff
    ///
    /// Note: this field is not implemented yet on Spacebar, see <https://github.com/spacebarchat/server/issues/1251>
    pub staff_only: Option<bool>,

    /// The ID of the channel where system event messages, like member joins and boosts are posted
    pub system_channel_id: Option<Snowflake>,

    /// Flags that sets which messages are sent in the system channel
    ///
    /// Note: this field is not implemented yet on Spacebar, see <https://github.com/spacebarchat/server/issues/1251>
    pub system_channel_flags: Option<SystemChannelFlags>,

    /// The ID of the channel which contains the guild's rules
    ///
    /// Note: it is unclear whether this is an official part of the schema or an addition for
    /// Spacebar
    pub rules_channel_id: Option<Snowflake>,

    /// The template code that was used for this guild, used for analytics
    pub guild_template_code: Option<String>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for GuildCreateSchema {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.region == other.region
            && self.icon == other.icon
            && self.channels == other.channels
            && self.guild_template_code == other.guild_template_code
            && self.system_channel_id == other.system_channel_id
            && self.rules_channel_id == other.rules_channel_id
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create a Guild Ban.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/guild#create-guild-ban>
pub struct GuildBanCreateSchema {
    /// Number of seconds to delete messages for (0-604800, default 0)
    pub delete_message_seconds: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Schema for the [crate::instance::ChorusUser::leave_guild] route
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#json-params>
pub(crate) struct GuildLeaveSchema {
    /// "Whether the user is lurking in the guild"
    pub lurking: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Schema for the bulk guild ban endpoint.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#json-params>
pub struct BulkGuildBanSchema {
    /// The user IDs to ban (max 200)
    pub user_ids: Vec<Snowflake>,
    /// Number of seconds to delete messages for (0-604800, default 0)
    pub delete_message_seconds: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Return type for the bulk guild ban endpoint.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#response-body>
pub struct BulkGuildBanReturn {
    /// The user IDs that were successfully banned
    pub banned_users: Vec<Snowflake>,

    /// The user IDs that were not banned
    ///
    /// They may have failed because:
    /// - the user is already banned
    /// - the user has a higher role than the current user
    /// - the user is the owner of the guild
    /// - the user is the current user
    pub failed_users: Vec<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Schema for the Add Guild Role Members endpoint.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#add-guild-role-members>
pub struct AddRoleMembersSchema {
    /// The member IDs to assign the role to (max 30)
    pub member_ids: Vec<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Represents the schema used to modify a guild.
/// See: <https://docs.discord.sex/resources/guild#modify-guild>
pub struct GuildModifySchema {
    /// The name of the guild (2-100 characters, excluding trailing and leading whitespace)
    pub name: Option<String>,

    /// See <https://docs.discord.sex/reference#cdn-data>
    pub icon: Option<String>,

    /// See <https://docs.discord.sex/reference#cdn-data>
    pub banner: Option<String>,

    /// The guild's banner
    ///
    /// For it tobe shown, the guild must have the BANNER feature
    ///
    /// See <https://docs.discord.sex/reference#cdn-data>
    pub home_header: Option<String>,

    /// The guild's invite splash
    ///
    /// For it to be shown, the guild must have the INVITE_SPLASH feature
    ///
    /// See <https://docs.discord.sex/reference#cdn-data>
    pub splash: Option<String>,

    /// The guild's discovery splash
    pub discovery_splash: Option<String>,

    /// The user ID of the guild's owner (must be the current owner to change)
    pub owner_id: Option<Snowflake>,

    /// The description of the guild
    pub description: Option<String>,

    /// The main voice region ID of the guild
    ///
    /// Note: deprecated
    pub region: Option<String>,

    /// The ID of the guild's AFK channel
    ///
    /// This is where members in voice idle for longer than afk_timeout are moved
    pub afk_channel_id: Option<Snowflake>,

    /// The AFK timeout of the guild (one of 60, 300, 900, 1800, 3600, in seconds)
    pub afk_timeout: Option<u16>,

    pub verification_level: Option<VerificationLevel>,

    /// The default [MessageNotificationLevel] for members of the guild
    pub default_message_notifications: Option<MessageNotificationLevel>,

    /// Whose messages are scanned for explicit content
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,

    pub features: Option<Vec<GuildFeatures>>,

    /// The ID of the channel where system messages, such as member joins and boosts, are posted
    pub system_channel_id: Option<Snowflake>,

    /// Flags that sets which messages are sent in the system channel
    pub system_channel_flags: Option<SystemChannelFlags>,

    /// The ID of the channel where community guilds display rules
    ///
    /// If set to Some(1), will create a new #rules channel
    ///
    /// Reference: <https://docs.discord.sex/resources/guild#modify-guild>
    pub rules_channel_id: Option<Snowflake>,

    /// The ID of the channel where admins and moderators of community guilds receive notices from Discord
    ///
    /// If set to Some(1), will create a new #moderator-only channel
    ///
    pub public_updates_channel_id: Option<Snowflake>,

    /// The ID of the channel where admins and moderators of community guilds receive safety alerts from Discord
    pub safety_alerts_channel_id: Option<Snowflake>,

    /// The preferred locale of the guild, used in discovery and notices from Discord
    ///
    /// default is "en-US"
    pub preferred_locale: Option<String>,

    /// Whether the guild has the boost progress bar enabled
    pub premium_progress_bar_enabled: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Schema for the [crate::types::Guild::modify_mfa_level] route
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#modify-guild-mfa-level>
pub(crate) struct GuildModifyMFALevelSchema {
    pub level: MFALevel,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#get-user-guilds>
pub struct GetUserGuildsSchema {
    /// Get guilds before this guild id
    pub before: Option<Snowflake>,
    /// Get guilds after this guild id
    pub after: Option<Snowflake>,
    /// Max number of guilds to return (1 - 200)
    pub limit: Option<u8>,
    /// Whether to include approximate member and presence counts (false by default)
    pub with_counts: Option<bool>,
}

impl GetUserGuildsSchema {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {
        let mut query = Vec::with_capacity(4);

        if let Some(before) = self.before {
            query.push(("before", before.to_string()));
        }

        if let Some(after) = self.after {
            query.push(("after", after.to_string()));
        }

        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(with_counts) = self.with_counts {
            query.push(("with_counts", with_counts.to_string()));
        }

        query
    }
}

impl std::default::Default for GetUserGuildsSchema {
    fn default() -> Self {
        Self {
            before: Default::default(),
            after: Default::default(),
            limit: Some(200),
            with_counts: Some(false),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct GuildPreview {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub home_header: Option<String>,
    pub features: Vec<String>,
    pub emojis: Vec<Emoji>,
    pub stickers: Vec<Sticker>,
    pub approximate_member_count: u32,
    pub approximate_presence_count: u32,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// Schema for the [crate::types::Guild::get_members] route
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#get-guild-members>
pub struct GetGuildMembersSchema {
    /// Max number of members to return (1-1000, default 1)
    pub limit: Option<u16>,
    /// Get members after this member ID
    pub after: Option<Snowflake>,
}

impl GetGuildMembersSchema {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {
        let mut query = Vec::with_capacity(2);

        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(after) = self.after {
            query.push(("after", after.to_string()));
        }

        query
    }
}

impl Default for GetGuildMembersSchema {
    fn default() -> Self {
        Self {
            limit: Some(1),
            after: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// Schema for the [Guild::query_members](crate::types::Guild::query_members) route
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#query-guild-members>
pub struct QueryGuildMembersSchema {
    /// Query to match username(s) and nickname(s) against
    pub query: String,
    /// Max number of members to return (1-1000, default 1)
    pub limit: Option<u16>,
}

impl QueryGuildMembersSchema {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {
        let mut query = Vec::with_capacity(2);

        query.push(("query", self.query));

        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }

        query
    }
}

impl Default for QueryGuildMembersSchema {
    fn default() -> Self {
        Self {
            query: Default::default(),
            limit: Some(1),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, Hash)]
pub struct GuildGetMembersQuery {
    pub limit: Option<u16>,
    pub after: Option<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// Schema for the [Guild::modify_member](crate::types::Guild::modify_member) route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#modify-guild-member>
pub struct ModifyGuildMemberSchema {
    #[serde(rename = "nick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's nickname in the guild (1 - 32 characters)
    ///
    /// Requires the [MANAGE_NICKNAMES](crate::types::PermissionFlags::MANAGE_NICKNAMES) permission.
    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The IDs of roles assigned to this member
    ///
    /// Requires the [MANAGE_ROLES](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    pub roles: Option<Vec<Snowflake>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the member is server-muted in voice channels.
    ///
    /// Requires the [MUTE_MEMBERS](crate::types::PermissionFlags::MUTE_MEMBERS) permission.
    pub mute: Option<bool>,

    #[serde(rename = "deaf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the user is server-deafened in voice channels.
    ///
    /// Requires the [DEAFEN_MEMBERS](crate::types::PermissionFlags::DEAFEN_MEMBERS) permission.
    pub deafen: Option<bool>,

    #[serde(rename = "channel_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the voice channel the member is currently connected to.
    ///
    /// Requires the [MOVE_MEMBERS](crate::types::PermissionFlags::MOVE_MEMBERS) permission.
    pub connected_voice_channel_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// When the user's timeout will expire and they will be able to communicate in the guild again
    ///
    /// Up to 28 days in the future
    ///
    /// Requires the [MODERATE_MEMBERS](crate::types::PermissionFlags::MODERATE_MEMBERS) permission.
    pub communication_disabled_until: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's flags.
    ///
    /// Only [BYPASSES_VERIFICATION](GuildMemberFlags::BYPASSES_VERIFICATION) can be set.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission or all
    /// of ([MODERATE_MEMBERS](crate::types::PermissionFlags::MODERATE_MEMBERS), [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) and [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS)) permissions.
    pub flags: Option<GuildMemberFlags>,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// Represents the flags of a Guild Member.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#guild-member-flags>
    pub struct GuildMemberFlags: u64 {
        const DID_REJOIN = 1 << 0;
        const COMPLETED_ONBOARDING = 1 << 1;
        const BYPASSES_VERIFICATION = 1 << 2;
        const STARTED_ONBOARDING = 1 << 3;
        const GUEST = 1 << 3;
        const AUTOMOD_QUARANTINED_NAME = 1 << 7;
        const AUTOMOD_QUARANTINED_BIO = 1 << 8;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// Schema for the [Guild::modify_current_member](crate::types::Guild::modify_current_member) route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#modify-current-guild-member>
pub struct ModifyCurrentGuildMemberSchema {
    #[serde(rename = "nick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The nickname of the member (1-32 characters)
    ///
    /// Requires the [CHANGE_NICKNAME](crate::types::PermissionFlags::CHANGE_NICKNAME) permission.
    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild avatar.
    ///
    /// Can only be changed for premium users
    pub avatar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the member's avatar decoration
    pub avatar_decoration_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The SKU ID of the member's avatar decoration
    pub avatar_decoration_sku_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild pronouns (up to 40 characters)
    pub pronouns: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild bio.
    ///
    /// Can only be changed for premium users
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild banner.
    ///
    /// Can only be changed for premium users
    pub banner: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Schema for the
/// [Guild::modify_current_member_profile](crate::types::Guild::modify_current_member_profile) route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#modify-guild-member-profile>
pub struct ModifyGuildMemberProfileSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild pronouns (up to 40 characters)
    pub pronouns: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild bio (max 190 characters)
    ///
    /// Can only be changed for premium users
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild banner
    ///
    /// Can only be changed for premium users
    pub banner: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild accent color as a hexadecimal integer
    ///
    /// Can only be changed for premium users
    pub accent_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's two guild theme colors
    ///
    /// Can only be changed for premium users
    pub theme_colors: Option<ThemeColors>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild profile popout animation particle type
    pub popout_animation_particle_type: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild profile emoji ID
    pub emoji_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The member's guild profile effect ID
    pub profile_effect_id: Option<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, Hash)]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#get-guild-bans>
pub struct GetGuildBansQuery {
    /// Get bans before this user ID
    pub before: Option<Snowflake>,
    /// Get bans after this user ID
    pub after: Option<Snowflake>,
    /// Max number of bans to return (1-1000, default all or 1000)
    pub limit: Option<u16>,
}

impl GetGuildBansQuery {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {
        let mut query = Vec::with_capacity(3);

        if let Some(before) = self.before {
            query.push(("before", before.to_string()));
        }

        if let Some(after) = self.after {
            query.push(("after", after.to_string()));
        }

        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }

        query
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#search-guild-bans>
pub struct SearchGuildBansQuery {
    /// Query to match username(s) and display name(s) against
    ///
    /// (1 - 32 characters)
    pub query: String,

    /// Max number of bans to return (1-10, default 10)
    pub limit: Option<u16>,
}

impl SearchGuildBansQuery {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {
        let mut query = Vec::with_capacity(2);

        query.push(("query", self.query));

        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }

        query
    }
}

/// Max query length is 32 characters.
/// The limit argument is a number between 1 and 10, defaults to 10.
pub type GuildBansSearchQuery = GenericSearchQueryWithLimit;

/// Query is partial or full, username or nickname.
/// Limit argument is a number between 1 and 1000, defaults to 1.
pub type GuildMembersSearchQuery = GenericSearchQueryWithLimit;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
///  A guild's progress on meeting the requirements of joining discovery.
///
///  Certain guilds, such as those that are verified, are exempt from discovery requirements. These guilds will not have a fully populated discovery requirements object, and are guaranteed to receive only sufficient and sufficient_without_grace_period.
///
/// # Reference
/// See <https://docs.discord.sex/resources/discovery#discovery-requirements-object>
pub struct GuildDiscoveryRequirements {
    pub guild_id: Option<Snowflake>,
    pub safe_environment: Option<bool>,
    pub healthy: Option<bool>,
    pub health_score_pending: Option<bool>,
    pub size: Option<bool>,
    pub nsfw_properties: Option<GuildDiscoveryNsfwProperties>,
    pub protected: Option<bool>,
    pub sufficient: Option<bool>,
    pub sufficient_without_grace_period: Option<bool>,
    pub valid_rules_channel: Option<bool>,
    pub retention_healthy: Option<bool>,
    pub engagement_healthy: Option<bool>,
    pub age: Option<bool>,
    pub minimum_age: Option<u16>,
    pub health_score: Option<GuildDiscoveryHealthScore>,
    pub minimum_size: Option<u64>,
    pub grace_period_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// # Reference
/// See <https://docs.discord.sex/resources/discovery#discovery-nsfw-properties-structure>
pub struct GuildDiscoveryNsfwProperties {
    pub channels: Vec<Snowflake>,
    pub channel_banned_keywords: HashMap<Snowflake, Vec<String>>,
    pub name: Option<String>,
    pub name_banned_keywords: Vec<String>,
    pub description: Option<String>,
    pub description_banned_keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Activity metrics are recalculated weekly, as an 8-week rolling average. If they are not yet eligible to be calculated, all fields will be null.
///
/// # Reference
/// See <https://docs.discord.sex/resources/discovery#discovery-health-score-structure>
pub struct GuildDiscoveryHealthScore {
    pub avg_nonnew_communicators: u64,
    pub avg_nonnew_participators: u64,
    pub num_intentful_joiners: u64,
    pub perc_ret_w1_intentful: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/emoji#create-guild-emoji>
pub struct EmojiCreateSchema {
    pub name: Option<String>,
    /// # Reference:
    /// See <https://docs.discord.sex/reference#cdn-data>
    pub image: String,
    #[serde(default)]
    pub roles: Vec<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
///  See <https://docs.discord.sex/resources/emoji#modify-guild-emoji>
pub struct EmojiModifySchema {
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#get-guild-prune>
pub struct GuildPruneQuerySchema {
    pub days: u8,
    /// Only used on POST
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_prune_count: Option<bool>,
    #[serde(default)]
    pub include_roles: Vec<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
/// Parameters used to specify which guild members to prune.
///
/// Is itself a query parameter schema for the [Guild::get_prune](crate::types::Guild::get_prune) endpoint and
/// a part of [GuildPruneSchema].
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#query-string-params>
pub struct GuildPruneParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of inactive days to count prune for.
    ///
    /// (1 - 30, default is 7)
    pub days: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional roles that can be pruned
    pub include_roles: Option<Vec<Snowflake>>,
}

impl GuildPruneParameters {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {

        let mut query = Vec::with_capacity(1);

        if let Some(days) = self.days {
            query.push(("days", days.to_string()));
        }

        if let Some(include_roles) = self.include_roles {
            // FIXME: is discord happy with this?
            // how are arrays meant to be encoded?
            for role in include_roles {
                query.push(("include_roles", role.to_string()));
            }
        }

        query
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
/// Schema for the [Guild::prune](crate::types::Guild::prune) endpoint
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#query-string-params>
pub struct GuildPruneSchema {
    #[serde(flatten)]
    /// Parameters which set how to prune the guild
    pub parameters: GuildPruneParameters,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to wait for the prune to be completed before responding
    ///
    /// (true by default)
    pub compute_prune_count: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Return schema for the [Guild::get_prune](crate::types::Guild::get_prune) endpoint
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#response-body>
pub struct GetGuildPruneResult {
    pub pruned: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Return schema for the [Guild::prune](crate::types::Guild::prune) endpoint
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#get-guild-prune>
pub struct GuildPruneResult {
    /// `None` if `compute_prune_count` is `false`
    pub pruned: Option<usize>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/sticker#create-guild-sticker>
pub struct GuildCreateStickerSchema {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
    pub file_data: Vec<u8>,
    #[serde(skip)]
    pub sticker_format_type: StickerFormatType,
}

impl GuildCreateStickerSchema {
    #[cfg(feature = "poem")]
    pub async fn from_multipart(mut multipart: poem::web::Multipart) -> Result<Self, poem::Error> {
        let mut _self = GuildCreateStickerSchema::default();
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().ok_or(poem::Error::from_string(
                "All fields must be named",
                poem::http::StatusCode::BAD_REQUEST,
            ))?;
            match name {
                "name" => {
                    _self.name = field.text().await?;
                }
                "description" => {
                    _self.description = Some(field.text().await?);
                }
                "tags" => {
                    _self.tags = Some(field.text().await?);
                }
                "file_data" => {
                    if _self.name.is_empty() {
                        _self.name =
                            field
                                .file_name()
                                .map(String::from)
                                .ok_or(poem::Error::from_string(
                                    "File name must be set",
                                    poem::http::StatusCode::BAD_REQUEST,
                                ))?;
                    }
                    _self.sticker_format_type = StickerFormatType::from_mime(
                        field.content_type().ok_or(poem::Error::from_string(
                            "Content type must be set",
                            poem::http::StatusCode::BAD_REQUEST,
                        ))?,
                    )
                    .ok_or(poem::Error::from_string(
                        "Unknown sticker format",
                        poem::http::StatusCode::BAD_REQUEST,
                    ))?;
                    _self.file_data = field.bytes().await?;
                }
                _ => {}
            }
        }
        if _self.name.is_empty() || _self.file_data.is_empty() {
            return Err(poem::Error::from_string(
                "At least the name and file_data are required",
                poem::http::StatusCode::BAD_REQUEST,
            ));
        }

        Ok(_self)
    }

    // #[cfg(feature = "client")]
    pub fn to_multipart(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new()
            .text("name", self.name.clone())
            .part(
                "file_data",
                reqwest::multipart::Part::bytes(self.file_data.clone())
                    .mime_str(self.sticker_format_type.to_mime())
                    .unwrap(),
            );

        if let Some(description) = &self.description {
            form = form.text("description", description.to_owned());
        }

        if let Some(tags) = &self.tags {
            form = form.text("tags", tags.to_owned())
        }
        form
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/sticker#modify-guild-sticker>
pub struct GuildModifyStickerSchema {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#modify-guild-welcome-screen>
pub struct GuildModifyWelcomeScreenSchema {
    pub enabled: Option<bool>,
    pub description: Option<String>,
    /// Max of 5
    pub welcome_channels: Option<Vec<WelcomeScreenChannel>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference:
/// See <https://docs.discord.sex/resources/guild-template#create-guild-template>
pub struct GuildTemplateCreateSchema {
    /// Name of the template (1-100 characters)
    pub name: String,
    /// Description of the template (max 120 characters)
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Return type for the [Guild::search_members](crate::types::Guild::search_members) endpoint.
///
/// Possible values are:
/// - [SGMReturnNotIndexed] - if the guild has not yet been indexed
/// - [SGMReturnOk] - which returns the search results
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub enum SearchGuildMembersReturn {
    NotIndexed(SGMReturnNotIndexed),
    Ok(SGMReturnOk),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Return type for the [Guild::search_members](crate::types::Guild::search_members) endpoint.
///
/// This type is returned when the guild specified is not yet indexed.
///
/// You should retry the request after waiting the timeframe specified in `retry_after`.
///
/// If that is `0`, you should retry after a short delay.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMReturnNotIndexed {
    pub message: String,
    pub code: u32,
    pub documents_indexed: u32,
    /// Number of seconds you should wait until retrying
    pub retry_after: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Return type for the [Guild::search_members](crate::types::Guild::search_members) endpoint.
///
/// This type is returned when the guild specified is not yet indexed.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMReturnOk {
    /// The id of the guild searchedd
    pub guild_id: Snowflake,
    /// The resulting members
    pub members: Vec<SupplementalGuildMember>,
    /// The number of results returned
    pub page_result_count: u16,
    /// The number of results found
    pub total_result_count: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// JSON schema for the
/// [Guild::search_members](crate::types::Guild::search_members) endpoint.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SearchGuildMembersSchema {
    /// Max number of members to return
    ///
    /// 0 - 1000, 25 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,

    /// How to sort the returned array
    ///
    /// By default, this is [MemberSortType::JoinedAtDescending]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MemberSortType>,

    /// The filter criteria to match against members using OR logic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_query: Option<SearchGuildMembersFilter>,

    /// The filter criteria to match against members using AND logic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_query: Option<SearchGuildMembersFilter>,

    /// Get members before this criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<SGMPaginationFilter>,

    /// Get members after this criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<SGMPaginationFilter>,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Default,
    Debug,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// How a user joined a guild
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#join-source-type>
pub enum MemberSortType {
    #[default]
    /// Sort by when the user joined the guild, descending (newest members first) (default)
    JoinedAtDescending = 1,
    /// Sort by when the user joined the guild, ascending (oldest members first)
    JoinedAtAscending = 2,
    /// Sort by when the user's account was created, descending (newest accounts first)
    UserIdDescending = 3,
    /// Sort by when the user's account was created, ascending (oldest accounts first)
    UserIdAscending = 4,
}

impl TryFrom<u8> for MemberSortType {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::JoinedAtDescending),
            2 => Ok(Self::JoinedAtAscending),
            3 => Ok(Self::UserIdDescending),
            4 => Ok(Self::UserIdAscending),
            _ => Err(ChorusError::InvalidArguments {
                error: "Value is not a valid MemberSortType".to_string(),
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for MemberSortType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for MemberSortType {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::from(*self as u8);
        sqlx_pg_uint.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MemberSortType {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::decode(value)?;
        MemberSortType::try_from(sqlx_pg_uint.to_uint()).map_err(|e| e.into())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// Part of [SearchGuildMembersSchema], used to offset the array (for paging)
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMPaginationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "ts_milliseconds_option")]
    pub guild_joined_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Part of [SearchGuildMembersSchema]
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SearchGuildMembersFilter {
    /// Query to match user IDs against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<SGMSnowflakeQuery>,

    /// Query to match usernames against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<SGMStringQuery>,

    /// Query to match role IDs against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<SGMSnowflakeQuery>,

    /// Query to match the timestamp when the member joined the guild against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_joined_at: Option<SGMTimestampQuery>,

    /// Safety signals to match member against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_signals: Option<SGMSafetySignals>,

    /// Whether the member has not yet passed the guild's member verification requirements against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pending: Option<bool>,

    /// Whether the member previously left and rejoined the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_rejoin: Option<bool>,

    /// Query for how members joined the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_source_type: Option<SGMJoinSourceQuery>,

    /// Query for the invite code used to join the guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_invite_code: Option<SGMStringQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// Part of [SearchGuildMembersFilter]
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMSafetySignals {
    /// When the member's unusual activity flag will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unusual_dm_activity_until: Option<SGMTimestampQuery>,

    /// When the member's timeout will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<SGMTimestampQuery>,

    /// Whether unusual account activity was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unusual_account_activity: Option<bool>,

    /// Whether the member has been quarantined by an automod rule for their username, display name
    /// or nickname
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automod_quarantined_username: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Enum for the various types of queries in [SearchGuildMembersQuery]
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub enum SearchGuildMembersQuery {
    Snowflake(SGMSnowflakeQuery),
    String(SGMStringQuery),
    Timestamp(SGMTimestampQuery),
    JoinType(SGMJoinSourceQuery),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A possible type of [SearchGuildMembersQuery], used for querying with
/// a [Snowflake] type.
///
/// This type is used for querying user and role ids.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMSnowflakeQuery {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// The values to match against using OR logic
    pub or_query: Vec<Snowflake>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// The values to match against using AND logic
    ///
    /// Allowed only when matching for role IDs
    pub and_query: Vec<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The range of values to match against
    ///
    /// Allowed only when matching for user IDs
    pub range: Option<SGMSnowflakeQueryRange>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// Part of [SGMSnowflakeQuery]
///
/// Used to specify bounds (>=, <=) for [Snowflake] queries
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMSnowflakeQueryRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gte")]
    /// Inclusive lower bound (>=) value to match
    pub greater_than_or_equal: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lte")]
    /// Inclusive upper bound (<=) value to match
    pub less_than_or_equal: Option<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A possible type of [SearchGuildMembersQuery], used for querying with
/// a [String] type.
///
/// This type is used for querying usernames.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMStringQuery {
    /// The values to match against using OR logic
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub or_query: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// A possible type of [SearchGuildMembersQuery], used for querying with
/// a timestamp as a [chrono::DateTime] type.
///
/// This type is used for querying guild join times.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMTimestampQuery {
    /// The range of values to match against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SGMSnowflakeQueryRange>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// Part of [SGMTimestampQuery]
///
/// Used to specify bounds (>=, <=) for timestamp queries
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMTimestampQueryRange {
    #[serde(with = "ts_milliseconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gte")]
    /// Inclusive lower bound (>=) value to match
    pub greater_than_or_equal: Option<chrono::DateTime<Utc>>,

    #[serde(with = "ts_milliseconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lte")]
    /// Inclusive upper bound (<=) value to match
    pub less_than_or_equal: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A possible type of [SearchGuildMembersQuery], used for querying with
/// the [JoinSourceType].
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/guild#search-guild-members>
pub struct SGMJoinSourceQuery {
    /// The values to match against using OR logic
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub or_query: Vec<JoinSourceType>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// Schema for the [Guild::get_members_supplemental](crate::types::Guild::get_members_supplemental) route
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#get-guild-members-supplemental>
pub struct GetGuildMembersSupplementalSchema {
    /// The user IDs to fetch supplemental guild member information for (max 200)
    pub users: Vec<Snowflake>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// Schema for the [Guild::add_member](crate::types::Guild::add_member) endpoint.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#add-guild-member>
pub struct AddGuildMemberSchema {
    /// The OAuth2 access token granted with guilds.join to the bot's application for the user you want to add
    pub access_token: String,

    /// The guild-specific nickname to set for the member
    /// (1 - 32 characters)
    ///
    /// Requires the [MANAGE_NICKNAMES](crate::types::PermissionFlags::MANAGE_NICKNAMES) permission.
    #[serde(rename = "nick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The IDs of roles to assign to this member
    ///
    /// Requires the [MANAGE_ROLES](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Snowflake>,

    /// Whether to server-mute this new member
    ///
    /// Requires the [MUTE_MEMBERS](crate::types::PermissionFlags::MUTE_MEMBERS) permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    /// Whether to server-deafen this new member
    ///
    /// Requires the [DEAFEN_MEMBERS](crate::types::PermissionFlags::DEAFEN_MEMBERS) permission.
    #[serde(rename = "deaf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deafen: Option<bool>,

    /// Flags to set for this new member
    ///
    /// Only [BYPASSES_VERIFICATION](GuildMemberFlags::BYPASSES_VERIFICATION) can be set.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission or all
    /// of ([MODERATE_MEMBERS](crate::types::PermissionFlags::MODERATE_MEMBERS), [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) and [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS)) permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<GuildMemberFlags>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Return object for the [Guild::add_member](crate::types::Guild::add_member) route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#add-guild-member>
pub enum AddGuildMemberReturn {
    /// The request succeeded, and the user is now a member
    Joined(GuildMember),

    /// The user was already a member of the guild
    AlreadyAMember,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Return schema for the [Guild::get_widget_settings](crate::types::Guild::get_widget_settings) endpoint.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#guild-widget-settings-structure>
pub struct GuildWidgetSettings {
	/// Whether the widget is enabled
	pub enabled: bool,

	/// The channel ID that we widget will generate an invite to, if any
	pub channel_id: Option<Snowflake>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
/// Schema for the [Guild::modify_widget](crate::types::Guild::modify_widget) endpoint.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#json-params>
pub struct ModifyGuildWidgetSchema {

   #[serde(skip_serializing_if = "Option::is_none")]
	/// Whether the widget is enabled
	pub enabled: Option<bool>,

   #[serde(skip_serializing_if = "Option::is_none")]
	/// The channel ID that we widget will generate an invite to, if any
	///
	/// Note that the first `Option` represents whether we want to modify
	/// the field, and the second allows us to set it to `None` or `Some`
	pub channel_id: Option<Option<Snowflake>>
}
