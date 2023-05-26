use serde::{Deserialize, Serialize};

use crate::{entities::User, utils::Snowflake};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sticker {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: Option<String>,
    #[serde(rename = "type")]
    pub sticker_type: u8,
    pub format_type: u8,
    pub available: Option<bool>,
    pub guild_id: Option<u64>,
    pub user: Option<User>,
    pub sort_value: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerItem {
    pub id: Snowflake,
    pub name: String,
    pub format_type: u8,
}
