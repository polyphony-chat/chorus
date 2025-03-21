// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus_macros::{JsonField, SourceUrlField};
use serde::{Deserialize, Serialize};

use crate::types::entities::{Channel, ThreadMember};
use crate::types::events::WebSocketEvent;
use crate::types::{JsonField, Snowflake, SourceUrlField};

#[cfg(feature = "client")]
use super::UpdateMessage;

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#thread-create>
pub struct ThreadCreate {
    #[serde(flatten)]
    pub thread: Channel,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, JsonField, SourceUrlField, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#thread-update>
pub struct ThreadUpdate {
    #[serde(flatten)]
    pub thread: Channel,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Channel> for ThreadUpdate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.thread.id)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#thread-delete>
pub struct ThreadDelete {
    #[serde(flatten)]
    pub thread: Channel,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#thread-list-sync>
pub struct ThreadListSync {
    pub guild_id: Snowflake,
    pub channel_ids: Option<Vec<Snowflake>>,
    pub threads: Vec<Channel>,
    pub members: Option<Vec<ThreadMember>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#thread-member-update>
/// The inner payload is a thread member object with an extra field.
pub struct ThreadMemberUpdate {
    #[serde(flatten)]
    pub member: ThreadMember,
    pub guild_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#thread-members-update>
pub struct ThreadMembersUpdate {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    /// Capped at 50
    pub member_count: u8,
    pub added_members: Option<Vec<ThreadMember>>,
    pub removed_members: Option<Vec<Snowflake>>,
}

