use serde::{Deserialize, Serialize};
use crate::events::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#request-guild-members-request-guild-members-structure
pub struct GatewayRequestGuildMembers {
    pub guild_id: String,
    pub query: Option<String>,
    pub limit: u64,
    pub presence: Option<bool>,
    pub user_ids: Option<String>,
    pub nonce: Option<String>,
}

impl WebSocketEvent for GatewayRequestGuildMembers {}