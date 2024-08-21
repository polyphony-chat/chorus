// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)]
use super::option_vec_arc_rwlock_ptr_eq;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::utils::Snowflake;
use crate::types::{
    AutoModerationRuleTriggerType, IntegrationType, PermissionOverwriteType, Shared,
};
use crate::UInt64;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object>
pub struct AuditLogEntry {
    pub target_id: Option<String>,
    #[cfg(feature = "sqlx")]
    pub changes: sqlx::types::Json<Option<Vec<Shared<AuditLogChange>>>>,
    #[cfg(not(feature = "sqlx"))]
    pub changes: Option<Vec<Shared<AuditLogChange>>>,
    pub user_id: Option<Snowflake>,
    pub id: Snowflake,
    pub action_type: AuditLogActionType,
    #[cfg(feature = "sqlx")]
    pub options: Option<sqlx::types::Json<AuditEntryInfo>>,
    #[cfg(not(feature = "sqlx"))]
    pub options: Option<AuditEntryInfo>,
    pub reason: Option<String>,
}

impl PartialEq for AuditLogEntry {
    fn eq(&self, other: &Self) -> bool {
        self.target_id == other.target_id
            && self.user_id == other.user_id
            && self.id == other.id
            && self.action_type == other.action_type
            && compare_options(&self.options, &other.options)
            && self.reason == other.reason
            && compare_changes(&self.changes, &other.changes)
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(feature = "sqlx")]
fn compare_options(
    a: &Option<sqlx::types::Json<AuditEntryInfo>>,
    b: &Option<sqlx::types::Json<AuditEntryInfo>>,
) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => match (a.encode_to_string(), b.encode_to_string()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        },
        (None, None) => true,
        _ => false,
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(not(feature = "sqlx"))]
fn compare_options(a: &Option<AuditEntryInfo>, b: &Option<AuditEntryInfo>) -> bool {
    a == b
}

#[cfg(not(tarpaulin_include))]
#[cfg(feature = "sqlx")]
fn compare_changes(
    a: &sqlx::types::Json<Option<Vec<Shared<AuditLogChange>>>>,
    b: &sqlx::types::Json<Option<Vec<Shared<AuditLogChange>>>>,
) -> bool {
    match (a.encode_to_string(), b.encode_to_string()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(not(feature = "sqlx"))]
fn compare_changes(
    a: &Option<Vec<Shared<AuditLogChange>>>,
    b: &Option<Vec<Shared<AuditLogChange>>>,
) -> bool {
    option_vec_arc_rwlock_ptr_eq(a, b)
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See <https://discord.com/developers/docs/resources/audit-log#audit-log-change-object>
pub struct AuditLogChange {
    pub new_value: Option<serde_json::Value>,
    pub old_value: Option<serde_json::Value>,
    pub key: String,
}

#[derive(
    Default,
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference:
/// See <https://docs.discord.sex/resources/audit-log#audit-log-events>
pub enum AuditLogActionType {
    #[default]
    /// Guild settings were updated
    GuildUpdate = 1,
    /// Channel was created
    ChannelCreate = 10,
    /// Channel settings were updated
    ChannelUpdate = 11,
    /// Channel was deleted
    ChannelDelete = 12,
    /// Permission overwrite was added to a channel
    ChannelOverwriteCreate = 13,
    /// Permission overwrite was updated for a channel
    ChannelOverwriteUpdate = 14,
    /// Permission overwrite was deleted from a channel
    ChannelOverwriteDelete = 15,
    /// Member was removed from guild
    MemberKick = 20,
    /// Members were pruned from guild
    MemberPrune = 21,
    /// Member was banned from guild
    MemberBanAdd = 22,
    /// Member was unbanned from guild
    MemberBanRemove = 23,
    /// Member was updated in guild
    MemberUpdate = 24,
    /// Member was added or removed from a role
    MemberRoleUpdate = 25,
    /// Member was moved to a different voice channel
    MemberMove = 26,
    /// Member was disconnected from a voice channel
    MemberDisconnect = 27,
    /// Bot user was added to guild
    BotAdd = 28,
    /// Role was created
    RoleCreate = 30,
    /// Role was edited
    RoleUpdate = 31,
    /// Role was deleted
    RoleDelete = 32,
    /// Guild invite was created
    InviteCreate = 40,
    /// Guild invite was updated
    InviteUpdate = 41,
    /// Guild invite was deleted
    InviteDelete = 42,
    /// Webhook was created
    WebhookCreate = 50,
    /// Webhook properties or channel were updated
    WebhookUpdate = 51,
    /// Webhook was deleted
    WebhookDelete = 52,
    /// Emoji was created
    EmojiCreate = 60,
    /// Emoji name was updated
    EmojiUpdate = 61,
    /// Emoji was deleted
    EmojiDelete = 62,
    /// Single message was deleted
    MessageDelete = 72,
    /// Multiple messages were deleted
    MessageBulkDelete = 73,
    /// Message was pinned to a channel
    MessagePin = 74,
    /// Message was unpinned from a channel
    MessageUnpin = 75,
    /// Interaction was added to guild
    IntegrationCreate = 80,
    /// Integration was updated (e.g. its scopes were updated)
    IntegrationUpdate = 81,
    /// Integration was removed from guild
    IntegrationDelete = 82,
    /// Stage instance was created (stage channel becomes live)
    StageInstanceCreate = 83,
    /// Stage instance details were updated
    StageInstanceUpdate = 84,
    /// Stage instance was deleted (stage channel no longer live)
    StageInstanceDelete = 85,
    /// Sticker was created
    StickerCreate = 90,
    /// Sticker details were updated
    StickerUpdate = 91,
    /// Sticker was deleted
    StickerDelete = 92,
    /// Event was created
    GuildScheduledEventCreate = 100,
    /// Event was updated
    GuildScheduledEventUpdate = 101,
    /// Event was cancelled
    GuildScheduledEventDelete = 102,
    /// Thread was created in a channel
    ThreadCreate = 110,
    /// Thread was updated
    ThreadUpdate = 111,
    /// Thread was deleted
    ThreadDelete = 112,
    /// Permissions were updated for a command
    ApplicationCommandPermissionUpdate = 121,
    /// AutoMod rule created
    AutoModerationRuleCreate = 140,
    /// AutoMod rule was updated
    AutoModerationRuleUpdate = 141,
    /// AutoMod rule was deleted
    AutoModerationRuleDelete = 142,
    /// Message was blocked by AutoMod
    AutoModerationBlockMessage = 143,
    /// Message was flagged by AutoMod
    AutoModerationFlagToChannel = 144,
    /// Member was timed out by AutoMod
    AutoModerationUserCommunicationDisabled = 145,
    /// Member was quarantined by AutoMod
    AutoModerationQuarantineUser = 146,
    /// Creator monetization request was created
    CreatorMonetizationRequestCreated = 150,
    /// Creator monetization terms were accepted
    CreatorMonetizationTermsAccepted = 151,
    /// Onboarding prompt was created
    OnboardingPromptCreate = 163,
    /// Onboarding prompt was updated
    OnboardingPromptUpdate = 164,
    /// Onboarding prompt was deleted
    OnboardingPromptDelete = 165,
    /// Onboarding was created
    OnboardingCreate = 166,
    /// Onboarding was updated
    OnboardingUpdate = 167,
    /// Voice channel status was updated
    VoiceChannelStatusUpdate = 192,
    /// Voice channel status was deleted
    VoiceChannelStatusDelete = 193,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AuditEntryInfo {
    pub application_id: Option<Snowflake>,
    pub auto_moderation_rule_name: Option<String>,
    pub auto_moderation_rule_trigger_type: Option<AutoModerationRuleTriggerType>,
    pub channel_id: Option<Snowflake>,
    // #[serde(option_string)]
    pub count: Option<UInt64>,
    // #[serde(option_string)]
    pub delete_member_days: Option<UInt64>,
    /// The ID of the overwritten entity
    pub id: Option<Snowflake>,
    pub integration_type: Option<IntegrationType>,
    // #[serde(option_string)]
    pub members_removed: Option<UInt64>,
    // #[serde(option_string)]
    pub message_id: Option<UInt64>,
    pub role_name: Option<String>,
    #[serde(rename = "type")]
    pub overwrite_type: Option<PermissionOverwriteType>,
    pub status: Option<String>,
}
