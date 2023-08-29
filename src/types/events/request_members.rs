use crate::types::{events::WebSocketEvent, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
/// See <https://discord.com/developers/docs/topics/gateway-events#request-guild-members-request-guild-members-structure>
pub struct GatewayRequestGuildMembers {
    pub guild_id: Snowflake,
    pub query: Option<String>,
    pub limit: u64,
    pub presences: Option<bool>,
    // TODO: allow array
    pub user_ids: Option<Snowflake>,
    pub nonce: Option<String>,
}

impl WebSocketEvent for GatewayRequestGuildMembers {}
