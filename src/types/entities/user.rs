// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::errors::ChorusError;
use crate::types::utils::Snowflake;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;
use serde_repr::{Deserialize_repr, Serialize_repr};
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

use super::{Emoji, GuildMember, PublicConnection};

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
/// # Reference
/// See <https://docs.discord.sex/resources/user#user-structure>
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
    pub premium: Option<bool>,
    /// The type of premium (Nitro) a user has
    pub premium_type: Option<PremiumType>,
    pub premium_since: Option<DateTime<Utc>>,
    pub pronouns: Option<String>,
    pub public_flags: Option<UserFlags>,
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub theme_colors: Option<ThemeColors>,
    pub phone: Option<String>,
    pub nsfw_allowed: Option<bool>,
    pub purchased_flags: Option<i32>,
    pub premium_usage_flags: Option<i32>,
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
/// A user's theme colors, as u32s representing hex color codes
///
/// found in [UserProfileMetadata]
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
/// # Reference
/// See <https://docs.discord.sex/resources/user#partial-user-structure>
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
    /// The type of premium (Nitro) a user has
    pub premium_type: Option<PremiumType>,
    /// The date the user's premium (Nitro) subscribtion started
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
     /// # Reference
     /// See <https://docs.discord.sex/resources/user#user-flags>
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
          /// Note: deprecated by Discord
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

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// **User** premium (Nitro) type
///
/// See <https://docs.discord.sex/resources/user#premium-type>
pub enum PremiumType {
    #[default]
    /// No Nitro
    None = 0,
    /// Nitro Classic
    Tier1 = 1,
    /// Nitro
    Tier2 = 2,
    /// Nitro Basic
    Tier3 = 3,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// # Reference
/// See <https://docs.discord.sex/resources/user#profile-metadata-object>
pub struct UserProfileMetadata {
    /// The guild ID this profile applies to, if it is a guild profile.
    pub guild_id: Option<Snowflake>,
    /// The user's pronouns, up to 40 characters
    pub pronouns: String,
    /// The user's bio / description, up to 190 characters
    pub bio: Option<String>,
    /// The hash used to retrieve the user's banned from the CDN
    pub banner: Option<String>,
    /// Banner color encoded as an i32 representation of a hex color code
    pub accent_color: Option<i32>,
    /// See [ThemeColors]
    pub theme_colors: Option<ThemeColors>,
    pub popout_animation_particle_type: Option<Snowflake>,
    pub emoji: Option<Emoji>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// A user's publically facing profile
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#profile-metadata-object>
pub struct UserProfile {
    // TODO: add profile application object
    pub user: PublicUser,

    #[serde(rename = "user_profile")]
    pub profile_metadata: UserProfileMetadata,

    #[serde(default)]
    pub badges: Vec<ProfileBadge>,

    pub guild_member: Option<GuildMember>,

    #[serde(rename = "guild_member_profile")]
    pub guild_member_profile_metadata: Option<UserProfileMetadata>,

    #[serde(default)]
    pub guild_badges: Vec<ProfileBadge>,

    /// The user's legacy username#discriminator, if existing and shown
    pub legacy_username: Option<String>,

    #[serde(default)]
    pub mutual_guilds: Vec<MutualGuild>,

    #[serde(default)]
    pub mutual_friends: Vec<PublicUser>,

    pub mutual_friends_count: Option<u32>,

    pub connected_accounts: Vec<PublicConnection>,

    // TODO: Add application role connections!
    /// The type of premium (Nitro) a user has
    pub premium_type: Option<PremiumType>,
    /// The date the user's premium (Nitro) subscribtion started
    pub premium_since: Option<DateTime<Utc>>,
    /// The date the user's premium guild (Boosting) subscribtion started
    pub premium_guild_since: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Info about a badge on a user's profile ([UserProfile])
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#profile-badge-structure>
///
/// For a list of know badges, see <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
pub struct ProfileBadge {
    /// The badge's unique id, e.g. "staff", "partner", "premium", ...
    pub id: String,
    /// Description of what the badge represents, e.g. "Discord Staff"
    pub description: String,
    /// An icon hash, to get the badge's icon from the CDN
    pub icon: String,
    /// A link (potentially used for href) for the badge.
    ///
    /// e.g.:
    /// `"staff"` badge links to `"https://discord.com/company"`
    /// `"certified_moderator"` links to `"https://discord.com/safety"`
    pub link: Option<String>,
}

impl PartialEq for ProfileBadge {
    fn eq(&self, other: &Self) -> bool {
        // Note: does not include description, since it changes for some badges
        //
        // Think nitro "Subscriber since ...", "Server boosting since ..."
        self.id.eq(&other.id) && self.icon.eq(&other.icon) && self.link.eq(&other.link)
    }
}

impl ProfileBadge {
    /// Returns a badge representing the "staff" badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_staff() -> Self {
        Self {
            id: "staff".to_string(),
            description: "Discord Staff".to_string(),
            icon: "5e74e9b61934fc1f67c65515d1f7e60d".to_string(),
            link: Some("https://discord.com/company".to_string()),
        }
    }

    /// Returns a badge representing the partnered server owner badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_partner() -> Self {
        Self {
            id: "partner".to_string(),
            description: "Partnered Server Owner".to_string(),
            icon: "3f9748e53446a137a052f3454e2de41e".to_string(),
            link: Some("https://discord.com/partners".to_string()),
        }
    }

    /// Returns a badge representing the certified moderator badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_certified_moderator() -> Self {
        Self {
            id: "certified_moderator".to_string(),
            description: "Moderator Programs Alumni".to_string(),
            icon: "fee1624003e2fee35cb398e125dc479b".to_string(),
            link: Some("https://discord.com/safety".to_string()),
        }
    }

    /// Returns a badge representing the hypesquad events badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_hypesquad() -> Self {
        Self {
            id: "hypesquad".to_string(),
            description: "HypeSquad Events".to_string(),
            icon: "bf01d1073931f921909045f3a39fd264".to_string(),
            link: Some("https://support.discord.com/hc/en-us/articles/360035962891-Profile-Badges-101#h_01GM67K5EJ16ZHYZQ5MPRW3JT3".to_string()),
        }
    }

    /// Returns a badge representing the hypesquad bravery badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_hypesquad_bravery() -> Self {
        Self {
            id: "hypesquad_house_1".to_string(),
            description: "HypeSquad Bravery".to_string(),
            icon: "8a88d63823d8a71cd5e390baa45efa02".to_string(),
            link: Some("https://discord.com/settings/hypesquad-online".to_string()),
        }
    }

    /// Returns a badge representing the hypesquad brilliance badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_hypesquad_brilliance() -> Self {
        Self {
            id: "hypesquad_house_2".to_string(),
            description: "HypeSquad Brilliance".to_string(),
            icon: "011940fd013da3f7fb926e4a1cd2e618".to_string(),
            link: Some("https://discord.com/settings/hypesquad-online".to_string()),
        }
    }

    /// Returns a badge representing the hypesquad balance badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_hypesquad_balance() -> Self {
        Self {
            id: "hypesquad_house_3".to_string(),
            description: "HypeSquad Balance".to_string(),
            icon: "3aa41de486fa12454c3761e8e223442e".to_string(),
            link: Some("https://discord.com/settings/hypesquad-online".to_string()),
        }
    }

    /// Returns a badge representing the bug hunter level 1 badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_bug_hunter_1() -> Self {
        Self {
            id: "bug_hunter_level_1".to_string(),
            description: "Discord Bug Hunter".to_string(),
            icon: "2717692c7dca7289b35297368a940dd0".to_string(),
            link: Some(
                "https://support.discord.com/hc/en-us/articles/360046057772-Discord-Bugs"
                    .to_string(),
            ),
        }
    }

    /// Returns a badge representing the bug hunter level 2 badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_bug_hunter_2() -> Self {
        Self {
            id: "bug_hunter_level_2".to_string(),
            description: "Discord Bug Hunter".to_string(),
            icon: "848f79194d4be5ff5f81505cbd0ce1e6".to_string(),
            link: Some(
                "https://support.discord.com/hc/en-us/articles/360046057772-Discord-Bugs"
                    .to_string(),
            ),
        }
    }

    /// Returns a badge representing the active developer badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_active_developer() -> Self {
        Self {
            id: "active_developer".to_string(),
            description: "Active Developer".to_string(),
            icon: "6bdc42827a38498929a4920da12695d9".to_string(),
            link: Some(
                "https://support-dev.discord.com/hc/en-us/articles/10113997751447?ref=badge"
                    .to_string(),
            ),
        }
    }

    /// Returns a badge representing the early verified bot developer badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_early_verified_developer() -> Self {
        Self {
            id: "verified_developer".to_string(),
            description: "Early Verified Bot Developer".to_string(),
            icon: "6df5892e0f35b051f8b61eace34f4967".to_string(),
            link: None,
        }
    }

    /// Returns a badge representing the early supporter badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_early_supporter() -> Self {
        Self {
            id: "early_supporter".to_string(),
            description: "Early Supporter".to_string(),
            icon: "7060786766c9c840eb3019e725d2b358".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the nitro subscriber badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_nitro() -> Self {
        Self {
            id: "premium".to_string(),
            description: "Subscriber since 1 Jan 2015".to_string(),
            icon: "2ba85e8026a8614b640c2837bcdfe21b".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 1 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_1() -> Self {
        Self {
            id: "guild_booster_lvl1".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "51040c70d4f20a921ad6674ff86fc95c".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 2 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_2() -> Self {
        Self {
            id: "guild_booster_lvl2".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "0e4080d1d333bc7ad29ef6528b6f2fb7".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 3 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_3() -> Self {
        Self {
            id: "guild_booster_lvl3".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "72bed924410c304dbe3d00a6e593ff59".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 4 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_4() -> Self {
        Self {
            id: "guild_booster_lvl4".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "df199d2050d3ed4ebf84d64ae83989f8".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 5 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_5() -> Self {
        Self {
            id: "guild_booster_lvl5".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "996b3e870e8a22ce519b3a50e6bdd52f".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 6 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_6() -> Self {
        Self {
            id: "guild_booster_lvl6".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "991c9f39ee33d7537d9f408c3e53141e".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 7 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_7() -> Self {
        Self {
            id: "guild_booster_lvl7".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "cb3ae83c15e970e8f3d410bc62cb8b99".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 8 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_8() -> Self {
        Self {
            id: "guild_booster_lvl8".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "7142225d31238f6387d9f09efaa02759".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the level 9 server boosting badge on Discord.com
    ///
    /// Note: The description updates for the start date
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_server_boosting_9() -> Self {
        Self {
            id: "guild_booster_lvl9".to_string(),
            description: "Server boosting since 1 Jan 2015".to_string(),
            icon: "ec92202290b48d0879b7413d2dde3bab".to_string(),
            link: Some("https://discord.com/settings/premium".to_string()),
        }
    }

    /// Returns a badge representing the legacy username badge on Discord.com
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_legacy_username() -> Self {
        Self {
            id: "legacy_username".to_string(),
            description: "Originally known as USERNAME".to_string(),
            icon: "6de6d34650760ba5551a79732e98ed60".to_string(),
            link: None,
        }
    }

    /// Returns a badge representing the legacy username badge on Discord.com,
    /// with the provided username (which should already contain the #DISCRIM part)
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_legacy_username_with_username(username: String) -> Self {
        Self {
            id: "legacy_username".to_string(),
            description: format!("Originally known as {username}"),
            icon: "6de6d34650760ba5551a79732e98ed60".to_string(),
            link: None,
        }
    }

    /// Returns a badge representing the legacy username badge on Discord.com,
    /// with the provided username and discriminator
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_legacy_username_with_username_and_discriminator(
        username: String,
        discriminator: String,
    ) -> Self {
        Self {
            id: "legacy_username".to_string(),
            description: format!("Originally known as {username}#{discriminator}"),
            icon: "6de6d34650760ba5551a79732e98ed60".to_string(),
            link: None,
        }
    }

    /// Returns a badge representing the bot commands badge on Discord.com
    ///
    /// Note: This badge is only for bot accounts
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_bot_commands() -> Self {
        Self {
            id: "bot_commands".to_string(),
            description: "Supports Commands".to_string(),
            icon: "6f9e37f9029ff57aef81db857890005e".to_string(),
            link: Some(
                "https://discord.com/blog/welcome-to-the-new-era-of-discord-apps?ref=badge"
                    .to_string(),
            ),
        }
    }

    /// Returns a badge representing the bot automod badge on Discord.com
    ///
    /// Note: This badge is only for bot accounts
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_bot_automod() -> Self {
        Self {
            id: "automod".to_string(),
            description: "Uses AutoMod".to_string(),
            icon: "f2459b691ac7453ed6039bbcfaccbfcd".to_string(),
            link: None,
        }
    }

    /// Returns a badge representing the application guild subscription badge on Discord.com
    ///
    /// No idea where this badge could show up, but apparently it means a guild has an
    /// application's premium
    ///
    /// # Reference
    /// See <https://gist.github.com/XYZenix/c45156b7c883b5301c9028e39d71b479>
    pub fn discord_application_guild_subscription() -> Self {
        Self {
            id: "application_guild_subscription".to_string(),
            description: "This server has APPLICATION Premium".to_string(),
            icon: "d2010c413a8da2208b7e4f35bd8cd4ac".to_string(),
            link: None,
        }
    }
}

/// Structure which shows a mutual guild with a user
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#mutual-guild-structure>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MutualGuild {
    pub id: Snowflake,
    /// The user's nickname in the guild, if any
    pub nick: Option<String>,
}

/// Structure which is returned by the [crate::instance::ChorusUser::get_user_note] endpoint.
///
/// Note that [crate::instance::ChorusUser::get_user_notes] endpoint
/// returns a completely different structure;
// Specualation: this is probably how Discord stores notes internally
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#get-user-note>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct UserNote {
    /// Actual note contents; max 256 characters
    pub note: String,
    /// The ID of the user the note is on
    pub note_user_id: Snowflake,
    /// The ID of the user who created the note (always the current user)
    pub user_id: Snowflake,
}

/// Structure which defines an affinity the local user has with another user.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#user-affinity-structure>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct UserAffinity {
    /// The other user's id
    pub user_id: Snowflake,
    /// The affinity score
    pub affinity: f32,
}

/// Structure which defines an affinity the local user has with a guild.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#guild-affinity-structure>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct GuildAffinity {
    /// The guild's id
    pub guild_id: Snowflake,
    /// The affinity score
    pub affinity: f32,
}

/// Structure which defines the local user's premium perk usage.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#get-user-premium-usage>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PremiumUsage {
    /// Number of Nitro stickers the user has sent
    pub nitro_sticker_sends: PremiumUsageData,
    /// Number of animated emojis the user has sent
    pub total_animated_emojis: PremiumUsageData,
    /// Number of global emojis the user has sent
    pub total_global_emojis: PremiumUsageData,
    /// Number of large uploads the user has made
    pub total_large_uploads: PremiumUsageData,
    /// Number of times the user has streamed in HD
    pub total_hd_streams: PremiumUsageData,
    /// Number of hours the user has streamed in HD
    pub hd_hours_streamed: PremiumUsageData,
}

/// Structure for the data in [PremiumUsage].
///
/// Currently only contains the number of uses of a premium perk.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#premium-usage-structure>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PremiumUsageData {
    /// Total number of uses for this perk
    pub value: usize,
}

impl Into<PremiumUsageData> for usize {
	fn into(self) -> PremiumUsageData {
	    PremiumUsageData { value: self }
	}
}

impl Into<usize> for PremiumUsageData {
	fn into(self) -> usize {
	    self.value
	}
}
