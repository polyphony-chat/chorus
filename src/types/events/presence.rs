// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, UserStatus};
use crate::types::{Activity, ClientStatusObject, PublicUser, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Sent by the client to update its status and presence;
/// See <https://discord.com/developers/docs/topics/gateway-events#update-presence>
pub struct UpdatePresence {
    /// Unix time of when the client went idle, or n
    /// one if client is not idle.
    pub since: Option<u128>,
    /// the client's status (online, invisible, offline, dnd, idle..)
    pub status: UserStatus,
    pub activities: Vec<Activity>,
    pub afk: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, WebSocketEvent)]
/// Received to tell the client that a user updated their presence / status. If you are looking for
/// the PresenceUpdate used in the IDENTIFY gateway event, see
///
/// See <https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields>
/// (Same structure as <https://docs.discord.sex/resources/presence#presence-object>)
pub struct PresenceUpdate {
    pub user: PublicUser,
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub status: UserStatus,
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatusObject,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, WebSocketEvent)]
/// Sent to the gateway as part of [GatewayIdentifyPayload](crate::types::GatewayIdentifyPayload)
pub struct GatewayIdentifyPresenceUpdate {
    #[serde(default)]
    pub guild_id: Option<Snowflake>,
    pub status: UserStatus,
    #[serde(default)]
    pub activities: Vec<Activity>,
}

impl From<PresenceUpdate> for GatewayIdentifyPresenceUpdate {
    fn from(value: PresenceUpdate) -> Self {
        Self {
            guild_id: value.guild_id,
            status: value.status,
            activities: value.activities,
        }
    }
}
