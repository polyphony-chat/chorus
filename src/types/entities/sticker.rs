// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{entities::User, utils::Snowflake, Shared};

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
    pub tags: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub asset: Option<String>,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormatType,
    pub available: Option<bool>,
    pub guild_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub sort_value: Option<u8>,
}

impl std::hash::Hash for Sticker {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.pack_id.hash(state);
        self.name.hash(state);
        self.description.hash(state);
        self.tags.hash(state);
        self.asset.hash(state);
        self.sticker_type.hash(state);
        self.format_type.hash(state);
        self.available.hash(state);
        self.guild_id.hash(state);
        self.sort_value.hash(state);
    }
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

impl Sticker {
    pub fn tags(&self) -> Vec<String> {
        self.tags
            .as_ref()
            .map_or(vec![], |s|
                s.split(',')
                    .map(|tag| tag.trim().to_string())
                    .collect()
            )
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
    pub format_type: StickerFormatType,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
/// # Reference
/// See <https://docs.discord.sex/resources/sticker#sticker-types>
pub enum StickerType {
    /// An official sticker in a current or legacy purchasable pack
    Standard = 1,
    #[default]
    /// A sticker uploaded to a guild for the guild's members
    Guild = 2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.sex/resources/sticker#sticker-format-types>
pub enum StickerFormatType {
    #[default]
    /// A PNG image
    PNG = 1,
    /// An animated PNG image, using the APNG format - uses CDN
    APNG = 2,
    /// A lottie animation; requires the VERIFIED and/or PARTNERED guild feature - uses CDN
    LOTTIE = 3,
    /// An animated GIF image - does not use CDN
    GIF = 4,
}

impl StickerFormatType {
    pub fn is_animated(&self) -> bool {
        matches!(self, StickerFormatType::APNG | StickerFormatType::LOTTIE | StickerFormatType::GIF)
    }

    pub const fn to_mime(&self) -> &'static str {
        match self {
            StickerFormatType::PNG => "image/png",
            StickerFormatType::APNG => "image/apng",
            StickerFormatType::LOTTIE => "application/json",
            StickerFormatType::GIF => "image/gif",
        }
    }

    pub fn from_mime(mime: &str) -> Option<Self> {
        match mime {
            "image/png" => Some(StickerFormatType::PNG),
            "image/apng" => Some(StickerFormatType::APNG),
            "application/json" => Some(StickerFormatType::LOTTIE),
            "image/gif" => Some(StickerFormatType::GIF),
            _ => None,
        }
    }
}
