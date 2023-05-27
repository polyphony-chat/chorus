use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

use crate::types::utils::Snowflake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct UserData {
    pub valid_tokens_since: DateTime<Utc>,
    pub hash: Option<String>,
}

impl User {
    pub fn to_public_user(self) -> PublicUser {
        PublicUser::from(self)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub accent_color: Option<u8>,
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    /// This field comes as either a string or a number as a string
    /// So we need to account for that
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    flags: Option<i32>,
    pub premium_since: Option<DateTime<Utc>>,
    pub premium_type: u8,
    pub pronouns: Option<String>,
    pub public_flags: Option<u16>,
    pub banner: Option<String>,
    pub bio: String,
    pub theme_colors: Option<Vec<u8>>,
    pub phone: Option<String>,
    pub nsfw_allowed: bool,
    pub premium: bool,
    pub purchased_flags: i32,
    pub premium_usage_flags: Option<i32>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub accent_color: Option<u8>,
    pub banner: Option<String>,
    pub theme_colors: Option<Vec<u8>>,
    pub pronouns: Option<String>,
    pub bot: Option<bool>,
    pub bio: String,
    pub premium_type: u8,
    pub premium_since: Option<DateTime<Utc>>,
    pub public_flags: Option<u16>,
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            discriminator: value.discriminator,
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

const CUSTOM_USER_FLAG_OFFSET: u64 = 1 << 32;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    #[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
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
