// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::errors::ChorusError;
use crate::types::utils::Snowflake;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;
use std::array::TryFromSliceError;
use std::fmt::Debug;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use chorus_macros::{Composite, Updateable};

use super::Emoji;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct UserData {
    pub valid_tokens_since: DateTime<Utc>,
    pub hash: Option<String>,
}

impl User {
    pub fn into_public_user(self) -> PublicUser {
        PublicUser::from(self)
    }
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub accent_color: Option<u32>,
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    /// This field comes as either a string or a number as a string
    /// So we need to account for that
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub flags: Option<UserFlags>,
    pub premium_since: Option<DateTime<Utc>>,
    pub premium_type: Option<u8>,
    pub pronouns: Option<String>,
    pub public_flags: Option<UserFlags>,
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub theme_colors: Option<ThemeColors>,
    pub phone: Option<String>,
    pub nsfw_allowed: Option<bool>,
    pub premium: Option<bool>,
    pub purchased_flags: Option<i32>,
    pub premium_usage_flags: Option<i32>,
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub struct ThemeColors {
    #[serde(flatten)]
    inner: (u32, u32),
}

impl TryFrom<Vec<u8>> for ThemeColors {
    type Error = ChorusError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() % 4 != 0 || value.len() > 8 {
            return Err(ChorusError::InvalidArguments {
                error: "Value has incorrect length to be decodeable from Vec<u8>".to_string(),
            });
        }
        let first: Result<[u8; 4], TryFromSliceError> = value[0..3].try_into();
        let second: Result<[u8; 4], TryFromSliceError> = {
            if value.len() == 8 {
                value[0..3].try_into()
            } else {
                [0; 4][0..3].try_into()
            }
        };

        match (first, second) {
            (Ok(first), Ok(second)) => Ok(Self {
                inner: (u32::from_be_bytes(first), u32::from_be_bytes(second)),
            }),
            _ => Err(ChorusError::InvalidArguments {
                error: "ThemeColors cannot be built from this Vec<u8>".to_string(),
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
// TODO: Add tests for Encode and Decode.
impl<'q> sqlx::Encode<'q, sqlx::Any> for ThemeColors {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Any as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let mut vec_u8 = Vec::new();
        vec_u8.extend_from_slice(&self.inner.0.to_be_bytes());
        vec_u8.extend_from_slice(&self.inner.1.to_be_bytes());
        <Vec<u8> as sqlx::Encode<sqlx::Any>>::encode_by_ref(&vec_u8, buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'d> sqlx::Decode<'d, sqlx::Any> for ThemeColors {
    fn decode(
        value: <sqlx::Any as sqlx::Database>::ValueRef<'d>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value_vec = <Vec<u8> as sqlx::Decode<'d, sqlx::Any>>::decode(value)?;
        value_vec.try_into().map_err(|e: ChorusError| e.into())
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Any> for ThemeColors {
    fn type_info() -> <sqlx::Any as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Any>>::type_info()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct PublicUser {
    pub id: Snowflake,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub accent_color: Option<u32>,
    pub banner: Option<String>,
    pub theme_colors: Option<ThemeColors>,
    pub pronouns: Option<String>,
    pub bot: Option<bool>,
    pub bio: Option<String>,
    pub premium_type: Option<u8>,
    pub premium_since: Option<DateTime<Utc>>,
    pub public_flags: Option<UserFlags>,
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: Some(value.username),
            discriminator: Some(value.discriminator),
            avatar: value.avatar,
            accent_color: value.accent_color,
            banner: value.banner,
            theme_colors: value.theme_colors,
            pronouns: value.pronouns,
            bot: value.bot,
            bio: value.bio,
            premium_type: value.premium_type,
            premium_since: value.premium_since,
            public_flags: value.public_flags,
        }
    }
}

#[allow(dead_code)] // FIXME: Remove this when we actually use this
const CUSTOM_USER_FLAG_OFFSET: u64 = 1 << 32;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    pub struct UserFlags: u64 {
        const DISCORD_EMPLOYEE = 1 << 0;
        const PARTNERED_SERVER_OWNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUGHUNTER_LEVEL_1 =1 << 3;
        const MFA_SMS = 1 << 4;
        const PREMIUM_PROMO_DISMISSED = 1 << 5;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
        const TRUST_AND_SAFETY = 1 << 11;
        const SYSTEM = 1 << 12;
        const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;
        const BUGHUNTER_LEVEL_2 = 1 << 14;
        const UNDERAGE_DELETED = 1 << 15;
        const VERIFIED_BOT = 1 << 16;
        const EARLY_VERIFIED_BOT_DEVELOPER = 1 << 17;
        const CERTIFIED_MODERATOR = 1 << 18;
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserProfileMetadata {
    pub guild_id: Option<Snowflake>,
    pub pronouns: String,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub theme_colors: Option<ThemeColors>,
    pub popout_animation_particle_type: Option<Snowflake>,
    pub emoji: Option<Emoji>,
}
