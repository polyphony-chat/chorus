use serde::{Deserialize, Serialize};
use crate::entities::{UnavailableGuild, User};
use crate::events::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayReady {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: Option<String>,
    pub shard: Option<(u64, u64)>,
}

impl WebSocketEvent for GatewayReady {}