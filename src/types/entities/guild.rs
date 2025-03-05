// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;
use std::hash::Hash;

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::types::guild_configuration::GuildFeaturesList;
use crate::types::Shared;
use crate::types::{
    entities::{Channel, Emoji, RoleObject, Sticker, User, VoiceState, Webhook},
    interfaces::WelcomeScreenObject,
    utils::Snowflake,
};
use crate::UInt64;

use super::{option_arc_rwlock_ptr_eq, vec_arc_rwlock_ptr_eq, PublicUser, UserStatus};

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::{observe_vec, Composite, Updateable};

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
    #[serde(default)]
    pub bans: Vec<GuildBan>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_vec)]
    #[serde(default)]
    pub channels: Vec<Shared<Channel>>,
    pub default_message_notifications: Option<MessageNotificationLevel>,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_vec)]
    #[serde(default)]
    pub emojis: Vec<Shared<Emoji>>,
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,
    //#[cfg_attr(feature = "sqlx", sqlx(try_from = "String"))]
    #[serde(default)]
    pub features: GuildFeaturesList,
    pub icon: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub icon_hash: Option<String>,
    pub id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub invites: Vec<GuildInvite>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub joined_at: Option<DateTime<Utc>>,
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
    #[cfg_attr(feature = "client", observe_vec)]
    #[serde(default)]
    pub roles: Vec<Shared<RoleObject>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub rules_channel: Option<String>,
    pub rules_channel_id: Option<Snowflake>,
    pub splash: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[serde(default)]
    pub stickers: Vec<Sticker>,
    pub system_channel_flags: Option<SystemChannelFlags>,
    pub system_channel_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub vanity_url_code: Option<String>,
    pub verification_level: Option<VerificationLevel>,
    #[serde(default)]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_vec)]
    pub voice_states: Vec<Shared<VoiceState>>,
    #[serde(default)]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_vec)]
    pub webhooks: Vec<Shared<Webhook>>,
    #[cfg(feature = "sqlx")]
    pub welcome_screen: sqlx::types::Json<Option<WelcomeScreenObject>>,
    #[cfg(not(feature = "sqlx"))]
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub widget_channel_id: Option<Snowflake>,
    pub widget_enabled: Option<bool>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for Guild {
    fn eq(&self, other: &Self) -> bool {
        self.afk_channel_id == other.afk_channel_id
            && self.afk_timeout == other.afk_timeout
            && self.application_id == other.application_id
            && self.approximate_member_count == other.approximate_member_count
            && self.approximate_presence_count == other.approximate_presence_count
            && self.banner == other.banner
            && self.bans == other.bans
            && vec_arc_rwlock_ptr_eq(&self.channels, &other.channels)
            && self.default_message_notifications == other.default_message_notifications
            && self.description == other.description
            && self.discovery_splash == other.discovery_splash
            && vec_arc_rwlock_ptr_eq(&self.emojis, &other.emojis)
            && self.explicit_content_filter == other.explicit_content_filter
            && self.features == other.features
            && self.icon == other.icon
            && self.icon_hash == other.icon_hash
            && self.id == other.id
            && self.invites == other.invites
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
            && vec_arc_rwlock_ptr_eq(&self.roles, &other.roles)
            && self.rules_channel == other.rules_channel
            && self.rules_channel_id == other.rules_channel_id
            && self.splash == other.splash
            && self.stickers == other.stickers
            && self.system_channel_flags == other.system_channel_flags
            && self.system_channel_id == other.system_channel_id
            && self.vanity_url_code == other.vanity_url_code
            && self.verification_level == other.verification_level
            && vec_arc_rwlock_ptr_eq(&self.voice_states, &other.voice_states)
            && vec_arc_rwlock_ptr_eq(&self.webhooks, &other.webhooks)
            && self.welcome_screen == other.welcome_screen
            && self.welcome_screen == other.welcome_screen
            && self.widget_channel_id == other.widget_channel_id
            && self.widget_enabled == other.widget_enabled
    }
}

/// See <https://docs.spacebar.chat/routes/#get-/guilds/-guild_id-/bans/-user->
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GuildBan {
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
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
    pub guild: Option<Shared<Guild>>,
    pub channel_id: Snowflake,
    pub channel: Option<Shared<Channel>>,
    pub inviter_id: Option<Snowflake>,
    pub inviter: Option<Shared<User>>,
    pub target_user_id: Option<Snowflake>,
    pub target_user: Option<String>,
    pub target_user_type: Option<i32>,
    pub vanity_url: Option<bool>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for GuildInvite {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.temporary == other.temporary
            && self.uses == other.uses
            && self.max_uses == other.max_uses
            && self.max_age == other.max_age
            && self.created_at == other.created_at
            && self.expires_at == other.expires_at
            && self.guild_id == other.guild_id
            && option_arc_rwlock_ptr_eq(&self.guild, &other.guild)
            && self.channel_id == other.channel_id
            && option_arc_rwlock_ptr_eq(&self.channel, &other.channel)
            && self.inviter_id == other.inviter_id
            && option_arc_rwlock_ptr_eq(&self.inviter, &other.inviter)
            && self.target_user_id == other.target_user_id
            && self.target_user == other.target_user
            && self.target_user_type == other.target_user_type
            && self.vanity_url == other.vanity_url
    }
}

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Copy,
)]
pub struct UnavailableGuild {
    pub id: Snowflake,
    pub unavailable: Option<bool>,
    pub geo_restricted: Option<bool>,
}

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy,
)]
pub struct GuildCreateResponse {
    pub id: Snowflake,
}

/// An embeddable widget for a guild.
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#guild-widget-object>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GuildWidget {
    /// The ID of the guild the widget is for
    pub id: Snowflake,

    /// The name of the guild the widget is for
    pub name: String,

    /// The invite URL for the guild's widget channel, if any
    pub instant_invite: Option<String>,

    /// Approximate count of non-offline members in the guild
    pub presence_count: usize,

    /// The public voice and stage channels in the guild
    pub channels: Vec<GuildWidgetChannel>,

    /// The non-offline guild members (max 100)
    pub members: Vec<GuildWidgetMember>,
}

/// A channel, as provided in [GuildWidget]
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#guild-widget-channel-structure>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GuildWidgetChannel {
    /// The ID of the channel
    pub id: Snowflake,

    /// The name of the channel (1 - 100 characters)
    pub name: String,

    /// The sorting position of the channel
    pub position: i32,
}

/// A guild member, as provided in [GuildWidget]
///
/// Due to privacy concerns, `id`, `discriminator` and `avatar` are anonymized.
///
/// `id` is replaced with an incrementing integer, `discriminator` is always `0000`,
/// and `avatar` is always `null` (replaced with an encrypted `avatar_url` field).
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#guild-widget-member-structure>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GuildWidgetMember {
    /// The incrementing ID of the member
    pub id: Snowflake,

    /// The display name or censored username of the member
    pub username: String,

    pub avatar_url: String,

    /// The status of the user
    pub status: UserStatus,

    /// The primary activity the member is participating in
    pub activity: Option<GuildWidgetMemberActivity>,

    /// The ID of the voice or stage channel the member is in
    pub channel_id: Option<Snowflake>,

    /// Whether the member is server-deafened
    pub deaf: Option<bool>,

    /// Whether the member is server-muted
    pub mute: Option<bool>,

    /// Whether the member is locally deafened
    pub self_deaf: Option<bool>,

    /// Whether the member is locally muted
    pub self_mute: Option<bool>,

    /// Whether the member's permission to speak is denied
    pub suppress: Option<bool>,
}

/// An activity, as provided in [GuildWidgetMember]
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#guild-widget-member-activity-structure>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GuildWidgetMemberActivity {
    /// The name of the activity
    pub name: String,
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
    pub creator: Option<Shared<User>>,
    pub user_count: Option<UInt64>,
    pub image: Option<String>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for GuildScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.guild_id == other.guild_id
            && self.channel_id == other.channel_id
            && self.creator_id == other.creator_id
            && self.name == other.name
            && self.description == other.description
            && self.scheduled_start_time == other.scheduled_start_time
            && self.scheduled_end_time == other.scheduled_end_time
            && self.privacy_level == other.privacy_level
            && self.status == other.status
            && self.entity_type == other.entity_type
            && self.entity_id == other.entity_id
            && self.entity_metadata == other.entity_metadata
            && option_arc_rwlock_ptr_eq(&self.creator, &other.creator)
            && self.user_count == other.user_count
            && self.image == other.image
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Copy)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level>
pub enum GuildScheduledEventPrivacyLevel {
    #[default]
    GuildOnly = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Copy)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status>
pub enum GuildScheduledEventStatus {
    #[default]
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Copy,
    Hash,
)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
/// See <https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types>
pub enum GuildScheduledEventEntityType {
    #[default]
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#message-notification-level>
pub enum MessageNotificationLevel {
    #[default]
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord-userdoccers.vercel.app/resources/guild#explicit-content-filter-level>
pub enum ExplicitContentFilterLevel {
    #[default]
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#verification-level>
pub enum VerificationLevel {
    #[default]
    /// Unrestricted
    None = 0,

    /// Must have a verified email
    Low = 1,

    /// Must be a registered user for longer than 5 minutes
    Medium = 2,

    /// Must be a member of the server for longer than 10 minutes
    High = 3,

    /// Must have a verified phone number
    VeryHigh = 4,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://docs.discord.sex/resources/guild#mfa-level>
pub enum MFALevel {
    #[default]
    None = 0,
    Elevated = 1,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://docs.discord.sex/resources/guild#nsfw-level>
pub enum NSFWLevel {
    #[default]
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// Note: Maybe rename this to GuildPremiumTier?
/// **Guild** premium (Boosting) tier
///
/// See <https://docs.discord.sex/resources/guild#premium-tier>
pub enum PremiumTier {
    #[default]
    /// No server boost perks
    None = 0,
    /// Level 1 server boost perks
    Tier1 = 1,
    /// Level 2 server boost perks
    Tier2 = 2,
    /// Level 3 server boost perks
    Tier3 = 3,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/guild#member-verification-object>
pub struct GuildMemberVerification {
    /// When the verification object was last modified
    #[serde(default)]
    pub version: Option<DateTime<Utc>>,

    /// Questions for the applicants to answer (max 5)
    #[serde(default)]
    pub form_fields: Vec<GuildMemberVerificationFormField>,

    /// A description of what the guild is about; max 300 characters
    #[serde(default)]
    pub description: Option<String>,

    /// The guild this member verification is for
    ///
    /// This field is only included when returned by the [`Guild::get_member_verification`] endpoint with
    /// `with_guild set` to `true`.
    #[serde(default)]
    pub guild: Option<GuildMemberVerificationGuild>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
/// A question in [GuildMemberVerification].
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#member-verification-form-field-structure>
pub struct GuildMemberVerificationFormField {
    /// The type of question
    pub field_type: GuildMemberVerificationFormFieldType,

    /// The label for the form field (max 300 characters)
    pub label: String,

    /// Multiple choice answers (1-8, max 150 characters)
    #[serde(default)]
    pub choices: Option<Vec<String>>,

    /// The rules that the user must agree to (1-16, max 300 characters)
    #[serde(default)]
    pub values: Option<Vec<String>>,

    /// The correct response for this field.
    ///
    /// See [the type](GuildMemberVerificationResponse) for related docs on which variant to use.
    ///
    /// This field is not present when fetched from the [`Guild::get_member_verification`] endpoint.
    #[serde(default)]
    pub response: Option<GuildMemberVerificationResponse>,

    /// Whether this field is required for a successful application
    pub required: bool,

    /// The subtext of the form field
    #[serde(default)]
    pub description: Option<String>,

    /// Unknown (max 300 characters, max 10)
    #[serde(default)]
    pub automations: Option<Vec<String>>,

    /// Placeholder text for the field's response area
    #[serde(default)]
    pub placeholder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[serde(untagged)]
/// Types of form responses in [GuildMemberVerificationFormField].
///
/// For [GuildMemberVerificationFormFieldType::Terms], this should be [GuildMemberVerificationResponse::Boolean] (with a value of `true`)
///
/// For [GuildMemberVerificationFormFieldType::MultipleChoice], this should be [GuildMemberVerificationResponse::Index] (with the index of the selected choice)
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#member-verification-form-field-type>
pub enum GuildMemberVerificationResponse {
    String(String),
    Index(usize),
    Boolean(bool),
}

#[derive(
    Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Types of form questions in [GuildMemberVerificationFormField].
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#member-verification-form-field-type>
pub enum GuildMemberVerificationFormFieldType {
    /// User must agree to the guild rules
    #[default]
    Terms,

    /// User must respond with a short answer (max 150 characters)
    TextInput,

    /// User must respond with a paragraph (max 1000 characters)
    Paragraph,

    /// User must select one of the provided choices
    MultipleChoice,

    /// User must verify their email or phone number
    ///
    /// Deprecated
    Verification,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
/// A guild, as provided in [GuildMemberVerification].
///
/// # Reference
/// See <https://docs.discord.sex/resources/guild#member-verification-guild-structure>
pub struct GuildMemberVerificationGuild {
    /// The ID of the guild
    pub id: Snowflake,

    /// The name of the guild (2-100 characters)
    pub name: String,

    /// The guild's icon hash
    pub icon: Option<String>,

    /// The description for the guild (max 300 characters)
    pub description: Option<String>,

    /// The guild's splash image hash
    pub splash: Option<String>,

    /// The guild's discovery splash image hash
    pub discovery_splash: Option<String>,

    /// The guild's home header hash
    pub home_header: Option<String>,

    /// The [VerificationLevel] required to talk in the guild.
    pub verification_level: VerificationLevel,

    /// Enabled [GuildFeatures]
    pub features: GuildFeaturesList,

    /// Custom guild emojis
    pub emojis: Vec<Emoji>,

    /// Approximate number of total members in the guild
    pub approximate_member_count: usize,

    /// Approximate number of non-offline members in the guild
    pub approximate_presence_count: usize,
}
