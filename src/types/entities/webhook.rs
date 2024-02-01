// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::gateway::Shared;
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

/// See <https://docs.spacebar.chat/routes/#cmp--schemas-webhook>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub webhook_type: i32,
    pub name: String,
    pub avatar: String,
    pub token: String,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub application_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub source_guild: Option<Shared<Guild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
