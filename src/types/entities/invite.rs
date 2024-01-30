// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::gateway::Shared;
use crate::types::{Snowflake, WelcomeScreenObject};

use super::guild::GuildScheduledEvent;
use super::{Application, Channel, GuildMember, NSFWLevel, User};

/// Represents a code that when used, adds a user to a guild or group DM channel, or creates a relationship between two users.
/// See <https://discord-userdoccers.vercel.app/resources/invite#invite-object>
#[derive(Debug, Serialize, Deserialize)]
pub struct Invite {
    pub approximate_member_count: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub channel: Option<Channel>,
    pub code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub flags: Option<i32>,
    pub guild: Option<InviteGuild>,
    pub guild_id: Option<Snowflake>,
    pub guild_scheduled_event: Option<Shared<GuildScheduledEvent>>,
    #[serde(rename = "type")]
    pub invite_type: Option<i32>,
    pub inviter: Option<User>,
    pub max_age: Option<i32>,
    pub max_uses: Option<i32>,
    pub stage_instance: Option<InviteStageInstance>,
    pub target_application: Option<Application>,
    pub target_type: Option<i32>,
    pub target_user: Option<User>,
    pub temporary: Option<bool>,
    pub uses: Option<i32>,
}

/// The guild an invite is for.
/// See <https://discord-userdoccers.vercel.app/resources/invite#invite-guild-object>
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteGuild {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub verification_level: i32,
    pub features: Vec<String>,
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

/// See <https://discord-userdoccers.vercel.app/resources/invite#invite-stage-instance-object>
#[derive(Debug, Serialize, Deserialize)]
pub struct InviteStageInstance {
    pub members: Vec<Shared<GuildMember>>,
    pub participant_count: i32,
    pub speaker_count: i32,
    pub topic: String,
}
