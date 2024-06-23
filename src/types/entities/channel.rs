// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Debug;

use crate::types::{
    PermissionFlags, Shared,
    entities::{GuildMember, User},
    utils::Snowflake,
};

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::{observe_option_vec, Composite, Updateable};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
/// Represents a guild or private channel
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/channel#channels-resource>
pub struct Channel {
    pub application_id: Option<Snowflake>,
    #[cfg(feature = "sqlx")]
    pub applied_tags: Option<sqlx::types::Json<Vec<String>>>,
    #[cfg(not(feature = "sqlx"))]
    pub applied_tags: Option<Vec<String>>,
    #[cfg(feature = "sqlx")]
    pub available_tags: Option<sqlx::types::Json<Vec<Tag>>>,
    #[cfg(not(feature = "sqlx"))]
    pub available_tags: Option<Vec<Tag>>,
    pub bitrate: Option<i32>,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_forum_layout: Option<i32>,
    #[cfg(feature = "sqlx")]
    pub default_reaction_emoji: Option<sqlx::types::Json<DefaultReaction>>,
    #[cfg(not(feature = "sqlx"))]
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_sort_order: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub flags: Option<i32>,
    pub guild_id: Option<Snowflake>,
    pub icon: Option<String>,
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    pub managed: Option<bool>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub member: Option<ThreadMember>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub member_count: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub message_count: Option<i32>,
    pub name: Option<String>,
    pub nsfw: Option<bool>,
    pub owner_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>,
    #[cfg(feature = "sqlx")]
    pub permission_overwrites: Option<sqlx::types::Json<Vec<PermissionOverwrite>>>,
    #[cfg(not(feature = "sqlx"))]
    #[cfg_attr(feature = "client", observe_option_vec)]
    pub permission_overwrites: Option<Vec<Shared<PermissionOverwrite>>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub permissions: Option<String>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    #[cfg_attr(feature = "client", observe_option_vec)]
    pub recipients: Option<Vec<Shared<User>>>,
    pub rtc_region: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub thread_metadata: Option<ThreadMetadata>,
    pub topic: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub total_message_sent: Option<i32>,
    pub user_limit: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.application_id == other.application_id
            && self.bitrate == other.bitrate
            && self.channel_type == other.channel_type
            && self.created_at == other.created_at
            && self.default_auto_archive_duration == other.default_auto_archive_duration
            && self.default_forum_layout == other.default_forum_layout
            && self.default_sort_order == other.default_sort_order
            && self.default_thread_rate_limit_per_user == other.default_thread_rate_limit_per_user
            && self.flags == other.flags
            && self.guild_id == other.guild_id
            && self.icon == other.icon
            && self.id == other.id
            && self.last_message_id == other.last_message_id
            && self.last_pin_timestamp == other.last_pin_timestamp
            && self.managed == other.managed
            && self.member_count == other.member_count
            && self.message_count == other.message_count
            && self.name == other.name
            && self.nsfw == other.nsfw
            && self.owner_id == other.owner_id
            && self.parent_id == other.parent_id
            && self.permissions == other.permissions
            && self.position == other.position
            && self.rate_limit_per_user == other.rate_limit_per_user
            && self.rtc_region == other.rtc_region
            && self.topic == other.topic
            && self.total_message_sent == other.total_message_sent
            && self.user_limit == other.user_limit
            && self.video_quality_mode == other.video_quality_mode
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// A tag that can be applied to a thread in a [ChannelType::GuildForum] or [ChannelType::GuildMedia] channel.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/channel#forum-tag-object>
pub struct Tag {
    pub id: Snowflake,
    /// The name of the tag (max 20 characters)
    pub name: String,
    /// Whether this tag can only be added to or removed from threads by members with the [MANAGE_THREADS](crate::types::PermissionFlags::MANAGE_THREADS) permission
    pub moderated: bool,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
pub struct PermissionOverwrite {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub overwrite_type: PermissionOverwriteType,
    #[serde(default)]
    pub allow: PermissionFlags,
    #[serde(default)]
    pub deny: PermissionFlags,
}


#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, PartialEq, Eq, PartialOrd)]
#[repr(u8)]
/// # Reference
pub enum PermissionOverwriteType {
    Role = 0,
    Member = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/channel#thread-metadata-object>
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i32,
    pub archive_timestamp: DateTime<Utc>,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<DateTime<Utc>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/channel#thread-member-object>
pub struct ThreadMember {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    pub join_timestamp: Option<DateTime<Utc>>,
    pub flags: Option<u64>,
    pub member: Option<Shared<GuildMember>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd)]
/// Specifies the emoji to use as the default way to react to a [ChannelType::GuildForum] or [ChannelType::GuildMedia] channel post.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/channel#default-reaction-object>
pub struct DefaultReaction {
    #[serde(default)]
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

#[derive(
    Default,
    Clone,
    Copy,
    Debug,
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u32)]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/channel#channel-type>
pub enum ChannelType {
    #[default]
    /// A text channel within a guild
    GuildText = 0,
    /// A private channel between two users
    Dm = 1,
    /// A voice channel within a guild
    GuildVoice = 2,
    /// A private channel between multiple users
    GroupDm = 3,
    /// An organizational category that contains up to 50 channels
    GuildCategory = 4,
    /// Similar to [GuildText](ChannelType::GuildText), a channel that users can follow and crosspost into their own guild
    GuildNews = 5,
    /// A channel in which game developers can sell their game on Discord
    ///
    /// # Note
    /// Deprecated.
    GuildStore = 6,
    // FIXME userdoccers says 7 is GuildLfg, is this a spacebar specific thing?
    Encrypted = 7,
    // FIXME userdoccers says 8 is LfgGuildDm, is this a spacebar specific thing?
    EncryptedThreads = 8,
    // FIXME userdoccers says 9 is ThreadAlpha, was this changed?
    Transactional = 9,
    /// A thread within a [GuildNews](ChannelType::GuildNews) channel
    GuildNewsThread = 10,
    /// A thread within a [GuildText](ChannelType::GuildText), [GuildForum](ChannelType::GuildForum), or [GuildMedia](ChannelType::GuildMedia) channel
    GuildPublicThread = 11,
    /// A thread within a [GuildText](ChannelType::GuildText) channel, that is only viewable by those invited and those with the [MANAGE_THREADS](crate::types::entities::PermissionFlags::MANAGE_THREADS) permission
    GuildPrivateThread = 12,
    /// A voice channel for hosting events with an audience in a guild
    GuildStageVoice = 13,
    /// The main channel in a hub containing the listed guilds
    Directory = 14,
    /// A channel that can only contain threads
    GuildForum = 15,
    /// A channel that can only contain threads in a gallery view
    GuildMedia = 16,
    // TODO: Couldn't find reference
    TicketTracker = 33,
    // TODO: Couldn't find reference
    Kanban = 34,
    // TODO: Couldn't find reference
    VoicelessWhiteboard = 35,
    // TODO: Couldn't find reference
    CustomStart = 64,
    // TODO: Couldn't find reference
    Unhandled = 255,
}


/// # Reference
/// See <https://docs.discord.sex/resources/message#followed-channel-object>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct FollowedChannel {
    pub channel_id: Snowflake,
    pub webhook_id: Snowflake
}
