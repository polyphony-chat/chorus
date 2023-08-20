use serde::{Deserialize, Serialize};

use crate::types::entities::Channel;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create a Guild.
/// See: <https://docs.spacebar.chat/routes/#cmp--schemas-guildcreateschema>
pub struct GuildCreateSchema {
    pub name: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub guild_template_code: Option<String>,
    pub system_channel_id: Option<String>,
    pub rules_channel_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create a Guild Ban.
/// See: <https://discord-userdoccers.vercel.app/resources/guild#create-guild-ban>
pub struct GuildBanCreateSchema {
    pub delete_message_days: Option<u8>,
    pub delete_message_seconds: Option<u32>,
}
