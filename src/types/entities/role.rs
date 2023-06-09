use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

use crate::types::utils::Snowflake;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See https://discord.com/developers/docs/topics/permissions#role-object
pub struct RoleObject {
    pub id: Snowflake,
    pub name: String,
    pub color: f64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u16,
    #[serde(default)]
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    #[cfg(feature = "sqlx")]
    pub tags: Option<sqlx::types::Json<RoleTags>>,
    #[cfg(not(feature = "sqlx"))]
    pub tags: Option<RoleTags>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: Snowflake,
    pub tier_name: String,
    pub total_months_subscribed: u32,
    pub is_renewal: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
/// See https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
pub struct RoleTags {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub bot_id: Option<usize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub integration_id: Option<usize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub subscription_listing_id: Option<usize>,
    // These use the bad bool format, "Tags with type null represent booleans. They will be present and set to null if they are "true", and will be not present if they are "false"."
    // premium_subscriber: bool,
    // available_for_purchase: bool,
    // guild_connections: bool,
}

bitflags! {
    #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    pub struct PermissionFlags: u64 {
        const CREATE_INSTANT_INVITE = 1 << 0;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        const USE_APPLICATION_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        const MANAGE_EVENTS = 1 << 33;
        const MANAGE_THREADS = 1 << 34;
        const CREATE_PUBLIC_THREADS = 1 << 35;
        const CREATE_PRIVATE_THREADS = 1 << 36;
        const USE_EXTERNAL_STICKERS = 1 << 37;
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        const MODERATE_MEMBERS = 1 << 40;
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        const USE_SOUNDBOARD = 1 << 42;
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        const SEND_VOICE_MESSAGES = 1 << 46;
    }
}

impl PermissionFlags {
    pub fn has_permission(&self, permission: PermissionFlags) -> bool {
        self.contains(permission) || self.contains(PermissionFlags::ADMINISTRATOR)
    }

    pub fn to_string(&self) -> String {
        self.bits().to_string()
    }

    /// Creates a String of Permissions from a given [`Vec`] of [`PermissionFlags`].
    /// # Example:
    /// ```
    /// use chorus::types::{PermissionFlags};
    ///
    /// let mut vector: Vec<PermissionFlags> = Vec::new();
    /// vector.push(PermissionFlags::MUTE_MEMBERS);
    /// vector.push(PermissionFlags::DEAFEN_MEMBERS);
    ///
    /// let permissions: String = PermissionFlags::from_vec(vector);
    ///
    /// println!("The permissions string is {}.", permissions);
    /// assert_eq!(permissions, "12582912".to_string());
    /// ```
    pub fn from_vec(flags: Vec<PermissionFlags>) -> String {
        let mut permissions: PermissionFlags = Default::default();
        for flag in flags.iter() {
            permissions = permissions | flag.clone();
        }
        permissions.to_string()
    }
}
