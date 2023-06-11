use serde::{Deserialize, Serialize};

use crate::types::entities::{Guild, User};
use crate::types::events::{Session, WebSocketEvent};
use crate::types::interfaces::ClientStatusObject;
use crate::types::{Activity, GuildMember, PresenceUpdate, VoiceState};

#[derive(Debug, Deserialize, Serialize, Default)]
/// Sort of documented, though most fields are left out
/// For a full example see https://gist.github.com/kozabrada123/a347002b1fb8825a5727e40746d4e199
/// to:do add all undocumented fields
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

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Sent after the READY event when a client is a user
/// {"t":"READY_SUPPLEMENTAL","s":2,"op":0,"d":{"merged_presences":{"guilds":[[{"user_id":"463640391196082177","status":"online","game":null,"client_status":{"web":"online"},"activities":[]}]],"friends":[{"user_id":"463640391196082177","status":"online","last_modified":1684053508443,"client_status":{"web":"online"},"activities":[]}]},"merged_members":[[{"user_id":"463640391196082177","roles":[],"premium_since":null,"pending":false,"nick":"pog","mute":false,"joined_at":"2021-05-30T15:24:08.763000+00:00","flags":0,"deaf":false,"communication_disabled_until":null,"avatar":null}]],"lazy_private_channels":[],"guilds":[{"voice_states":[],"id":"848582562217590824","embedded_activities":[]}],"disclose":["pomelo"]}}
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

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MergedPresences {
    pub guilds: Vec<Vec<MergedPresenceGuild>>,
    pub friends: Vec<MergedPresenceFriend>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MergedPresenceFriend {
    pub user_id: String,
    pub status: String,
    /// Looks like ms??
    pub last_modified: u128,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MergedPresenceGuild {
    pub user_id: String,
    pub status: String,
    // ?
    pub game: Option<serde_json::Value>,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SupplementalGuild {
    pub voice_states: Option<Vec<VoiceState>>,
    pub id: String,
    pub embedded_activities: Vec<serde_json::Value>,
}
