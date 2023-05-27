use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{Channel, Emoji, GuildTemplate, RoleObject, Sticker, User, VoiceState, Webhook},
    interfaces::WelcomeScreenObject,
    utils::Snowflake,
};

/// See https://discord.com/developers/docs/resources/guild
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub owner: bool, // True if requesting user is owner
    pub owner_id: Option<Snowflake>,
    pub permissions: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<u8>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<Snowflake>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub roles: Vec<RoleObject>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub emojis: Vec<Emoji>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub features: String, // TODO: Make this a 'simple-array'
    pub application_id: Option<String>,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: Option<u8>,
    pub rules_channel_id: Option<Snowflake>,
    pub rules_channel: Option<String>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: Option<u8>,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: Option<String>,
    pub public_updates_channel_id: Option<Snowflake>,
    pub max_video_channel_users: Option<u8>,
    pub max_stage_video_channel_users: Option<u8>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    #[cfg(feature = "sqlx")]
    pub welcome_screen: Option<sqlx::types::Json<WelcomeScreenObject>>,
    #[cfg(not(feature = "sqlx"))]
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub nsfw_level: u8,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: Option<bool>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub joined_at: String,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub bans: Option<Vec<GuildBan>>,
    pub primary_category_id: Option<Snowflake>,
    pub large: Option<bool>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub channels: Option<Vec<Channel>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub invites: Option<Vec<GuildInvite>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub voice_states: Option<Vec<VoiceState>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub webhooks: Option<Vec<Webhook>>,
    pub mfa_level: Option<u8>,
    pub region: Option<String>,
}

/// See https://docs.spacebar.chat/routes/#get-/guilds/-guild_id-/bans/-user-
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GuildBan {
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
    pub reason: Option<String>,
}

/// See https://docs.spacebar.chat/routes/#cmp--schemas-invite
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GuildInvite {
    pub code: String,
    pub temporary: Option<bool>,
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub guild_id: String,
    pub guild: Option<Guild>,
    pub channel_id: String,
    pub channel: Option<Channel>,
    pub inviter_id: Option<String>,
    pub inviter: Option<User>,
    pub target_user_id: Option<String>,
    pub target_user: Option<String>,
    pub target_user_type: Option<i32>,
    pub vanity_url: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct UnavailableGuild {
    id: String,
    unavailable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct GuildCreateResponse {
    pub id: String,
}
