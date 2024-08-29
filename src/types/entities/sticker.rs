// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{entities::User, utils::Snowflake, Shared};

use super::option_arc_rwlock_ptr_eq;

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

#[cfg(not(tarpaulin_include))]
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
            && option_arc_rwlock_ptr_eq(&self.user, &other.user)
            && self.sort_value == other.sort_value
    }
}

impl Sticker {
    pub fn tags(&self) -> Vec<String> {
        self.tags.as_ref().map_or(vec![], |s| {
            s.split(',').map(|tag| tag.trim().to_string()).collect()
        })
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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
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

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
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
        matches!(
            self,
            StickerFormatType::APNG | StickerFormatType::LOTTIE | StickerFormatType::GIF
        )
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
