use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{GuildMember, User},
    utils::Snowflake,
};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Channel {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<i32>,
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    pub available_tags: Option<Vec<Tag>>,
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<i32>,
    pub default_forum_layout: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub overwrite_type: u8,
    pub allow: String,
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
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
