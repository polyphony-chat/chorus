// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

use crate::types::{ChannelType, DefaultReaction, Error, entities::PermissionOverwrite, Snowflake};

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreateSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: Option<ChannelType>,
    pub topic: Option<String>,
    pub icon: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<Snowflake>,
    pub id: Option<Snowflake>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<String>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct ChannelModifySchema {
    pub name: Option<String>,
    pub channel_type: Option<ChannelType>,
    pub topic: Option<String>,
    pub icon: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<Snowflake>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct GetChannelMessagesSchema {
    /// Between 1 and 100, defaults to 50.
    pub limit: Option<i32>,
    #[serde(flatten)]
    pub anchor: Option<ChannelMessagesAnchor>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMessagesAnchor {
    Before(Snowflake),
    Around(Snowflake),
    After(Snowflake),
}

impl GetChannelMessagesSchema {
    pub fn before(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: Some(ChannelMessagesAnchor::Before(anchor)),
        }
    }

    pub fn around(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: Some(ChannelMessagesAnchor::Around(anchor)),
        }
    }

    pub fn after(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: Some(ChannelMessagesAnchor::After(anchor)),
        }
    }

    /// Must be between 1 and 100
    pub fn limit(self, limit: i32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct CreateChannelInviteSchema {
    pub flags: Option<InviteFlags>,
    pub max_age: Option<u32>,
    pub max_uses: Option<u8>,
    pub temporary: Option<bool>,
    pub unique: Option<bool>,
    pub validate: Option<String>,
    pub target_type: Option<InviteTargetType>,
    pub target_user_id: Option<Snowflake>,
    pub target_application_id: Option<Snowflake>,
}

impl Default for CreateChannelInviteSchema {
    fn default() -> Self {
        Self {
            flags: None,
            max_age: Some(86400),
            max_uses: Some(0),
            temporary: Some(false),
            unique: Some(false),
            validate: None,
            target_type: None,
            target_user_id: None,
            target_application_id: None,
        }
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    pub struct InviteFlags: u64 {
        const GUEST = 1 << 0;
        const VIEWED = 1 << 1;
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum InviteType {
    #[default]
    Guild = 0,
    GroupDm = 1,
    Friend = 2,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum InviteTargetType {
    #[default]
    Stream = 1,
    EmbeddedApplication = 2,
    RoleSubscriptions = 3,
    CreatorPage = 4,
}

/// See <https://discord-userdoccers.vercel.app/resources/channel#add-channel-recipient>
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct AddChannelRecipientSchema {
    pub access_token: Option<String>,
    pub nick: Option<String>,
}

/// See <https://discord-userdoccers.vercel.app/resources/channel#add-channel-recipient>
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct ModifyChannelPositionsSchema {
    pub id: Snowflake,
    pub position: Option<u32>,
    pub lock_permissions: Option<bool>,
    pub parent_id: Option<Snowflake>,
}

/// See <https://docs.discord.sex/resources/channel#follow-channel>
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct AddFollowingChannelSchema {
    pub webhook_channel_id: Snowflake,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct CreateWebhookSchema {
    pub name: String,
    pub avatar: Option<String>,
}