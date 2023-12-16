use serde::{Deserialize, Serialize};

use crate::types::{ExplicitContentFilterLevel, MessageNotificationLevel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildDefaults {
    pub max_presences: u64,
    pub max_video_channel_users: u16,
    pub afk_timeout: u16,
    pub default_message_notifications: MessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
}

impl Default for GuildDefaults {
    fn default() -> Self {
        Self {
            max_presences: 250_000,
            max_video_channel_users: 200,
            afk_timeout: 300,
            default_message_notifications: MessageNotificationLevel::OnlyMentions,
            explicit_content_filter: ExplicitContentFilterLevel::Disabled,
        }
    }
}
