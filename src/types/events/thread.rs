use crate::types::entities::{Channel, ThreadMember};
use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-create
pub struct ThreadCreate {
    #[serde(flatten)]
    pub thread: Channel,
}

impl WebSocketEvent for ThreadCreate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-update
pub struct ThreadUpdate {
    #[serde(flatten)]
    pub thread: Channel,
}

impl WebSocketEvent for ThreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-delete
pub struct ThreadDelete {
    #[serde(flatten)]
    pub thread: Channel,
}

impl WebSocketEvent for ThreadDelete {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-list-sync
pub struct ThreadListSync {
    pub guild_id: String,
    pub channel_ids: Option<Vec<String>>,
    pub threads: Vec<Channel>,
    pub members: Option<Vec<ThreadMember>>,
}

impl WebSocketEvent for ThreadListSync {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-member-update
/// The inner payload is a thread member object with an extra field.
pub struct ThreadMemberUpdate {
    #[serde(flatten)]
    pub member: ThreadMember,
    pub guild_id: String,
}

impl WebSocketEvent for ThreadMemberUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-members-update
pub struct ThreadMembersUpdate {
    pub id: String,
    pub guild_id: String,
    /// Capped at 50
    pub member_count: u8,
    pub added_members: Option<Vec<ThreadMember>>,
    pub removed_members: Option<Vec<String>>,
}

impl WebSocketEvent for ThreadMembersUpdate {}
