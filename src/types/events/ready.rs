// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::entities::{Guild, User};
use crate::types::events::{Session, WebSocketEvent};
use crate::types::{Activity, Channel, ClientStatusObject, GuildMember, PresenceUpdate, Snowflake, VoiceState};

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// 1/2 officially documented;
/// Received after identifying, provides initial user info;
///
/// See <https://docs.discord.sex/topics/gateway-events#ready> and <https://discord.com/developers/docs/topics/gateway-events#ready>
// TODO: There are a LOT of fields missing here
pub struct GatewayReady {
    pub analytics_token: Option<String>,
    pub auth_session_id_hash: Option<String>,
    pub country_code: Option<String>,

    pub v: u8,
    pub user: User,
    /// For bots these are [crate::types::UnavailableGuild]s, for users they are [Guild]
    pub guilds: Vec<Guild>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub sessions: Option<Vec<Session>>,
    pub session_id: String,
    pub session_type: Option<String>,
    pub resume_gateway_url: Option<String>,
    pub shard: Option<(u64, u64)>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Officially Undocumented;
/// Sent after the READY event when a client is a user, 
/// seems to somehow add onto the ready event;
///
/// See <https://docs.discord.sex/topics/gateway-events#ready-supplemental>
pub struct GatewayReadySupplemental {
    /// The presences of the user's relationships and guild presences sent at startup
    pub merged_presences: MergedPresences,
    pub merged_members: Vec<Vec<GuildMember>>,
    pub lazy_private_channels: Vec<Channel>,
    pub guilds: Vec<SupplementalGuild>,
    // "Upcoming changes that the client should disclose to the user" (discord.sex)
    pub disclose: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://docs.discord.sex/topics/gateway-events#merged-presences-structure>
pub struct MergedPresences {
    /// "Presences of the user's guilds in the same order as the guilds array in ready"
    /// (discord.sex)
    pub guilds: Vec<Vec<MergedPresenceGuild>>, 
    /// "Presences of the user's friends and implicit relationships" (discord.sex)
    pub friends: Vec<MergedPresenceFriend>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Not documented even unofficially
pub struct MergedPresenceFriend {
    pub user_id: Snowflake,
    pub status: String,
    // Looks like ms??
    //
    // Not always sent
    pub last_modified: Option<u128>,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Not documented even unofficially
pub struct MergedPresenceGuild {
    pub user_id: Snowflake,
    pub status: String,
    // ?
    pub game: Option<serde_json::Value>,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://docs.discord.sex/topics/gateway-events#supplemental-guild-structure>
pub struct SupplementalGuild {
    pub id: Snowflake,
    pub voice_states: Option<Vec<VoiceState>>,
    /// Field not documented even unofficially
    pub embedded_activities: Vec<serde_json::Value>,
}
