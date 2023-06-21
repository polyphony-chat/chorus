use serde::{Deserialize, Serialize};

use crate::types::Snowflake;

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#webhooks-update
pub struct WebhooksUpdate {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}

impl WebSocketEvent for WebhooksUpdate {}
