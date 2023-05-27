use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{Channel, Emoji, GuildTemplate, RoleObject, Sticker, User, VoiceState, Webhook},
    interfaces::WelcomeScreenObject,
    utils::Snowflake,
};

/// See https://discord.com/developers/docs/resources/guild
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<User>,
    pub owner_id: Option<Snowflake>,
    pub permissions: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<u8>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<Snowflake>,
    pub widget_channel: Option<Channel>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    pub roles: Vec<RoleObject>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub application_id: Option<String>,
    pub system_channel_id: Option<Snowflake>,
    pub system_channel_flags: Option<u8>,
    pub rules_channel_id: Option<String>,
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
    pub public_updates_channel: Option<Channel>,
    pub max_video_channel_users: Option<u8>,
    pub max_stage_video_channel_users: Option<u8>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub member_count: Option<u64>,
    pub presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub nsfw_level: u8,
    pub nsfw: bool,
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: Option<bool>,
    pub joined_at: String,
    pub afk_channel: Option<Channel>,
    pub bans: Option<Vec<GuildBan>>,
    pub primary_category_id: Option<Snowflake>,
    pub large: Option<bool>,
    pub channels: Option<Vec<Channel>>,
    pub template_id: Option<Snowflake>,
    pub template: Option<GuildTemplate>,
    pub invites: Option<Vec<GuildInvite>>,
    pub voice_states: Option<Vec<VoiceState>>,
    pub webhooks: Option<Vec<Webhook>>,
    pub mfa_level: Option<u8>,
    pub region: Option<String>,
    pub unavailable: bool,
    pub parent: Option<String>,
}

/// See https://docs.spacebar.chat/routes/#get-/guilds/-guild_id-/bans/-user-
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildBan {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
    pub executor_id: Snowflake,
    pub reason: Option<String>,
}

/// See https://docs.spacebar.chat/routes/#cmp--schemas-invite
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnavailableGuild {
    id: String,
    unavailable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildCreateResponse {
    pub id: String,
}
