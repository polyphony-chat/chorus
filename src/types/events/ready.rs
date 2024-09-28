// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::entities::{Guild, User};
use crate::types::events::{Session, WebSocketEvent};
use crate::types::{
    Activity, Channel, ClientStatusObject, GuildMember, PresenceUpdate, Relationship, Snowflake,
    UserSettings, VoiceState,
};

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Received after identifying, provides initial user info
///
/// See <https://docs.discord.sex/topics/gateway-events#ready> and <https://discord.com/developers/docs/topics/gateway-events#ready>
pub struct GatewayReady {
    pub analytics_token: String,
    pub auth_session_id_hash: String,
    pub country_code: String,
    pub v: u8,
    pub user: User,
    pub guilds: Vec<Guild>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub sessions: Option<Vec<Session>>,
    pub session_id: String,
    pub session_type: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u64, u64)>,
    pub user_settings: Option<UserSettings>,
    pub user_settings_proto: Option<String>,
    pub relationships: Vec<Relationship>,
    pub friend_suggestion_count: u32,
    pub private_channels: Vec<Channel>,
    pub notes: HashMap<Snowflake, String>,
    pub merged_presences: Option<MergedPresences>,
    pub users: Vec<User>,
    pub auth_token: Option<String>,
    pub authenticator_types: Vec<AuthenticatorType>,
    pub required_action: Option<String>,
    pub geo_ordered_rtc_regions: Vec<String>,
    /// TODO: Make tutorial object into object
    pub tutorial: Option<String>,
    pub api_code_version: u8,
    pub experiments: Vec<String>,
    pub guild_experiments: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Received after identifying, provides initial information about the bot session.
///
/// See <https://docs.discord.sex/topics/gateway-events#ready> and <https://discord.com/developers/docs/topics/gateway-events#ready>
pub struct GatewayReadyBot {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<Guild>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub sessions: Option<Vec<Session>>,
    pub session_id: String,
    pub session_type: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u64, u64)>,
    pub merged_presences: Option<MergedPresences>,
    pub users: Vec<User>,
    pub authenticator_types: Vec<AuthenticatorType>,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub api_code_version: u8,
}

impl From<GatewayReady> for GatewayReadyBot {
    fn from(value: GatewayReady) -> Self {
        GatewayReadyBot {
            v: value.v,
            user: value.user,
            guilds: value.guilds,
            presences: value.presences,
            sessions: value.sessions,
            session_id: value.session_id,
            session_type: value.session_type,
            resume_gateway_url: value.resume_gateway_url,
            shard: value.shard,
            merged_presences: value.merged_presences,
            users: value.users,
            authenticator_types: value.authenticator_types,
            geo_ordered_rtc_regions: value.geo_ordered_rtc_regions,
            api_code_version: value.api_code_version,
        }
    }
}

impl GatewayReady {
    /// Convert this struct into a [GatewayReadyBot] struct
    pub fn to_bot(self) -> GatewayReadyBot {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum AuthenticatorType {
    WebAuthn = 1,
    Totp = 2,
    Sms = 3,
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
