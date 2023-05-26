use serde::{Deserialize, Serialize};
use crate::entities::{Channel, GuildMember, ThreadMember};
use crate::events::WebSocketEvent;

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-create
/// Not directly serialized, as the inner payload is a channel object
pub struct ThreadCreate {
    pub thread: Channel,
}

impl WebSocketEvent for ThreadCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-update
/// Not directly serialized, as the inner payload is a channel object
pub struct ThreadUpdate {
    pub thread: Channel,
}

impl WebSocketEvent for ThreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-delete
/// Not directly serialized, as the inner payload is a channel object
pub struct ThreadDelete {
    pub thread: Channel,
}

impl WebSocketEvent for ThreadDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-list-sync
pub struct ThreadListSync {
    pub guild_id: String,
    pub channel_ids: Option<Vec<String>>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>,
}

impl WebSocketEvent for ThreadListSync {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-member-update
/// The inner payload is a thread member object with an extra field.
/// The extra field is a bit painful, because we can't just serialize a thread member object
pub struct ThreadMemberUpdate {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub join_timestamp: Option<String>,
    pub flags: Option<u64>,
    pub member: Option<GuildMember>,
    pub guild_id: String,
}

impl ThreadMemberUpdate {
    /// Convert self to a thread member, losing the added guild_id field
    pub fn to_thread_member(self) -> ThreadMember {
        ThreadMember {
            id: self.id,
            user_id: self.user_id,
            join_timestamp: self.join_timestamp.clone(),
            flags: self.flags,
            member: self.member,
        }
    }
}

impl WebSocketEvent for ThreadMemberUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
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