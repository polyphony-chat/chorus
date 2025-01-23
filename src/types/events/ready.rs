// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::entities::{Guild, User};
use crate::types::events::{Session, WebSocketEvent};
use crate::types::types::guild_configuration::GuildFeaturesList;
use crate::types::{
    Activity, Channel, ClientStatusObject, Emoji, GuildMember, GuildScheduledEvent, HubType,
    MfaAuthenticatorType, PresenceUpdate, Relationship, RoleObject, Snowflake, StageInstance,
    Sticker, UserSettings, VoiceState,
};
use crate::{UInt32, UInt64, UInt8};
use chrono::{DateTime, Utc};
use serde::de::{SeqAccess, Visitor};
use serde::ser::Error;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Formatter, Write};

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
    pub guilds: Vec<GatewayGuild>,
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
    pub experiments: Vec<UserExperiment>,
    #[serde(default)]
    /// Guild experiment rollouts for the user
    ///
    /// TODO: Make Guild Experiments into own struct
    // Note: this is a pain to parse! See the above TODO
    pub guild_experiments: Vec<serde_json::value::Value>,
    pub read_state: ReadState,
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
    pub guilds: Vec<GatewayGuild>,
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

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct GatewayGuild {
    pub joined_at: DateTime<Utc>,
    pub large: bool,
    pub unavailable: bool,
    #[serde(default)]
    pub geo_restricted: bool,
    pub member_count: u64,
    pub voice_states: Vec<VoiceState>,
    pub members: Vec<GuildMember>,
    pub channels: Vec<Channel>,
    pub threads: Vec<Channel>,
    pub presences: Vec<PresenceUpdate>,
    pub stage_instances: Vec<StageInstance>,
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub data_mode: GuildDataMode,
    pub properties: Guild,
    pub stickers: Vec<Sticker>,
    pub roles: Vec<RoleObject>,
    pub emojis: Vec<Emoji>,
    pub premium_subscription_count: i32,
}

#[derive(
    Debug, Deserialize, Serialize, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "lowercase")]
pub enum GuildDataMode {
    Full,
    Partial,
    #[default]
    Unavailable,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserExperiment {
    pub hash: u32,
    pub revision: u32,
    pub bucket: u32,
    pub r#override: i8,
    pub population: u32,
    pub hash_result: u32,
    pub aa_mode: i8,
    pub trigger_debugging: i8,
}

impl serde::Serialize for UserExperiment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde_json::to_value([
            serde_json::to_value(self.hash).map_err(S::Error::custom)?,
            serde_json::to_value(self.revision).map_err(S::Error::custom)?,
            serde_json::to_value(self.bucket).map_err(S::Error::custom)?,
            serde_json::to_value(self.r#override).map_err(S::Error::custom)?,
            serde_json::to_value(self.population).map_err(S::Error::custom)?,
            serde_json::to_value(self.hash_result).map_err(S::Error::custom)?,
            serde_json::to_value(self.aa_mode).map_err(S::Error::custom)?,
            serde_json::to_value(self.trigger_debugging).map_err(S::Error::custom)?,
        ])
        .map_err(S::Error::custom)?
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserExperiment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(UserExperimentsVisitor)
    }
}

struct UserExperimentsVisitor;

impl<'de> serde::de::Visitor<'de> for UserExperimentsVisitor {
    type Value = UserExperiment;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of 8 integers")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let hash = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let revision = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let bucket = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let r#override = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        let population = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(4, &self))?;
        let hash_result = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(5, &self))?;
        let aa_mode = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(6, &self))?;
        let trigger_debugging = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(7, &self))?;

        Ok(UserExperiment {
            hash,
            revision,
            bucket,
            r#override,
            population,
            hash_result,
            aa_mode,
            trigger_debugging,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperiment {
    pub hash: u32,
    pub hash_key: Option<String>,
    pub revision: u32,
    pub populations: Vec<GuildExperimentPopulation>,
    pub overrides: Vec<GuildExperimentBucketOverride>,
    pub overrides_formatted: Vec<Vec<GuildExperimentPopulation>>,
    pub holdout_name: Option<String>,
    pub holdout_bucket: Option<u32>,
    pub aa_mode: i8,
    pub trigger_debugging: i8,
}

impl<'de> Deserialize<'de> for GuildExperiment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(GuildExperimentsVisitor)
    }
}

struct GuildExperimentsVisitor;
impl<'de> Visitor<'de> for GuildExperimentsVisitor {
    type Value = GuildExperiment;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let hash = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let hash_key = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let revision = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let populations = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        let overrides = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(4, &self))?;
        let overrides_formatted = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(5, &self))?;
        let holdout_name = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(6, &self))?;
        let holdout_bucket = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(7, &self))?;
        let aa_mode = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(8, &self))?;
        let trigger_debugging = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(9, &self))?;

        Ok(GuildExperiment {
            hash,
            hash_key,
            revision,
            populations,
            overrides,
            overrides_formatted,
            holdout_name,
            holdout_bucket,
            aa_mode,
            trigger_debugging,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperimentPopulation {
    pub ranges: Vec<GuildExperimentPopulationRange>,
    pub filters: GuildExperimentPopulationFilters,
}

impl<'de> Deserialize<'de> for GuildExperimentPopulation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(GuildExperimentPopulationVisitor)
    }
}

struct GuildExperimentPopulationVisitor;

impl<'de> Visitor<'de> for GuildExperimentPopulationVisitor {
    type Value = GuildExperimentPopulation;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let ranges = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let filters = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        Ok(GuildExperimentPopulation { ranges, filters })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperimentPopulationRange {
    pub bucket: i32,
    pub rollout: Vec<GuildExperimentPopulationRollout>,
}

impl<'de> Deserialize<'de> for GuildExperimentPopulationRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(GuildExperimentPopulationRangeVisitor)
    }
}

struct GuildExperimentPopulationRangeVisitor;

impl<'de> Visitor<'de> for GuildExperimentPopulationRangeVisitor {
    type Value = GuildExperimentPopulationRange;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let bucket = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let rollout = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        Ok(GuildExperimentPopulationRange { bucket, rollout })
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct GuildExperimentPopulationRollout {
    #[serde(rename = "s")]
    pub start: u32,
    #[serde(rename = "e")]
    pub end: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperimentPopulationFilters {
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_has_feature: Option<GuildExperimentPopulationFeatureFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id_range: Option<GuildExperimentPopulationRangeFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_age_range_days: Option<GuildExperimentPopulationRangeFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_member_count_range: Option<GuildExperimentPopulationRangeFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_ids: Option<GuildExperimentPopulationIdFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_hub_types: Option<GuildExperimentPopulationHubTypeFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_has_vanity_url: Option<GuildExperimentPopulationVanityUrlFilter>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_in_range_by_hash: Option<GuildExperimentRangeByHashFilter>,
}

impl<'de> Deserialize<'de> for GuildExperimentPopulationFilters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(GuildExperimentPopulationFiltersVisitor)
    }
}

struct GuildExperimentPopulationFiltersVisitor;

impl<'de> Visitor<'de> for GuildExperimentPopulationFiltersVisitor {
    type Value = GuildExperimentPopulationFilters;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut filters = GuildExperimentPopulationFilters::default();

        let v: serde_json::Value = seq.next_element()?.unwrap();

        let arr = v.as_array().unwrap();
        let murmur_hash = arr
            .first()
            .and_then(|v| v.as_number())
            .and_then(|n| n.as_u64())
            .unwrap();

        /*
        guild_has_feature = Ok(1604612045)
        guild_id_range = Ok(2404720969)
        guild_age_range_days = Ok(3730341874)
        guild_member_count_range = Ok(2918402255)
        guild_ids = Ok(3013771838)
        guild_hub_types = Ok(4148745523)
        guild_has_vanity_url = Ok(188952590)
        guild_in_range_by_hash = Ok(2294888943)
         */

        match murmur_hash {
            1604612045 => {
                let v = arr
                    .last()
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.first())
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.last())
                    .unwrap();

                let features = GuildExperimentPopulationFeatureFilter::deserialize(v).unwrap();
                filters.guild_has_feature = Some(features);
            }
            2404720969 => {
                let range =
                    GuildExperimentPopulationRangeFilter::deserialize(arr.last().unwrap()).unwrap();
                filters.guild_id_range = Some(range);
            }
            3730341874 => {
                let range =
                    GuildExperimentPopulationRangeFilter::deserialize(arr.last().unwrap()).unwrap();
                filters.guild_age_range_days = Some(range);
            }
            2918402255 => {
                let range =
                    GuildExperimentPopulationRangeFilter::deserialize(arr.last().unwrap()).unwrap();
                filters.guild_member_count_range = Some(range);
            }
            3013771838 => {
                let ids =
                    GuildExperimentPopulationIdFilter::deserialize(arr.last().unwrap()).unwrap();
                filters.guild_ids = Some(ids);
            }
            4148745523 => {
                let hub_types =
                    GuildExperimentPopulationHubTypeFilter::deserialize(arr.last().unwrap())
                        .unwrap();
                filters.guild_hub_types = Some(hub_types);
            }
            188952590 => {
                let has_vanity_url =
                    GuildExperimentPopulationVanityUrlFilter::deserialize(arr.last().unwrap())
                        .unwrap();
                filters.guild_has_vanity_url = Some(has_vanity_url);
            }
            2294888943 => {
                // TODO: I'm not sure how this is expected to be parsed, see commented out tests below.
                // let range_by_hash =
                //     GuildExperimentRangeByHashFilter::deserialize(arr.last().unwrap()).unwrap();
                // filters.guild_in_range_by_hash = Some(range_by_hash);
            }
            _ => {}
        }

        Ok(filters)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperimentPopulationFeatureFilter {
    pub guild_features: GuildFeaturesList,
}

impl<'de> Deserialize<'de> for GuildExperimentPopulationFeatureFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let features = GuildFeaturesList::deserialize(deserializer)?;
        Ok(GuildExperimentPopulationFeatureFilter {
            guild_features: features,
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperimentPopulationRangeFilter {
    pub min_id: Option<Snowflake>,
    pub max_id: Option<Snowflake>,
}

impl<'de> Deserialize<'de> for GuildExperimentPopulationRangeFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(GuildExperimentPopulationRangeFilterVisitor)
    }
}

struct GuildExperimentPopulationRangeFilterVisitor;

impl<'de> Visitor<'de> for GuildExperimentPopulationRangeFilterVisitor {
    type Value = GuildExperimentPopulationRangeFilter;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let min_id = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let max_id = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        Ok(GuildExperimentPopulationRangeFilter { min_id, max_id })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuildExperimentPopulationIdFilter {
    #[serde(flatten)]
    pub guild_ids: Vec<Snowflake>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuildExperimentPopulationHubTypeFilter {
    #[serde(flatten)]
    pub guild_hub_types: Vec<HubType>,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct GuildExperimentPopulationVanityUrlFilter {
    #[serde(flatten)]
    pub guild_has_vanity_url: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GuildExperimentRangeByHashFilter {
    pub hash_key: u64,
    pub target: u64,
}

impl<'de> Deserialize<'de> for GuildExperimentRangeByHashFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(GuildExperimentRangeByHashFilterVisitor)
    }
}

struct GuildExperimentRangeByHashFilterVisitor;

impl<'de> Visitor<'de> for GuildExperimentRangeByHashFilterVisitor {
    type Value = GuildExperimentRangeByHashFilter;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let hash_key = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let target = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        Ok(GuildExperimentRangeByHashFilter { hash_key, target })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GuildExperimentBucketOverride {
    pub b: u32,
    pub k: Vec<Snowflake>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_user_experiment_serde() {
//         let data = "[4130837190,0,10,-1,0,1932,0,0]";
//         let user_experiment: UserExperiment = serde_json::from_str(data).unwrap();
//         assert_eq!(user_experiment.hash, 4130837190);
//         assert_eq!(user_experiment.revision, 0);
//         assert_eq!(user_experiment.bucket, 10);
//         assert_eq!(user_experiment.r#override, -1);
//         assert_eq!(user_experiment.population, 0);
//         assert_eq!(user_experiment.hash_result, 1932);
//         assert_eq!(user_experiment.aa_mode, 0);
//         assert_eq!(user_experiment.trigger_debugging, 0);
//         let serialized = serde_json::to_string(&user_experiment).unwrap();
//         assert_eq!(serialized, data)
//     }
//
//     #[test]
//     fn test_guild_experiment_serde() {
//         // let data = r#"[1405831955,"2021-06_guild_role_subscriptions",0,[[[[-1,[{"s": 7200,"e": 10000}]],[1,[{"s": 0,"e": 7200}]]],[[2294888943,[[2690752156, 1405831955],[1982804121, 10000]]]]]],[],[[[[[1,[{"s": 0,"e": 10000}]]],[[1604612045, [[1183251248, ["GUILD_ROLE_SUBSCRIPTIONS"]]]]]]]],null,null,0,0]"#;
//         let data = r#"[
//   1405831955,
//   "2021-06_guild_role_subscriptions",
//   0,
//   [
//     [
//       [
//         [
//           -1,
//           [
//             {
//               "s": 7200,
//               "e": 10000
//             }
//           ]
//         ],
//         [
//           1,
//           [
//             {
//               "s": 0,
//               "e": 7200
//             }
//           ]
//         ]
//       ],
//       [
//         [
//           2294888943,
//           [
//             [2690752156, 1405831955],
//             [1982804121, 10000]
//           ]
//         ]
//       ]
//     ]
//   ],
//   [],
//   [
//     [
//       [
//         [
//           [
//             1,
//             [
//               {
//                 "s": 0,
//                 "e": 10000
//               }
//             ]
//           ]
//         ],
//         [[1604612045, [[1183251248, ["GUILD_ROLE_SUBSCRIPTIONS"]]]]]
//       ]
//     ]
//   ],
//   null,
//   null,
//   0,
//   0
// ]"#;
//         let jd = &mut serde_json::Deserializer::from_str(data);
//         let result: Result<GuildExperiment, _> = serde_path_to_error::deserialize(jd);
//         if let Err(e) = result {
//             println!("{}", e);
//         }
//
//         let guild_experiment: GuildExperiment = serde_json::from_str(data).unwrap();
//         println!("{:#?}", guild_experiment);
//     }
//
//     #[test]
//     fn test_murmur3() {
//         let guild_has_feature =
//             murmur3::murmur3_32(&mut std::io::Cursor::new("guild_has_feature"), 0);
//         println!("guild_has_feature = {:?}", guild_has_feature);
//         let guild_id_range = murmur3::murmur3_32(&mut std::io::Cursor::new("guild_id_range"), 0);
//         println!("guild_id_range = {:?}", guild_id_range);
//         let guild_age_range_days =
//             murmur3::murmur3_32(&mut std::io::Cursor::new("guild_age_range_days"), 0);
//         println!("guild_age_range_days = {:?}", guild_age_range_days);
//         let guild_member_count_range =
//             murmur3::murmur3_32(&mut std::io::Cursor::new("guild_member_count_range"), 0);
//         println!("guild_member_count_range = {:?}", guild_member_count_range);
//         let guild_ids = murmur3::murmur3_32(&mut std::io::Cursor::new("guild_ids"), 0);
//         println!("guild_ids = {:?}", guild_ids);
//         let guild_hub_types = murmur3::murmur3_32(&mut std::io::Cursor::new("guild_hub_types"), 0);
//         println!("guild_hub_types = {:?}", guild_hub_types);
//         let guild_has_vanity_url =
//             murmur3::murmur3_32(&mut std::io::Cursor::new("guild_has_vanity_url"), 0);
//         println!("guild_has_vanity_url = {:?}", guild_has_vanity_url);
//         let guild_in_range_by_hash =
//             murmur3::murmur3_32(&mut std::io::Cursor::new("guild_in_range_by_hash"), 0);
//         println!("guild_in_range_by_hash = {:?}", guild_in_range_by_hash);
//     }
// }
