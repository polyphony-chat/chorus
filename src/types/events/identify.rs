// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::GatewayIdentifyPresenceUpdate;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, WebSocketEvent)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: GatewayIdentifyConnectionProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i16>,
    //default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<(i32, i32)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<GatewayIdentifyPresenceUpdate>,
    // What is the difference between these two?
    // Intents is documented, capabilities is used in users
    // I wonder if these are interchangeable...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<GatewayIntents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<GatewayCapabilities>,
}

impl Default for GatewayIdentifyPayload {
    fn default() -> Self {
        Self::common()
    }
}

impl GatewayIdentifyPayload {
    /// Uses the most common, 25% data along with client capabilities
    ///
    /// Basically pretends to be an official client on Windows 10, with Chrome 113.0.0.0
    pub fn common() -> Self {
        Self {
            token: "".to_string(),
            properties: GatewayIdentifyConnectionProps::default(),
            compress: Some(false),
            large_threshold: None,
            shard: None,
            presence: None,
            intents: None,
            capabilities: Some(GatewayCapabilities::default()),
        }
    }
}

impl GatewayIdentifyPayload {
    /// Creates an identify payload with the same default capabilities as the official client
    pub fn default_w_client_capabilities() -> Self {
        Self {
            capabilities: Some(GatewayCapabilities::default()),
            ..Self::default()
        }
    }

    /// Creates an identify payload with all possible capabilities
    pub fn default_w_all_capabilities() -> Self {
        Self {
            capabilities: Some(GatewayCapabilities::all()),
            ..Self::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, WebSocketEvent)]
#[serde_as]
pub struct GatewayIdentifyConnectionProps {
    /// Almost always sent
    ///
    /// ex: "Linux", "Windows", "Mac OS X"
    ///
    /// ex (mobile): "Windows Mobile", "iOS", "Android", "BlackBerry"
    #[serde(default)]
    pub os: String,
    /// Almost always sent
    ///
    /// ex: "Firefox", "Chrome", "Opera Mini", "Opera", "Blackberry", "Facebook Mobile", "Chrome iOS", "Mobile Safari", "Safari", "Android Chrome", "Android Mobile", "Edge", "Konqueror", "Internet Explorer", "Mozilla", "Discord Client"
    #[serde(default)]
    pub browser: String,
    /// Sometimes not sent, acceptable to be ""
    ///
    /// Speculation:
    /// Only sent for mobile devices
    ///
    /// ex: "BlackBerry", "Windows Phone", "Android", "iPhone", "iPad", ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub device: Option<String>,
    /// Almost always sent, most commonly en-US
    ///
    /// ex: "en-US"
    #[serde(default)]
    pub system_locale: String,
    /// Almost always sent
    ///
    /// ex: any user agent, most common is "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36"
    #[serde(default)]
    pub browser_user_agent: String,
    /// Almost always sent
    ///
    /// ex: "113.0.0.0"
    #[serde(default)]
    pub browser_version: String,
    /// Sometimes not sent, acceptable to be ""
    ///
    /// ex: "10" (For os = "Windows")
    #[serde_as(as = "NoneAsEmptyString")]
    pub os_version: Option<String>,
    /// Sometimes not sent, acceptable to be ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer: Option<String>,
    /// Sometimes not sent, acceptable to be ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub referring_domain: Option<String>,
    /// Sometimes not sent, acceptable to be ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer_current: Option<String>,
    /// Almost always sent, most commonly "stable"
    #[serde(default)]
    pub release_channel: String,
    /// Almost always sent, identifiable if default is 0, should be around 199933
    #[serde(default)]
    pub client_build_number: u64,
    //pub client_event_source: Option<?>
}

impl Default for GatewayIdentifyConnectionProps {
    /// Uses the most common, 25% data
    fn default() -> Self {
        Self::common()
    }
}

impl GatewayIdentifyConnectionProps {
    /// Returns a minimal, least data possible default
    fn minimal() -> Self {
        Self {
            os: String::new(),
            browser: String::new(),
            device: None,
            system_locale: String::from("en-US"),
            browser_user_agent: String::new(),
            browser_version: String::new(),
            os_version: None,
            referrer: None,
            referring_domain: None,
            referrer_current: None,
            release_channel: String::from("stable"),
            client_build_number: 0,
        }
    }

    /// Returns the most common connection props so we can't be tracked
    pub fn common() -> Self {
        Self {
            // See https://www.useragents.me/#most-common-desktop-useragents
            // 25% of the web
            //default.browser_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36".to_string();
            browser: String::from("Chrome"),
            browser_version: String::from("126.0.0.0"),
            system_locale: String::from("en-US"),
            os: String::from("Windows"),
            os_version: Some(String::from("10")),
            client_build_number: 222963,
            release_channel: String::from("stable"),
            ..Self::minimal()
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.sex/topics/gateway#gateway-capabilities>
    pub struct GatewayCapabilities: u64 {
        /// Removes the notes field from the Ready event,
        const LAZY_USER_NOTES = 1 << 0;
        /// Prevents member/presence syncing and Presence Update events for implicit relationships
        const NO_AFFINE_USER_IDS = 1 << 1;
        /// Enables versioned read states, changing the read_state field in the Ready event to an object, allowing it to be cached when re-identifying
        const VERSIONED_READ_STATES = 1 << 2;
        /// Enables versioned user guild settings, changing the user_guild_settings field in the Ready event to an object, allowing it to be cached when re-identifying
        const VERSIONED_USER_GUILD_SETTINGS = 1 << 3;
        /// Dehydrates the Ready payload, moving all user objects to the users field and replacing them in various places in the payload with user_id or recipient_id, and merging the members fields of all guilds into a single merged_members field
        const DEDUPE_USER_OBJECTS = 1 << 4;
        /// 1	Separates the Ready payload into two parts (Ready and Ready Supplemental) allowing the client to receive the Ready payload faster and then receive the rest of the payload later
        const PRIORITIZED_READY_PAYLOAD = 1 << 5;
        /// Changes the populations entry of guild_experiments in the Ready event to be an array of populations rather than a single population
        const MULTIPLE_GUILD_EXPERIMENT_POPULATIONS = 1 << 6;
        /// Includes read states tied to non-channel resources (e.g. guild scheduled events and notification center) in the read_states field of the Ready event
        const NON_CHANNEL_READ_STATES = 1 << 7;
        /// Enables auth token refresh, allowing the client to optionally receive a new auth token in the auth_token field of the Ready event
        const AUTH_TOKEN_REFRESH = 1 << 8;
        /// Removes the user_settings field from the Ready event, and prevents User Settings Update events; the user_settings_proto field and User Settings Proto Update event is used instead
        const USER_SETTINGS_PROTO = 1 << 9;
        /// Enables client state caching v2
        const CLIENT_STATE_V2 = 1 << 10;
        /// Enables passive guild updates, allowing the client to receive Passive Update v1 events instead of Channel Unreads Update events for guilds it is not subscribed to
        const PASSIVE_GUILD_UPDATE = 1 << 11;
        /// Connects the client to all pre-existing calls upon connecting to the Gateway; this means clients will receive Call Create events for all calls created before the Gateway connection was established without needing to send a Request Call Connect first
        const AUTO_CALL_CONNECT = 1 << 12;
        /// Debounces message reaction events, preventing the client from receiving multiple Message Reaction Add events for the same message within a short period of time; clients will receive a single Message Reaction Add Many event instead
        const DEBOUNCE_MESSAGE_REACTIONS = 1 << 13;
        /// Enables passive guild updates v2, allowing the client to receive Passive Update v2 events instead of Channel Unreads Update events for guilds it is not subscribed to
        const PASSIVE_GUILD_UPDATE_V2 = 1 << 14;
    }
}

impl Default for GatewayCapabilities {
    fn default() -> Self {
        Self::NO_AFFINE_USER_IDS
        | Self::VERSIONED_READ_STATES
        | Self::VERSIONED_USER_GUILD_SETTINGS
        | Self::DEDUPE_USER_OBJECTS
        | Self::PRIORITIZED_READY_PAYLOAD
        | Self::MULTIPLE_GUILD_EXPERIMENT_POPULATIONS
        | Self::NON_CHANNEL_READ_STATES
        | Self::AUTH_TOKEN_REFRESH
        | Self::USER_SETTINGS_PROTO
        | Self::PASSIVE_GUILD_UPDATE
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.sex/topics/gateway#gateway-intents>
    pub struct GatewayIntents: u64 {
        ///   - GUILD_CREATE
        ///   - GUILD_UPDATE
        ///   - GUILD_DELETE
        ///   - GUILD_ROLE_CREATE
        ///   - GUILD_ROLE_UPDATE
        ///   - GUILD_ROLE_DELETE
        ///   - CHANNEL_CREATE
        ///   - CHANNEL_UPDATE
        ///   - CHANNEL_DELETE
        ///   - VOICE_CHANNEL_STATUS_UPDATE
        ///   - CHANNEL_PINS_UPDATE
        ///   - THREAD_CREATE
        ///   - THREAD_UPDATE
        ///   - THREAD_DELETE
        ///   - THREAD_LIST_SYNC
        ///   - THREAD_MEMBER_UPDATE
        ///   - THREAD_MEMBERS_UPDATE ¹
        ///   - STAGE_INSTANCE_CREATE
        ///   - STAGE_INSTANCE_UPDATE
        ///   - STAGE_INSTANCE_DELETE
        const GUILDS = 1 << 0;
        ///   - GUILD_MEMBER_ADD
        ///   - GUILD_MEMBER_UPDATE
        ///   - GUILD_MEMBER_REMOVE
        ///   - THREAD_MEMBERS_UPDATE ¹
        const GUILD_MEMBERS = 1 << 1;
        ///  - GUILD_AUDIT_LOG_ENTRY_CREATE
        ///   - GUILD_BAN_ADD
        ///   - GUILD_BAN_REMOVE
        const GUILD_MODERATION = 1 << 2;
        ///   - GUILD_EMOJIS_UPDATE
        ///   - GUILD_STICKERS_UPDATE
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
        ///   - GUILD_INTEGRATIONS_UPDATE
        ///   - INTEGRATION_CREATE
        ///   - INTEGRATION_UPDATE
        ///   - INTEGRATION_DELETE
        const GUILD_INTEGRATIONS = 1 << 4;
        ///   - WEBHOOKS_UPDATE
        const GUILD_WEBHOOKS = 1 << 5;
        ///   - INVITE_CREATE
        ///   - INVITE_DELETE
        const GUILD_INVITES = 1 << 6;
        ///   - VOICE_STATE_UPDATE
        ///   - VOICE_CHANNEL_EFFECT_SEND
        const GUILD_VOICE_STATES = 1 << 7;
        ///   - PRESENCE_UPDATE
        const GUILD_PRESENCES = 1 << 8;
        ///  - MESSAGE_CREATE
        ///   - MESSAGE_UPDATE
        ///   - MESSAGE_DELETE
        ///   - MESSAGE_DELETE_BULK
        const GUILD_MESSAGES = 1 << 9;
        ///   - MESSAGE_REACTION_ADD
        ///   - MESSAGE_REACTION_ADD_MANY
        ///   - MESSAGE_REACTION_REMOVE
        ///   - MESSAGE_REACTION_REMOVE_ALL
        ///   - MESSAGE_REACTION_REMOVE_EMOJI
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        ///   - TYPING_START
        const GUILD_MESSAGE_TYPING = 1 << 11;
        ///   - MESSAGE_CREATE
        ///   - MESSAGE_UPDATE
        ///   - MESSAGE_DELETE
        ///   - CHANNEL_PINS_UPDATE
        const DIRECT_MESSAGES = 1 << 12;
        ///   - MESSAGE_REACTION_ADD
        ///   - MESSAGE_REACTION_ADD_MANY
        ///   - MESSAGE_REACTION_REMOVE
        ///   - MESSAGE_REACTION_REMOVE_ALL
        ///   - MESSAGE_REACTION_REMOVE_EMOJI
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        ///   - TYPING_START
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        /// Unique privileged intent that isn't directly associated with any Gateway events. Instead, access to MESSAGE_CONTENT permits users to receive message content data across the APIs.
        /// # Reference
        /// See <https://docs.discord.sex/topics/gateway#message-content-intent>
        const MESSAGE_CONTENT = 1 << 15;
        ///   - GUILD_SCHEDULED_EVENT_CREATE
        ///   - GUILD_SCHEDULED_EVENT_UPDATE
        ///   - GUILD_SCHEDULED_EVENT_DELETE
        ///   - GUILD_SCHEDULED_EVENT_USER_ADD
        ///   - GUILD_SCHEDULED_EVENT_USER_REMOVE
        const GUILD_SCHEDULE_EVENTS = 1 << 16;
        ///   - EMBEDDED_ACTIVITY_UPDATE_V2
        const GUILD_EMBEDDED_ACTIVITIES = 1 << 17;
        ///   - CHANNEL_CREATE
        ///   - CHANNEL_UPDATE
        ///   - CHANNEL_DELETE
        ///   - CHANNEL_RECIPIENT_ADD
        ///   - CHANNEL_RECIPIENT_REMOVE
        const PRIVATE_CHANNELS = 1 << 18;
        ///   - AUTO_MODERATION_RULE_CREATE
        ///   - AUTO_MODERATION_RULE_UPDATE
        ///   - AUTO_MODERATION_RULE_DELETE
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        ///   - AUTO_MODERATION_ACTION_EXECUTION
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        ///   - RELATIONSHIP_ADD
        ///   - RELATIONSHIP_UPDATE
        ///   - RELATIONSHIP_REMOVE
        ///   - GAME_RELATIONSHIP_ADD
        ///   - GAME_RELATIONSHIP_REMOVE
        const USER_RELATIONSHIPS = 1 << 22;
        ///   - PRESENCE_UPDATE
        const USER_PRESENCE = 1 << 23;
        ///   - MESSAGE_POLL_VOTE_ADD
        ///   - MESSAGE_POLL_VOTE_REMOVE
        const GUILD_MESSAGE_POLLS = 1 << 24;
        ///   - MESSAGE_POLL_VOTE_ADD
        ///   - MESSAGE_POLL_VOTE_REMOVE
        const DIRECT_MESSAGE_POLLS = 1 << 25;
        ///   - EMBEDDED_ACTIVITY_UPDATE_V2
        const DIRECT_EMBEDDED_ACTIVITES = 1 << 26;
    }
}