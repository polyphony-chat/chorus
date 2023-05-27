use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_number_from_string, deserialize_option_number_from_string, deserialize_string_from_number};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{
    entities::{GuildMember, User},
    utils::Snowflake,
};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Channel {
    pub id: Snowflake,
    pub created_at: chrono::DateTime<Utc>,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<Snowflake>,
    pub position: Option<i32>,
    #[cfg(feature = "sqlx")]
    pub permission_overwrites: Option<sqlx::types::Json<Vec<PermissionOverwrite>>>,
    #[cfg(not(feature = "sqlx"))]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>,
    pub application_id: Option<Snowflake>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<i32>,
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub thread_metadata: Option<ThreadMetadata>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    #[cfg(feature = "sqlx")]
    pub available_tags: Option<sqlx::types::Json<Vec<Tag>>>,
    #[cfg(not(feature = "sqlx"))]
    pub available_tags: Option<Vec<Tag>>,
    #[cfg(feature = "sqlx")]
    pub applied_tags: Option<sqlx::types::Json<Vec<String>>>,
    #[cfg(not(feature = "sqlx"))]
    pub applied_tags: Option<Vec<String>>,
    #[cfg(feature = "sqlx")]
    pub default_reaction_emoji: Option<sqlx::types::Json<DefaultReaction>>,
    #[cfg(not(feature = "sqlx"))]
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<i32>,
    pub default_forum_layout: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Tag {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub overwrite_type: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub allow: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub deny: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i32,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ThreadMember {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub join_timestamp: Option<String>,
    pub flags: Option<u64>,
    pub member: Option<GuildMember>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct DefaultReaction {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Default, Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i32)]
pub enum ChannelType {
    #[default]
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    Encrypted = 7,
    EncryptedThreads = 8,
    Transactional = 9,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
    Directory = 14,
    GuildForum = 15,
    TicketTracker = 33,
    Kanban = 34,
    VoicelessWhiteboard = 35,
    CustomStart = 64,
    Unhandled = 255,
}
