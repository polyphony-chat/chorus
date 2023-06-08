use crate::types::{entities::Channel, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct GuildCreateSchema {
    pub name: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub guild_template_code: Option<String>,
    pub system_channel_id: Option<String>,
    pub rules_channel_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RoleCreateModifySchema {
    pub name: Option<String>,
    pub permissions: Option<String>,
    pub color: Option<u32>,
    pub hoist: Option<bool>,
    pub icon: Option<Vec<u8>>,
    pub unicode_emoji: Option<String>,
    pub mentionable: Option<bool>,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePositionUpdateSchema {
    pub id: Snowflake,
    pub position: i32,
}
