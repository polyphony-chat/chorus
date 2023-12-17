use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_option_number_from_string, deserialize_string_from_number};
use std::fmt::Debug;

use crate::types::utils::Snowflake;

#[cfg(feature = "client")]
use chorus_macros::{Composite, Updateable};

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord.com/developers/docs/topics/permissions#role-object>
pub struct RoleObject {
    pub id: Snowflake,
    pub name: String,
    pub color: f64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u16,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
/// See <https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure>
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
    #[derive(Debug, Default, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
    /// Permissions limit what users of certain roles can do on a Guild to Guild basis.
    ///
    /// # Reference:
    /// See <https://discord.com/developers/docs/topics/permissions#permissions>
    pub struct PermissionFlags: u64 {
        /// Allows creation of instant invites
        const CREATE_INSTANT_INVITE = 1 << 0;
        /// Allows kicking members
        const KICK_MEMBERS = 1 << 1;
        /// Allows banning members
        const BAN_MEMBERS = 1 << 2;
        /// Allows all permissions and bypasses channel permission overwrites
        const ADMINISTRATOR = 1 << 3;
        /// Allows management and editing of channels
        const MANAGE_CHANNELS = 1 << 4;
        /// Allows management and editing of the guild and guild settings
        const MANAGE_GUILD = 1 << 5;
        /// Allows for the addition of reactions to messages
        const ADD_REACTIONS = 1 << 6;
        /// Allows viewing of the audit log
        const VIEW_AUDIT_LOG = 1 << 7;
        /// Allows using priority speaker in a voice channel
        const PRIORITY_SPEAKER = 1 << 8;
        /// Allows the user to go live and share their screen
        const STREAM = 1 << 9;
        /// Allows guild members to view a channel, which includes reading messages in text channels and joining voice channels
        const VIEW_CHANNEL = 1 << 10;
        /// Allows sending messages in a channel and creating threads in a forum (does not allow sending messages in threads)
        const SEND_MESSAGES = 1 << 11;
        /// Allows sending /tts messages
        const SEND_TTS_MESSAGES = 1 << 12;
        /// Allows deletion of other users' messages
        const MANAGE_MESSAGES = 1 << 13;
        /// Links sent by users with this permission will be auto-embedded
        const EMBED_LINKS = 1 << 14;
        /// Allows uploading images and files
        const ATTACH_FILES = 1 << 15;
        /// Allows reading of message history
        const READ_MESSAGE_HISTORY = 1 << 16;
        /// Allows using the @everyone tag to notify all users in a channel, and the @here tag to notify all online users in a channel
        const MENTION_EVERYONE = 1 << 17;
        /// Allows the usage of custom emojis from other servers
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        /// Allows viewing guild insights
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        /// Allows joining of a voice channel
        const CONNECT = 1 << 20;
        /// Allows speaking in a voice channel
        const SPEAK = 1 << 21;
        /// Allows muting members in a voice channel
        const MUTE_MEMBERS = 1 << 22;
        /// Allows deafening of members in a voice channel
        const DEAFEN_MEMBERS = 1 << 23;
        /// Allows moving of members between voice channels
        const MOVE_MEMBERS = 1 << 24;
        /// Allows using voice activity (VAD = voice-activity-detection) in a voice channel
        const USE_VAD = 1 << 25;
        /// Allows modification of own nickname
        const CHANGE_NICKNAME = 1 << 26;
        /// Allows modification of other users' nicknames
        const MANAGE_NICKNAMES = 1 << 27;
        /// Allows management and editing of roles
        const MANAGE_ROLES = 1 << 28;
        /// Allows management and editing of webhooks
        const MANAGE_WEBHOOKS = 1 << 29;
        /// Allows management and editing of emojis, stickers, and soundboard sounds
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        /// Allows members to use application commands, including slash commands and context menu commands.
        const USE_APPLICATION_COMMANDS = 1 << 31;
        /// Allows requesting to speak in stage channels. (*This permission is under active development and may be changed or removed.*)
        const REQUEST_TO_SPEAK = 1 << 32;
        /// Allows creating, editing, and deleting scheduled events
        const MANAGE_EVENTS = 1 << 33;
        /// Allows deleting and archiving threads, and viewing all private threads
        const MANAGE_THREADS = 1 << 34;
        /// Allows creating public and announcement threads
        const CREATE_PUBLIC_THREADS = 1 << 35;
        /// Allows creating private threads
        const CREATE_PRIVATE_THREADS = 1 << 36;
        /// Allows the usage of custom stickers from other servers
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows sending messages in threads
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        /// Allows using Activities in a voice channel
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        /// Allows timing out users to prevent them from sending or reacting to messages in chat and threads, and from speaking in voice and stage channels
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows viewing role subscription insights
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        /// Allows using the soundboard in a voice channel
        const USE_SOUNDBOARD = 1 << 42;
        /// Allows using custom soundboard sounds from other servers
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        /// Allows sending voice messages
        const SEND_VOICE_MESSAGES = 1 << 46;
        /// Allows creating encrypted voice channels
        const MANAGE_ENCRYPTION = 1 << 63;
    }
}

impl PermissionFlags {
    /// Returns if the PermissionFlags object has specific permissions
    ///
    /// # Notes
    /// Note that if the object has the [PermissionFlags::ADMINISTRATOR] permission, this always returns true
    pub fn has_permission(&self, permission: PermissionFlags) -> bool {
        self.contains(permission) || self.contains(PermissionFlags::ADMINISTRATOR)
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.bits().to_string()
    }

    /// Creates a String of Permissions from a given [`Vec`] of [`PermissionFlags`].
    ///
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
            permissions |= flag.clone();
        }
        permissions.to_string()
    }
}
