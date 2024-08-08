// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{
    entities::{Application, User},
    utils::Snowflake,
    Shared,
};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord.com/developers/docs/resources/guild#integration-object-integration-structure>
pub struct Integration {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub integration_type: IntegrationType,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enabled_emoticons: Option<bool>,
    pub expire_behaviour: Option<IntegrationExpireBehaviour>,
    pub expire_grace_period: Option<u16>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub account: IntegrationAccount,
    pub synced_at: Option<DateTime<Utc>>,
    pub subscriber_count: Option<f64>,
    pub revoked: Option<bool>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub application: Option<Shared<Application>>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/resources/guild#integration-account-object-integration-account-structure>
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(
    Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "snake_case"))]
/// See <https://docs.discord.sex/resources/integration#integration-type>
pub enum IntegrationType {
    #[default]
    Twitch,
    Youtube,
    Discord,
    GuildSubscription,
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
/// Defines the behaviour that is executed when a user's subscription to the integration expires.
///
/// See <https://docs.discord.sex/resources/integration#integration-expire-behavior>
pub enum IntegrationExpireBehaviour {
    #[default]
	 /// Remove the subscriber role from the user
    RemoveRole = 0,
	 /// Kick the user from the guild
	 Kick = 1,
}


