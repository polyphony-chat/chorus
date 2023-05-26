use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{Application, User},
    utils::Snowflake,
};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/resources/guild#integration-object-integration-structure
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
    pub user: Option<User>,
    pub account: IntegrationAccount,
    pub synced_at: Option<DateTime<Utc>>,
    pub subscriber_count: Option<f64>,
    pub revoked: Option<bool>,
    pub application: Option<Application>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/resources/guild#integration-account-object-integration-account-structure
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}
