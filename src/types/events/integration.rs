use serde::{Deserialize, Serialize};

use crate::types::Integration;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#integration-create
pub struct IntegrationCreate {
    #[serde(flatten)]
    pub integration: Integration,
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#integration-update
pub struct IntegrationUpdate {
    #[serde(flatten)]
    pub integration: Integration,
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#integration-delete
pub struct IntegrationDelete {
    pub id: String,
    pub guild_id: String,
    pub application_id: Option<String>,
}
