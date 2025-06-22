// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::types::Shared;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::{Composite, Updateable};

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

use crate::types::{
    entities::{Guild, User},
    utils::Snowflake,
};

use super::option_arc_rwlock_ptr_eq;

/// See <https://docs.spacebar.chat/routes/#cmp--schemas-webhook>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub webhook_type: WebhookType,
    pub name: String,
    pub avatar: String,
    pub token: String,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub application_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub source_guild: Option<Shared<Guild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for Webhook {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.webhook_type == other.webhook_type
            && self.name == other.name
            && self.avatar == other.avatar
            && self.token == other.token
            && self.guild_id == other.guild_id
            && self.channel_id == other.channel_id
            && self.application_id == other.application_id
            && option_arc_rwlock_ptr_eq(&self.user, &other.user)
            && option_arc_rwlock_ptr_eq(&self.source_guild, &other.source_guild)
            && self.url == other.url
    }
}

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum WebhookType {
    #[default]
    Incoming = 1,
    ChannelFollower = 2,
    Application = 3,
}
