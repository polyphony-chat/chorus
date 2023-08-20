use serde::{Deserialize, Serialize};

use crate::types::entities::Channel;
use crate::types::Snowflake;

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

#[derive(Debug, Deserialize, Serialize, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GuildModifySchema {
    pub name: Option<String>,
    pub icon: Option<Vec<u8>>,
    pub banner: Option<Vec<u8>>,
    pub home_header: Option<Vec<u8>>,
    pub splash: Option<Vec<u8>>,
    pub discovery_splash: Option<Vec<u8>>,
    pub owner_id: Option<Snowflake>,
    pub description: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: Option<u16>,
    pub verification_level: Option<u8>,
}
