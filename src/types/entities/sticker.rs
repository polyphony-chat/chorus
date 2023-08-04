use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::types::{entities::User, utils::Snowflake};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Represents a sticker that can be sent in messages.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/sticker#sticker-object>
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
    pub user: Option<Arc<Mutex<User>>>,
    pub sort_value: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A partial sticker object.
///
/// Represents the smallest amount of data required to render a sticker.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/sticker#sticker-item-object>
pub struct StickerItem {
    pub id: Snowflake,
    pub name: String,
    pub format_type: u8,
}
