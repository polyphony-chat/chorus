use crate::types::entities::{Guild, User};
use crate::types::events::{Session, WebSocketEvent};
use crate::types::interfaces::ClientStatusObject;
use crate::types::{Activity, GuildMember, PresenceUpdate, VoiceState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// 1/2 half documented;
/// Received after identifying, provides initial user info;
/// See https://discord.com/developers/docs/topics/gateway-events#ready;
pub struct GatewayReady {
    pub analytics_token: Option<String>,
    pub auth_session_id_hash: Option<String>,
    pub country_code: Option<String>,

    pub v: u8,
    pub user: User,
    /// For bots these are [UnavailableGuild]s, for users they are [Guild]
    pub guilds: Vec<Guild>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub sessions: Option<Vec<Session>>,
    pub session_id: String,
    pub session_type: Option<String>,
    pub resume_gateway_url: Option<String>,
    pub shard: Option<(u64, u64)>,
}

impl WebSocketEvent for GatewayReady {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Officially Undocumented;
/// Sent after the READY event when a client is a user, seems to somehow add onto the ready event;
pub struct GatewayReadySupplemental {
    pub merged_presences: MergedPresences,
    pub merged_members: Vec<Vec<GuildMember>>,
    // ?
    pub lazy_private_channels: Vec<serde_json::Value>,
    pub guilds: Vec<SupplementalGuild>,
    // ? pomelo
    pub disclose: Vec<String>,
}

impl WebSocketEvent for GatewayReadySupplemental {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct MergedPresences {
    pub guilds: Vec<Vec<MergedPresenceGuild>>,
    pub friends: Vec<MergedPresenceFriend>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct MergedPresenceFriend {
    pub user_id: String,
    pub status: String,
    /// Looks like ms??
    pub last_modified: u128,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct MergedPresenceGuild {
    pub user_id: String,
    pub status: String,
    // ?
    pub game: Option<serde_json::Value>,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SupplementalGuild {
    pub voice_states: Option<Vec<VoiceState>>,
    pub id: String,
    pub embedded_activities: Vec<serde_json::Value>,
}
