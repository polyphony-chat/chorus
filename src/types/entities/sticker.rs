use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_option_number_from_string};

use crate::types::{entities::User, utils::Snowflake};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Sticker {
    #[serde(default)]
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
    pub guild_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<User>,
    pub sort_value: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickerItem {
    pub id: Snowflake,
    pub name: String,
    pub format_type: u8,
}
