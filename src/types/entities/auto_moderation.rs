use crate::gateway::Shared;
#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::Updateable;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::utils::Snowflake;

#[cfg_attr(feature = "client", derive(Updateable))]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object>
pub struct AutoModerationRule {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub name: String,
    pub creator_id: Snowflake,
    pub event_type: AutoModerationRuleEventType,
    pub trigger_type: AutoModerationRuleTriggerType,
    pub trigger_metadata: Shared<AutoModerationRuleTriggerMetadata>,
    pub actions: Vec<Shared<AutoModerationAction>>,
    pub enabled: bool,
    pub exempt_roles: Vec<Snowflake>,
    pub exempt_channels: Vec<Snowflake>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types>
pub enum AutoModerationRuleEventType {
    #[default]
    MessageSend = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types>
pub enum AutoModerationRuleTriggerType {
    #[default]
    Keyword = 1,
    Spam = 3,
    KeywordPreset = 4,
    MentionSpam = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(untagged)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata>
pub enum AutoModerationRuleTriggerMetadata {
    ForKeyword(AutoModerationRuleTriggerMetadataForKeyword),
    ForKeywordPreset(AutoModerationRuleTriggerMetadataForKeywordPreset),
    ForMentionSpam(AutoModerationRuleTriggerMetadataForMentionSpam),
    #[default]
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata>
pub struct AutoModerationRuleTriggerMetadataForKeyword {
    pub keyword_filter: Vec<String>,
    pub regex_patterns: Vec<String>,
    pub allow_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata>
pub struct AutoModerationRuleTriggerMetadataForKeywordPreset {
    pub presets: Vec<AutoModerationRuleKeywordPresetType>,
    pub allow_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata>
pub struct AutoModerationRuleTriggerMetadataForMentionSpam {
    /// Max 50
    pub mention_total_limit: u8,
    pub mention_raid_protection_enabled: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-preset-types>
pub enum AutoModerationRuleKeywordPresetType {
    #[default]
    Profanity = 1,
    SexualContent = 2,
    Slurs = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object>
pub struct AutoModerationAction {
    #[serde(rename = "type")]
    pub action_type: AutoModerationActionType,
    pub metadata: Option<Shared<AutoModerationActionMetadata>>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-types>
pub enum AutoModerationActionType {
    #[default]
    BlockMessage = 1,
    SendAlertMessage = 2,
    Timeout = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(untagged)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata>
pub enum AutoModerationActionMetadata {
    ForBlockMessage(AutoModerationActionMetadataForBlockMessage),
    ForSendAlertMessage(AutoModerationActionMetadataForSendAlertMessage),
    ForTimeout(AutoModerationActionMetadataForTimeout),
    #[default]
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata>
pub struct AutoModerationActionMetadataForBlockMessage {
    pub custom_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata>
pub struct AutoModerationActionMetadataForSendAlertMessage {
    pub channel_id: Snowflake,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// See <https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata>
pub struct AutoModerationActionMetadataForTimeout {
    /// Max 2419200
    pub duration_seconds: u32,
}
