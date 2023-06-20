use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#webhooks-update
pub struct WebhooksUpdate {
    pub guild_id: String,
    pub channel_id: String,
}
