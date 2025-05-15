// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use crate::errors::ChorusError;
use crate::types::{
    entities::{GuildMember, User},
    utils::Snowflake,
    PermissionFlags, Shared,
};

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use crate::gateway::Updateable;
use crate::UInt64;

#[cfg(feature = "client")]
use chorus_macros::{observe_option_vec, Composite, Updateable};
use serde::de::{Error, Visitor};

#[cfg(feature = "sqlx")]
use sqlx::types::Json;

use super::{option_arc_rwlock_ptr_eq, option_vec_arc_rwlock_ptr_eq, Emoji};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
/// Represents a guild or private channel
///
/// # Reference
/// See <https://docs.discord.food/resources/channel#channels-resource>
pub struct Channel {
    pub application_id: Option<Snowflake>,
    #[cfg(not(feature = "sqlx"))]
    pub applied_tags: Option<Vec<Snowflake>>,
    #[cfg(feature = "sqlx")]
    pub applied_tags: Option<Json<Vec<Snowflake>>>,
    #[cfg(not(feature = "sqlx"))]
    pub available_tags: Option<Vec<Tag>>,
    #[cfg(feature = "sqlx")]
    pub available_tags: Option<Json<Vec<Tag>>>,
    pub bitrate: Option<i32>,
    #[serde(rename = "type")]
    #[cfg_attr(feature = "sqlx", sqlx(rename = "type"))]
    pub channel_type: ChannelType,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_forum_layout: Option<DefaultForumLayout>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_sort_order: Option<DefaultSortOrder>,
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

#[cfg(not(tarpaulin_include))]
#[allow(clippy::nonminimal_bool)]
impl PartialEq for Channel {
    fn eq(&self, other: &Self) -> bool {
        self.application_id == other.application_id
            && self.applied_tags == other.applied_tags
            && self.applied_tags == other.applied_tags
            && self.available_tags == other.available_tags
            && self.available_tags == other.available_tags
            && self.bitrate == other.bitrate
            && self.channel_type == other.channel_type
            && self.created_at == other.created_at
            && self.default_auto_archive_duration == other.default_auto_archive_duration
            && self.default_forum_layout == other.default_forum_layout
            && self.default_reaction_emoji == other.default_reaction_emoji
            && self.default_reaction_emoji == other.default_reaction_emoji
            && self.default_sort_order == other.default_sort_order
            && self.default_thread_rate_limit_per_user == other.default_thread_rate_limit_per_user
            && self.flags == other.flags
            && self.guild_id == other.guild_id
            && self.icon == other.icon
            && self.id == other.id
            && self.last_message_id == other.last_message_id
            && self.last_pin_timestamp == other.last_pin_timestamp
            && self.managed == other.managed
            && self.member == other.member
            && self.member_count == other.member_count
            && self.message_count == other.message_count
            && self.name == other.name
            && self.nsfw == other.nsfw
            && self.owner_id == other.owner_id
            && self.parent_id == other.parent_id
            && compare_permission_overwrites(
                &self.permission_overwrites,
                &other.permission_overwrites,
            )
            && self.permissions == other.permissions
            && self.position == other.position
            && self.rate_limit_per_user == other.rate_limit_per_user
            && option_vec_arc_rwlock_ptr_eq(&self.recipients, &other.recipients)
            && self.rtc_region == other.rtc_region
            && self.thread_metadata == other.thread_metadata
            && self.topic == other.topic
            && self.total_message_sent == other.total_message_sent
            && self.user_limit == other.user_limit
            && self.video_quality_mode == other.video_quality_mode
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(feature = "sqlx")]
fn compare_permission_overwrites(
    a: &Option<Json<Vec<PermissionOverwrite>>>,
    b: &Option<Json<Vec<PermissionOverwrite>>>,
) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => match (a.encode_to_string(), b.encode_to_string()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        },
        (None, None) => true,
        _ => false,
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(not(feature = "sqlx"))]
fn compare_permission_overwrites(
    a: &Option<Vec<Shared<PermissionOverwrite>>>,
    b: &Option<Vec<Shared<PermissionOverwrite>>>,
) -> bool {
    option_vec_arc_rwlock_ptr_eq(a, b)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A tag that can be applied to a thread in a [ChannelType::GuildForum] or [ChannelType::GuildMedia] channel.
///
/// # Reference
/// See <https://docs.discord.food/resources/channel#forum-tag-object>
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow, sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "interface_type"))]
pub struct Tag {
    pub id: Snowflake,
    /// The name of the tag (max 20 characters)
    pub name: String,
    /// Whether this tag can only be added to or removed from threads by members with the [MANAGE_THREADS](crate::types::PermissionFlags::MANAGE_THREADS) permission
    pub moderated: bool,

    #[serde(default)]
    pub emoji_id: Option<Snowflake>,

    #[serde(default)]
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

#[derive(Debug, Serialize_repr, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
/// # Reference
///
/// See <https://docs.discord.food/resources/channel#permission-overwrite-type>
pub enum PermissionOverwriteType {
    Role = 0,
    Member = 1,
}

impl From<u8> for PermissionOverwriteType {
    fn from(v: u8) -> Self {
        match v {
            0 => PermissionOverwriteType::Role,
            1 => PermissionOverwriteType::Member,
            _ => unreachable!(),
        }
    }
}

impl FromStr for PermissionOverwriteType {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "role" => Ok(PermissionOverwriteType::Role),
            "member" => Ok(PermissionOverwriteType::Member),
            _ => Err(Self::Err::custom("invalid permission overwrite type")),
        }
    }
}

struct PermissionOverwriteTypeVisitor;

impl<'de> Visitor<'de> for PermissionOverwriteTypeVisitor {
    type Value = PermissionOverwriteType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid permission overwrite type")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(PermissionOverwriteType::from(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u8(v as u8)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        PermissionOverwriteType::from_str(v).map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v.as_str())
    }
}

impl<'de> Deserialize<'de> for PermissionOverwriteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = deserializer.deserialize_any(PermissionOverwriteTypeVisitor)?;

        Ok(val)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
/// # Reference
/// See <https://docs.discord.food/resources/channel#thread-metadata-object>
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
/// See <https://docs.discord.food/resources/channel#thread-member-object>
pub struct ThreadMember {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    pub join_timestamp: Option<DateTime<Utc>>,
    pub flags: Option<UInt64>,
    pub member: Option<Shared<GuildMember>>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for ThreadMember {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.user_id == other.user_id
            && self.join_timestamp == other.join_timestamp
            && self.flags == other.flags
            && option_arc_rwlock_ptr_eq(&self.member, &other.member)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd)]
/// Specifies the emoji to use as the default way to react to a [ChannelType::GuildForum] or [ChannelType::GuildMedia] channel post.
///
/// # Reference
/// See <https://docs.discord.food/resources/channel#default-reaction-object>
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow, sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "interface_type"))]
pub struct DefaultReaction {
    #[serde(default)]
    pub emoji_id: Option<Snowflake>,

    #[serde(default)]
    pub emoji_name: Option<String>,
}

impl From<Emoji> for DefaultReaction {
    fn from(value: Emoji) -> Self {
        Self {
            emoji_id: Some(value.id),
            emoji_name: value.name,
        }
    }
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
#[repr(i32)]
/// # Reference
/// See <https://docs.discord.food/resources/channel#channel-type>
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
/// See <https://docs.discord.food/resources/message#followed-channel-object>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct FollowedChannel {
    pub channel_id: Snowflake,
    pub webhook_id: Snowflake,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Copy)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DefaultForumLayout {
    #[default]
    Default = 0,
    List = 1,
    Grid = 2,
}

impl TryFrom<u8> for DefaultForumLayout {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DefaultForumLayout::Default),
            1 => Ok(DefaultForumLayout::List),
            2 => Ok(DefaultForumLayout::Grid),
            _ => Err(ChorusError::InvalidArguments {
                error: "Value is not a valid DefaultForumLayout".to_string(),
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for DefaultForumLayout {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for DefaultForumLayout {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::from(*self as u8);
        sqlx_pg_uint.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for DefaultForumLayout {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::decode(value)?;
        DefaultForumLayout::try_from(sqlx_pg_uint.to_uint()).map_err(|e| e.into())
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Copy)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DefaultSortOrder {
    #[default]
    LatestActivity = 0,
    CreationTime = 1,
}

impl TryFrom<u8> for DefaultSortOrder {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DefaultSortOrder::LatestActivity),
            1 => Ok(DefaultSortOrder::CreationTime),
            _ => Err(ChorusError::InvalidArguments {
                error: "Value is not a valid DefaultSearchOrder".to_string(),
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for DefaultSortOrder {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for DefaultSortOrder {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::from(*self as u8);
        sqlx_pg_uint.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for DefaultSortOrder {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::decode(value)?;
        DefaultSortOrder::try_from(sqlx_pg_uint.to_uint()).map_err(|e| e.into())
    }
}
