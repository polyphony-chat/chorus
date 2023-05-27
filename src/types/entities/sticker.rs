use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{entities::User, utils::Snowflake};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Sticker {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: Option<String>,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormat,
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
    pub format_type: StickerFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(i32)]
pub enum StickerType {
    Standard = 1,
    Guild = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(i32)]
pub enum StickerFormat {
    Png = 1,
    Apng = 2,
    Lottie = 3,
    Gif = 4,
}
