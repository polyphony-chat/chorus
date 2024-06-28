// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::entities::Channel;
use crate::types::types::guild_configuration::GuildFeatures;
use crate::types::{Emoji, ExplicitContentFilterLevel, GenericSearchQueryWithLimit, MessageNotificationLevel, Snowflake, Sticker, StickerFormatType, SystemChannelFlags, VerificationLevel, WelcomeScreenChannel};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create a Guild.
/// See: <https://docs.spacebar.chat/routes/#cmp--schemas-guildcreateschema>
pub struct GuildCreateSchema {
    pub name: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub guild_template_code: Option<String>,
    pub system_channel_id: Option<String>,
    pub rules_channel_id: Option<String>,
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
    pub name: Option<String>,
    pub icon: Option<Vec<u8>>,
    pub banner: Option<Vec<u8>>,
    pub home_header: Option<Vec<u8>>,
    pub splash: Option<Vec<u8>>,
    pub discovery_splash: Option<Vec<u8>>,
    pub owner_id: Option<Snowflake>,
    pub description: Option<String>,
    /// Deprecated
    pub region: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<u16>,
    pub verification_level: Option<VerificationLevel>,
    pub default_message_notifications: Option<MessageNotificationLevel>,
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
    pub features: Option<Vec<GuildFeatures>>,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: Option<SystemChannelFlags>,
    /// If set to Some(1), will create a new #rules channel
    ///
    /// Reference: <https://docs.discord.sex/resources/guild#modify-guild>
    pub rules_channel_id: Option<Snowflake>,
    pub public_updates_channel_id: Option<Snowflake>,
    pub safety_alerts_channel_id: Option<Snowflake>,
    pub preferred_locale: Option<String>,
    pub premium_progress_bar_enabled: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GetUserGuildSchema {
    pub before: Option<Snowflake>,
    pub after: Option<Snowflake>,
    pub limit: Option<u8>,
    pub with_counts: Option<bool>,
}

impl GetUserGuildSchema {
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

impl std::default::Default for GetUserGuildSchema {
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GuildMemberSearchSchema {
    pub query: String,
    pub limit: Option<u16>,
}

impl Default for GuildMemberSearchSchema {
    fn default() -> Self {
        Self {
            query: Default::default(),
            limit: Some(1),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    pub roles: Vec<Snowflake>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference:
/// See <https://docs.discord.sex/resources/emoji#modify-guild-emoji>
pub struct EmojiModifySchema {
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>
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
    pub include_roles: Vec<Snowflake>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    pub sticker_format_type: StickerFormatType
}

impl GuildCreateStickerSchema {
    #[cfg(feature = "poem")]
    pub async fn from_multipart(mut multipart: poem::web::Multipart) -> Result<Self, poem::Error> {
        let mut _self = GuildCreateStickerSchema::default();
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().ok_or(poem::Error::from_string("All fields must be named", poem::http::StatusCode::BAD_REQUEST))?;
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
                        _self.name = field.file_name().map(String::from).ok_or(poem::Error::from_string("File name must be set", poem::http::StatusCode::BAD_REQUEST))?;
                    }
                    _self.sticker_format_type = StickerFormatType::from_mime(field.content_type().ok_or(poem::Error::from_string("Content type must be set", poem::http::StatusCode::BAD_REQUEST))?).ok_or(poem::Error::from_string("Unknown sticker format", poem::http::StatusCode::BAD_REQUEST))?;
                    _self.file_data = field.bytes().await?;
                }
                _ => {}
            }

        }
        if _self.name.is_empty() || _self.file_data.is_empty() {
            return Err(poem::Error::from_string("At least the name and file_data are required", poem::http::StatusCode::BAD_REQUEST));
        }

        Ok(_self)
    }

    // #[cfg(feature = "client")]
    pub fn to_multipart(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new()
            .text("name", self.name.clone())
            .part("file_data", reqwest::multipart::Part::bytes(self.file_data.clone()).mime_str(self.sticker_format_type.to_mime()).unwrap());

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
    pub tags: Option<String>
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
    pub description: Option<String>
}
