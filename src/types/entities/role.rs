use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_option_number_from_string, deserialize_string_from_number};

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

#[derive(Debug)]
#[repr(u64)]
pub enum PermissionFlags {
    CreateInstantInvite = 0x0000000000000001,
    KickMembers = 0x0000000000000002,
    BanMembers = 0x0000000000000004,
    Administrator = 0x0000000000000008,
    ManageChannels = 0x0000000000000010,
    ManageGuild = 0x0000000000000020,
    AddReactions = 0x0000000000000040,
    ViewAuditLog = 0x0000000000000080,
    PrioritySpeaker = 0x0000000000000100,
    Stream = 0x0000000000000200,
    ViewChannel = 0x0000000000000400,
    SendMessages = 0x0000000000000800,
    SendTtsMessages = 0x0000000000001000,
    ManageMessages = 0x0000000000002000,
    EmbedLinks = 0x0000000000004000,
    AttachFiles = 0x0000000000008000,
    ReadMessageHistory = 0x0000000000010000,
    MentionEveryone = 0x0000000000020000,
    UseExternalEmojis = 0x0000000000040000,
    ViewGuildInsights = 0x0000000000080000,
    Connect = 0x0000000000100000,
    Speak = 0x0000000000200000,
    MuteMembers = 0x0000000000400000,
    DeafenMembers = 0x0000000000800000,
    MoveMembers = 0x0000000001000000,
    UseVad = 0x0000000002000000,
    ChangeNickname = 0x0000000004000000,
    ManageNicknames = 0x0000000008000000,
    ManageRoles = 0x0000000010000000,
    ManageWebhooks = 0x0000000020000000,
    ManageGuildExpressions = 0x0000000040000000,
    UseApplicationCommands = 0x0000000080000000,
    RequestToSpeak = 0x0000000100000000,
    ManageEvents = 0x0000000200000000,
    ManageThreads = 0x0000000400000000,
    CreatePublicThreads = 0x0000000800000000,
    CreatePrivateThreads = 0x0000001000000000,
    UseExternalStickers = 0x0000002000000000,
    SendMessagesInThreads = 0x0000004000000000,
    UseEmbeddedActivities = 0x0000008000000000,
    ModerateMembers = 0x0000010000000000,
    ViewCreatorMonetizationAnalytics = 0x0000020000000000,
    UseSoundboard = 0x0000040000000000,
    UseExternalSounds = 0x0000200000000000,
    SendVoiceMessages = 0x0000400000000000,
}

impl RoleObject {
    /// Checks if the role has a specific permission.
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission to check for.
    ///
    /// # Example
    ///
    /// ```
    /// use chorus::types;
    /// let mut role = types::RoleObject::default();
    /// let permission = types::PermissionFlags::ModerateMembers as u64 | types::PermissionFlags::UseSoundboard as u64;
    /// role.permissions = permission.to_string();
    /// assert_eq!(true, role.has_permission(types::PermissionFlags::ModerateMembers));
    /// assert_eq!(true, role.has_permission(types::PermissionFlags::UseSoundboard));
    /// ```
    pub fn has_permission(&self, permission: PermissionFlags) -> bool {
        if self.permissions.parse::<u64>().unwrap() & permission as u64 != 0 {
            return true;
        }
        false
    }
}
