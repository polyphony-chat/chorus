use serde::{Deserialize, Serialize};

use crate::types::{entities::PermissionOverwrite, Snowflake};

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
    pub parent_id: Option<Snowflake>,
    pub id: Option<Snowflake>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<String>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct ChannelModifySchema {
    pub name: Option<String>,
    pub channel_type: Option<u8>,
    pub topic: Option<String>,
    pub icon: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<Snowflake>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<String>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetChannelMessagesSchema {
    /// Between 1 and 100, defaults to 50.
    pub limit: Option<i32>,
    #[serde(flatten)]
    pub anchor: ChannelMessagesAnchor,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMessagesAnchor {
    Before(Snowflake),
    Around(Snowflake),
    After(Snowflake),
}

impl GetChannelMessagesSchema {
    pub fn before(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: ChannelMessagesAnchor::Before(anchor),
        }
    }

    pub fn around(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: ChannelMessagesAnchor::Around(anchor),
        }
    }

    pub fn after(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: ChannelMessagesAnchor::After(anchor),
        }
    }

    /// Must be between 1 and 100
    pub fn limit(self, limit: i32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}
