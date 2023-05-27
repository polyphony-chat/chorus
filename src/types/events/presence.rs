use crate::types::PublicUser;
use crate::types::events::WebSocketEvent;
use crate::types::interfaces::Activity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields
pub struct PresenceUpdate {
    pub user: PublicUser,
    pub guild_id: Option<String>,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatusObject,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#client-status-object
pub struct ClientStatusObject {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}

impl WebSocketEvent for PresenceUpdate {}
