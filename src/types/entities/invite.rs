// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::types::guild_configuration::GuildFeaturesList;
use crate::types::{
    Guild, InviteFlags, InviteTargetType, InviteType, Shared, Snowflake, VerificationLevel,
    WelcomeScreenObject,
};
use crate::{UInt32, UInt8};

use super::guild::GuildScheduledEvent;
use super::{Application, Channel, GuildMember, NSFWLevel, User};

/// Represents a code that when used, adds a user to a guild or group DM channel, or creates a relationship between two users.
/// See <https://discord-userdoccers.vercel.app/resources/invite#invite-object>
#[derive(Default, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Invite {
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub approximate_member_count: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub approximate_presence_count: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub channel: Option<Channel>,
    pub code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub flags: Option<InviteFlags>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub guild: Option<InviteGuild>,
    pub guild_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub guild_scheduled_event: Option<Shared<GuildScheduledEvent>>,
    #[serde(rename = "type")]
    #[cfg_attr(feature = "sqlx", sqlx(rename = "type"))]
    pub invite_type: Option<InviteType>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub inviter: Option<User>,
    pub max_age: Option<UInt32>,
    pub max_uses: Option<UInt8>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub stage_instance: Option<InviteStageInstance>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub target_application: Option<Application>,
    #[cfg_attr(feature = "sqlx", sqlx(rename = "target_user_type"))]
    pub target_type: Option<InviteTargetType>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub target_user: Option<User>,
    pub temporary: Option<bool>,
    pub uses: Option<UInt32>,
}

/// The guild an invite is for.
/// See <https://discord-userdoccers.vercel.app/resources/invite#invite-guild-object>
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteGuild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub verification_level: VerificationLevel,
    pub features: GuildFeaturesList,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_subscription_count: Option<i32>,
    #[serde(rename = "nsfw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw_deprecated: Option<bool>,
    pub nsfw_level: NSFWLevel,
    pub welcome_screen: Option<WelcomeScreenObject>,
}

impl From<Guild> for InviteGuild {
    fn from(value: Guild) -> Self {
        Self {
            id: value.id,
            name: value.name.unwrap_or_default(),
            icon: value.icon,
            splash: value.splash,
            verification_level: value.verification_level.unwrap_or_default(),
            features: value.features,
            vanity_url_code: value.vanity_url_code,
            description: value.description,
            banner: value.banner,
            premium_subscription_count: value.premium_subscription_count,
            nsfw_deprecated: None,
            nsfw_level: value.nsfw_level.unwrap_or_default(),
            #[cfg(feature = "sqlx")]
            welcome_screen: value.welcome_screen.0,
            #[cfg(not(feature = "sqlx"))]
            welcome_screen: value.welcome_screen,
        }
    }
}

/// See <https://discord-userdoccers.vercel.app/resources/invite#invite-stage-instance-object>
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteStageInstance {
    pub members: Vec<Shared<GuildMember>>,
    pub participant_count: i32,
    pub speaker_count: i32,
    pub topic: String,
}
