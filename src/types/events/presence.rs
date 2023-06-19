use crate::types::{events::WebSocketEvent, UserStatus};
use crate::types::{Activity, ClientStatusObject, PublicUser, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Sent by the client to update its status and presence;
/// See https://discord.com/developers/docs/topics/gateway-events#update-presence
pub struct UpdatePresence {
    /// unix time of when the client went idle, or none if client is not idle
    pub since: Option<u128>,
    /// the client's status (online, invisible, offline, dnd, idle..)
    pub status: UserStatus,
    pub activities: Vec<Activity>,
    pub afk: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Received to tell the client that a user updated their presence / status
/// See https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields
pub struct PresenceUpdate {
    pub user: PublicUser,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub status: UserStatus,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatusObject,
}

impl WebSocketEvent for PresenceUpdate {}
