use serde::{Deserialize, Serialize};

use crate::types::entities::PermissionOverwrite;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreateSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: Option<u8>,
    pub topic: Option<String>,
    pub icon: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<String>,
    pub id: Option<String>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<String>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChannelModifySchema {
    name: Option<String>,
    channel_type: Option<u8>,
    topic: Option<String>,
    icon: Option<String>,
    bitrate: Option<i32>,
    user_limit: Option<i32>,
    rate_limit_per_user: Option<i32>,
    position: Option<i32>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    parent_id: Option<String>,
    id: Option<String>,
    nsfw: Option<bool>,
    rtc_region: Option<String>,
    default_auto_archive_duration: Option<i32>,
    default_reaction_emoji: Option<String>,
    flags: Option<i32>,
    default_thread_rate_limit_per_user: Option<i32>,
    video_quality_mode: Option<i32>,
}
