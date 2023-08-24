use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::types::{entities::User, utils::Snowflake};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
    pub user: Option<Arc<RwLock<User>>>,
    pub sort_value: Option<u8>,
}

impl PartialEq for Sticker {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.pack_id == other.pack_id
            && self.name == other.name
            && self.description == other.description
            && self.tags == other.tags
            && self.asset == other.asset
            && self.sticker_type == other.sticker_type
            && self.format_type == other.format_type
            && self.available == other.available
            && self.guild_id == other.guild_id
            && self.sort_value == other.sort_value
    }
}

impl PartialOrd for Sticker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.pack_id.partial_cmp(&other.pack_id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.description.partial_cmp(&other.description) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.tags.partial_cmp(&other.tags) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.asset.partial_cmp(&other.asset) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.sticker_type.partial_cmp(&other.sticker_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.format_type.partial_cmp(&other.format_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.available.partial_cmp(&other.available) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.guild_id.partial_cmp(&other.guild_id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.sort_value.partial_cmp(&other.sort_value)
    }
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
