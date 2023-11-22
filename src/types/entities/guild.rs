use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::types::guild_configuration::GuildFeaturesList;
use crate::types::{
    entities::{Channel, Emoji, RoleObject, Sticker, User, VoiceState, Webhook},
    interfaces::WelcomeScreenObject,
    utils::Snowflake,
};

use super::PublicUser;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::{observe_option_vec, observe_vec, Composite, Updateable};

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

/// See <https://discord.com/developers/docs/resources/guild>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Guild {
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub application_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub approximate_member_count: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub approximate_presence_count: Option<i32>,
    pub banner: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub bans: Option<Vec<GuildBan>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_option_vec)]
    pub channels: Option<Vec<Arc<RwLock<Channel>>>>,
    pub default_message_notifications: Option<MessageNotificationLevel>,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_vec)]
    #[serde(default)]
    pub emojis: Vec<Arc<RwLock<Emoji>>>,
    pub explicit_content_filter: Option<i32>,
    //#[cfg_attr(feature = "sqlx", sqlx(try_from = "String"))]
    pub features: Option<GuildFeaturesList>,
    pub icon: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub icon_hash: Option<String>,
    pub id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub invites: Option<Vec<GuildInvite>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub max_members: Option<i32>,
    pub max_presences: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub max_stage_video_channel_users: Option<i32>,
    pub max_video_channel_users: Option<i32>,
    pub mfa_level: Option<MFALevel>,
    pub name: Option<String>,
    pub nsfw_level: Option<NSFWLevel>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub owner: Option<bool>,
    // True if requesting user is owner
    pub owner_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub permissions: Option<String>,
    pub preferred_locale: Option<String>,
    pub premium_progress_bar_enabled: Option<bool>,
    pub premium_subscription_count: Option<i32>,
    pub premium_tier: Option<PremiumTier>,
    pub primary_category_id: Option<Snowflake>,
    pub public_updates_channel_id: Option<Snowflake>,
    pub region: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_option_vec)]
    pub roles: Option<Vec<Arc<RwLock<RoleObject>>>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub rules_channel: Option<String>,
    pub rules_channel_id: Option<Snowflake>,
    pub splash: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub stickers: Option<Vec<Sticker>>,
    pub system_channel_flags: Option<u64>,
    pub system_channel_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub vanity_url_code: Option<String>,
    pub verification_level: Option<VerificationLevel>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_option_vec)]
    pub voice_states: Option<Vec<Arc<RwLock<VoiceState>>>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_option_vec)]
    pub webhooks: Option<Vec<Arc<RwLock<Webhook>>>>,
    #[cfg(feature = "sqlx")]
    pub welcome_screen: Option<sqlx::types::Json<WelcomeScreenObject>>,
    #[cfg(not(feature = "sqlx"))]
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub widget_channel_id: Option<Snowflake>,
    pub widget_enabled: Option<bool>,
}

impl std::hash::Hash for Guild {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.afk_channel_id.hash(state);
        self.afk_timeout.hash(state);
        self.application_id.hash(state);
        self.approximate_member_count.hash(state);
        self.approximate_presence_count.hash(state);
        self.banner.hash(state);
        self.bans.hash(state);
        self.default_message_notifications.hash(state);
        self.description.hash(state);
        self.discovery_splash.hash(state);
        self.explicit_content_filter.hash(state);
        self.features.hash(state);
        self.icon.hash(state);
        self.icon_hash.hash(state);
        self.id.hash(state);
        self.invites.hash(state);
        self.joined_at.hash(state);
        self.large.hash(state);
        self.max_members.hash(state);
        self.max_presences.hash(state);
        self.max_stage_video_channel_users.hash(state);
        self.max_video_channel_users.hash(state);
        self.mfa_level.hash(state);
        self.name.hash(state);
        self.nsfw_level.hash(state);
        self.owner.hash(state);
        self.owner_id.hash(state);
        self.permissions.hash(state);
        self.preferred_locale.hash(state);
        self.premium_progress_bar_enabled.hash(state);
        self.premium_subscription_count.hash(state);
        self.premium_tier.hash(state);
        self.primary_category_id.hash(state);
        self.public_updates_channel_id.hash(state);
        self.region.hash(state);
        self.rules_channel.hash(state);
        self.rules_channel_id.hash(state);
        self.splash.hash(state);
        self.stickers.hash(state);
        self.system_channel_flags.hash(state);
        self.system_channel_id.hash(state);
        self.vanity_url_code.hash(state);
        self.verification_level.hash(state);
        self.welcome_screen.hash(state);
        self.welcome_screen.hash(state);
        self.widget_channel_id.hash(state);
        self.widget_enabled.hash(state);
    }
}

impl std::cmp::PartialEq for Guild {
    fn eq(&self, other: &Self) -> bool {
        self.afk_channel_id == other.afk_channel_id
            && self.afk_timeout == other.afk_timeout
            && self.application_id == other.application_id
            && self.approximate_member_count == other.approximate_member_count
            && self.approximate_presence_count == other.approximate_presence_count
            && self.banner == other.banner
            && self.bans == other.bans
            && self.default_message_notifications == other.default_message_notifications
            && self.description == other.description
            && self.discovery_splash == other.discovery_splash
            && self.explicit_content_filter == other.explicit_content_filter
            && self.features == other.features
            && self.icon == other.icon
            && self.icon_hash == other.icon_hash
            && self.id == other.id
            && self.joined_at == other.joined_at
            && self.large == other.large
            && self.max_members == other.max_members
            && self.max_presences == other.max_presences
            && self.max_stage_video_channel_users == other.max_stage_video_channel_users
            && self.max_video_channel_users == other.max_video_channel_users
            && self.mfa_level == other.mfa_level
            && self.name == other.name
            && self.nsfw_level == other.nsfw_level
            && self.owner == other.owner
            && self.owner_id == other.owner_id
            && self.permissions == other.permissions
            && self.preferred_locale == other.preferred_locale
            && self.premium_progress_bar_enabled == other.premium_progress_bar_enabled
            && self.premium_subscription_count == other.premium_subscription_count
            && self.premium_tier == other.premium_tier
            && self.primary_category_id == other.primary_category_id
            && self.public_updates_channel_id == other.public_updates_channel_id
            && self.region == other.region
            && self.rules_channel == other.rules_channel
            && self.rules_channel_id == other.rules_channel_id
            && self.splash == other.splash
            && self.stickers == other.stickers
            && self.system_channel_flags == other.system_channel_flags
            && self.system_channel_id == other.system_channel_id
            && self.vanity_url_code == other.vanity_url_code
            && self.verification_level == other.verification_level
            && self.welcome_screen == other.welcome_screen
            && self.welcome_screen == other.welcome_screen
            && self.widget_channel_id == other.widget_channel_id
            && self.widget_enabled == other.widget_enabled
    }
}

impl std::cmp::Eq for Guild {}

/// See <https://docs.spacebar.chat/routes/#get-/guilds/-guild_id-/bans/-user->
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GuildBan {
    pub user: PublicUser,
    pub reason: Option<String>,
}

/// See <https://docs.spacebar.chat/routes/#cmp--schemas-invite>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GuildInvite {
    pub code: String,
    pub temporary: Option<bool>,
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub guild_id: Snowflake,
    pub guild: Option<Arc<RwLock<Guild>>>,
    pub channel_id: Snowflake,
    pub channel: Option<Arc<RwLock<Channel>>>,
    pub inviter_id: Option<Snowflake>,
    pub inviter: Option<Arc<RwLock<User>>>,
    pub target_user_id: Option<Snowflake>,
    pub target_user: Option<String>,
    pub target_user_type: Option<i32>,
    pub vanity_url: Option<bool>,
}

impl std::hash::Hash for GuildInvite {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        self.temporary.hash(state);
        self.uses.hash(state);
        self.max_uses.hash(state);
        self.max_age.hash(state);
        self.created_at.hash(state);
        self.expires_at.hash(state);
        self.guild_id.hash(state);
        self.channel_id.hash(state);
        self.inviter_id.hash(state);
        self.target_user_id.hash(state);
        self.target_user.hash(state);
        self.target_user_type.hash(state);
        self.vanity_url.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Hash)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    pub unavailable: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct GuildCreateResponse {
    pub id: Snowflake,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object>
pub struct GuildScheduledEvent {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Option<Snowflake>,
    pub creator_id: Option<Snowflake>,
    pub name: String,
    pub description: String,
    pub scheduled_start_time: DateTime<Utc>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    pub status: GuildScheduledEventStatus,
    pub entity_type: GuildScheduledEventEntityType,
    pub entity_id: Option<Snowflake>,
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,
    pub creator: Option<Arc<RwLock<User>>>,
    pub user_count: Option<u64>,
    pub image: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level>
pub enum GuildScheduledEventPrivacyLevel {
    #[default]
    GuildOnly = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status>
pub enum GuildScheduledEventStatus {
    #[default]
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types>
pub enum GuildScheduledEventEntityType {
    #[default]
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata>
pub struct GuildScheduledEventEntityMetadata {
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct VoiceRegion {
    id: String,
    name: String,
    optimal: bool,
    deprecated: bool,
    custom: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#message-notification-level>
pub enum MessageNotificationLevel {
    #[default]
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#explicit-content-filter-level>
pub enum ExplicitContentFilterLevel {
    #[default]
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#verification-level>
pub enum VerificationLevel {
    #[default]
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#verification-level>
pub enum MFALevel {
    #[default]
    None = 0,
    Elevated = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#verification-level>
pub enum NSFWLevel {
    #[default]
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#verification-level>
pub enum PremiumTier {
    #[default]
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#system-channel-flags>
    pub struct SystemChannelFlags: u64 {
        /// Indicates if an app uses the Auto Moderation API
        const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
        const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS_REPLIES = 1 << 5;
    }
}
