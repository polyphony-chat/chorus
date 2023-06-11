use serde::{Deserialize, Serialize};

use crate::types::{GuildInvite, WebSocketEvent};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#invite-create
pub struct InviteCreate {
    #[serde(flatten)]
    pub invite: GuildInvite,
}

impl WebSocketEvent for InviteCreate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#invite-delete
pub struct InviteDelete {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub code: String,
}

impl WebSocketEvent for InviteDelete {}
