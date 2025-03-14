// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::entities::{Guild, User};
use crate::types::events::{Session, WebSocketEvent};
use crate::types::{
    Activity, Channel, ClientStatusObject, GuildMember, MfaAuthenticatorType, PresenceUpdate,
    Relationship, Snowflake, UserSettings, VoiceState,
};
use crate::{UInt32, UInt64, UInt8};

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Received after identifying, provides initial user information and client state.
///
/// See <https://docs.discord.sex/topics/gateway-events#ready> and <https://discord.com/developers/docs/topics/gateway-events#ready>
pub struct GatewayReady {
    #[serde(default)]
    /// An array of stringified JSON values representing the connection trace, used for debugging
    pub _trace: Vec<String>,
    /// The token used for analytical tracking requests
    pub analytics_token: String,
    /// The hash of the auth session ID corresponding to the auth token used to connect
    pub auth_session_id_hash: String,
    /// The detected ISO 3166-1 alpha-2 country code of the user's current IP address
    pub country_code: String,
    #[serde(rename = "v")]
    /// API version
    pub api_version: UInt8,
    /// The connected user
    pub user: User,
    #[serde(default)]
    /// The guilds the user is in
    pub guilds: Vec<Guild>,
    /// The presences of the user's non-offline friends and implicit relationships (depending on the `NO_AFFINE_USER_IDS` Gateway capability).
    pub presences: Option<Vec<PresenceUpdate>>,
    /// Undocumented. Seems to be a list of sessions the user is currently connected with.
    /// On Discord.com, this includes the current session.
    pub sessions: Option<Vec<Session>>,
    /// Unique session ID, used for resuming connections
    pub session_id: String,
    /// The type of session that was started
    pub session_type: String,
    /// WebSocket URL for resuming connections
    pub resume_gateway_url: String,
    /// The shard information (shard_id, num_shards) associated with this session, if sharded
    pub shard: Option<(UInt64, UInt64)>,
    /// The client settings for the user
    pub user_settings: Option<UserSettings>,
    /// The base-64 encoded preloaded user settings for the user, (if missing, defaults are used)
    pub user_settings_proto: Option<String>,
    #[serde(default)]
    /// The relationships the user has with other users
    pub relationships: Vec<Relationship>,
    /// The number of friend suggestions the user has
    pub friend_suggestion_count: UInt32,
    #[serde(default)]
    /// The DMs and group DMs the user is participating in
    pub private_channels: Vec<Channel>,
    #[serde(default)]
    /// A mapping of user IDs to notes the user has made for them
    pub notes: HashMap<Snowflake, String>,
    /// The presences of the user's non-offline friends and implicit relationships (depending on the `NO_AFFINE_USER_IDS` Gateway capability), and any guild presences sent at startup
    pub merged_presences: Option<MergedPresences>,
    /// The members of the user's guilds, in the same order as the `guilds` array
    #[serde(default)]
    pub merged_members: Option<Vec<Vec<GuildMember>>>,
    #[serde(default)]
    /// The deduped users across all objects in the event
    pub users: Vec<User>,
    /// The refreshed auth token for this user; if present, the client should discard the current auth token and use this in subsequent requests to the API
    pub auth_token: Option<String>,
    #[serde(default)]
    /// The types of multi-factor authenticators the user has enabled
    pub authenticator_types: Vec<MfaAuthenticatorType>,
    /// The action a user is required to take before continuing to use Discord
    pub required_action: Option<String>,
    #[serde(default)]
    /// A geo-ordered list of RTC regions that can be used when when setting a voice channel's `rtc_region` or updating the client's voice state
    pub geo_ordered_rtc_regions: Vec<String>,
    /// The tutorial state of the user, if any
    /// TODO: Make tutorial object into object
    pub tutorial: Option<String>,
    /// The API code version, used when re-identifying with client state v2
    pub api_code_version: UInt8,
    #[serde(default)]
    /// User experiment rollouts for the user
    ///
    /// TODO: Make User Experiments into own struct
    // Note: this is a pain to parse! We need a way to parse arrays into structs via the index of
    // their feilds
    //
    // ex: [4130837190, 0, 10, -1, 0, 1932, 0, 0]
    // needs to be parsed into a struct with fields corresponding to the first, second.. value in
    // the array
    pub experiments: Vec<serde_json::value::Value>,
    #[serde(default)]
    /// Guild experiment rollouts for the user
    ///
    /// TODO: Make Guild Experiments into own struct
    // Note: this is a pain to parse! See the above TODO
    pub guild_experiments: Vec<serde_json::value::Value>,
    pub read_state: Vec<ReadStateEntry>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Received after identifying, provides initial information about the bot session.
///
/// See <https://docs.discord.sex/topics/gateway-events#ready> and <https://discord.com/developers/docs/topics/gateway-events#ready>
pub struct GatewayReadyBot {
    #[serde(default)]
    /// An array of stringified JSON values representing the connection trace, used for debugging
    pub _trace: Vec<String>,
    #[serde(rename = "v")]
    /// API version
    pub api_version: UInt8,
    /// The connected bot user
    pub user: User,
    #[serde(default)]
    /// The guilds the bot user is in. Will be `UnavailableGuilds` at first.
    pub guilds: Vec<Guild>,
    /// The presences of the user's non-offline friends and implicit relationships (depending on the `NO_AFFINE_USER_IDS` Gateway capability).
    pub presences: Option<Vec<PresenceUpdate>>,
    /// Unique session ID, used for resuming connections
    pub session_id: String,
    /// The type of session that was started
    pub session_type: String,
    /// WebSocket URL for resuming connections
    pub resume_gateway_url: String,
    /// The shard information (shard_id, num_shards) associated with this session, if sharded
    pub shard: Option<(UInt64, UInt64)>,
    /// The presences of the user's non-offline friends and implicit relationships (depending on the `NO_AFFINE_USER_IDS` Gateway capability), and any guild presences sent at startup
    pub merged_presences: Option<MergedPresences>,
    #[serde(default)]
    /// The deduped users across all objects in the event
    pub users: Vec<User>,
    #[serde(default)]
    /// The types of multi-factor authenticators the user has enabled
    pub authenticator_types: Vec<MfaAuthenticatorType>,
    #[serde(default)]
    /// A geo-ordered list of RTC regions that can be used when when setting a voice channel's `rtc_region` or updating the client's voice state
    pub geo_ordered_rtc_regions: Vec<String>,
    /// The API code version, used when re-identifying with client state v2
    pub api_code_version: UInt8,
}

impl From<GatewayReady> for GatewayReadyBot {
    fn from(value: GatewayReady) -> Self {
        GatewayReadyBot {
            api_version: value.api_version,
            user: value.user,
            guilds: value.guilds,
            presences: value.presences,
            session_id: value.session_id,
            session_type: value.session_type,
            resume_gateway_url: value.resume_gateway_url,
            shard: value.shard,
            merged_presences: value.merged_presences,
            users: value.users,
            authenticator_types: value.authenticator_types,
            geo_ordered_rtc_regions: value.geo_ordered_rtc_regions,
            api_code_version: value.api_code_version,
            _trace: value._trace,
        }
    }
}

impl GatewayReady {
    /// Convert this struct into a [GatewayReadyBot] struct
    pub fn to_bot(self) -> GatewayReadyBot {
        self.into()
    }
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

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Not documented even unofficially. Information about this type is likely to be partially incorrect.
pub struct ReadState {
    pub entries: Vec<ReadStateEntry>,
    pub partial: bool,
    pub version: u32,
}

#[derive(
    Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy,
)]
/// Not documented even unofficially. Information about this type is likely to be partially incorrect.
pub struct ReadStateEntry {
    /// Spacebar servers do not have flags in this entity at all (??)
    pub flags: Option<u32>,
    pub id: Snowflake,
    pub last_message_id: Option<Snowflake>,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    /// A value that is incremented each time the read state is read
    pub last_viewed: Option<u32>,
    // Temporary adding Option to fix Spacebar servers, they have mention count as a nullable
    pub mention_count: Option<u64>,
}
