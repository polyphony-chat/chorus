use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::entities::Channel;
use crate::types::types::guild_configuration::GuildFeatures;
use crate::types::{
    Emoji, ExplicitContentFilterLevel, MessageNotificationLevel, Snowflake, Sticker,
    SystemChannelFlags, VerificationLevel,
};

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
    pub delete_message_days: Option<u8>,
    pub delete_message_seconds: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GuildModifySchema {
    pub name: Option<String>,
    pub icon: Option<Vec<u8>>,
    pub banner: Option<Vec<u8>>,
    pub home_header: Option<Vec<u8>>,
    pub splash: Option<Vec<u8>>,
    pub discovery_splash: Option<Vec<u8>>,
    pub owner_id: Option<Snowflake>,
    pub description: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<u16>,
    pub verification_level: Option<VerificationLevel>,
    pub default_message_notifications: Option<MessageNotificationLevel>,
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
    pub features: Option<Vec<GuildFeatures>>,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: Option<SystemChannelFlags>,
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

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
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
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
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
