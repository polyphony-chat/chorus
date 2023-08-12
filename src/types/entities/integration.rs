use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{Application, User},
    utils::Snowflake,
};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord.com/developers/docs/resources/guild#integration-object-integration-structure>
pub struct Integration {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub integration_type: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enabled_emoticons: Option<bool>,
    pub expire_behaviour: Option<u8>,
    pub expire_grace_period: Option<u16>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Arc<Mutex<User>>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub account: IntegrationAccount,
    pub synced_at: Option<DateTime<Utc>>,
    pub subscriber_count: Option<f64>,
    pub revoked: Option<bool>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub application: Option<Arc<Mutex<Application>>>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/resources/guild#integration-account-object-integration-account-structure>
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}
