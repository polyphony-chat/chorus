// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::entities::Channel;
use crate::types::types::guild_configuration::GuildFeatures;
use crate::types::{
    Emoji, ExplicitContentFilterLevel, GenericSearchQueryWithLimit, MFALevel,
    MessageNotificationLevel, RoleObject, Snowflake, Sticker, StickerFormatType,
    SystemChannelFlags, VerificationLevel, WelcomeScreenChannel,
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
/// See: <https://discord-userdoccers.vercel.app/resources/guild#create-guild-ban>
pub struct GuildBanCreateSchema {
    /// Deprecated
    pub delete_message_days: Option<u8>,
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
/// Represents the schema which needs to be sent to create a Guild Ban.
/// See: <https://discord-userdoccers.vercel.app/resources/guild#create-guild-ban>
pub struct GuildBanBulkCreateSchema {
    pub user_ids: Vec<Snowflake>,
    pub delete_message_seconds: Option<u32>,
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
/// Schema for the [crate::types::Guild::search_members] route
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

        if let Some(query) = self.query {
            query.push(("query", query.to_string()));
        }

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
pub struct ModifyGuildMemberSchema {
    pub nick: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
    pub mute: Option<bool>,
    pub deaf: Option<bool>,
    pub channel_id: Option<Snowflake>,
    pub communication_disabled_until: Option<DateTime<Utc>>,
    pub flags: Option<GuildMemberFlags>,
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// Represents the flags of a Guild Member.
    ///
    /// # Reference:
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
pub struct ModifyCurrentGuildMemberSchema {
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ModifyGuildMemberProfileSchema {
    pub pronouns: Option<String>,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub accent_color: Option<String>,
    pub theme_colors: Option<Vec<i32>>,
    pub popout_animation_particle_type: Option<Snowflake>,
    pub emoji_id: Option<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, Hash)]
/// The limit argument is a number between 1 and 1000.
pub struct GuildBansQuery {
    pub before: Option<Snowflake>,
    pub after: Option<Snowflake>,
    pub limit: Option<u16>,
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
/// # Reference:
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
/// # Reference:
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
/// # Reference:
/// See <https://docs.discord.sex/resources/discovery#discovery-health-score-structure>
pub struct GuildDiscoveryHealthScore {
    pub avg_nonnew_communicators: u64,
    pub avg_nonnew_participators: u64,
    pub num_intentful_joiners: u64,
    pub perc_ret_w1_intentful: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference:
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
/// # Reference:
/// See <https://docs.discord.sex/resources/emoji#modify-guild-emoji>
pub struct EmojiModifySchema {
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference:
/// See <https://docs.discord.sex/resources/guild#get-guild-prune>
pub struct GuildPruneQuerySchema {
    pub days: u8,
    /// Only used on POST
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_prune_count: Option<bool>,
    #[serde(default)]
    pub include_roles: Vec<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// # Reference:
/// See <https://docs.discord.sex/resources/guild#get-guild-prune>
pub struct GuildPruneResult {
    /// Null if compute_prune_count is false
    pub pruned: Option<usize>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference:
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
/// # Reference:
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
/// # Reference:
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
