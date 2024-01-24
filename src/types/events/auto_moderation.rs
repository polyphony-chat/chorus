use crate::types::{JsonField, SourceUrlField};
use chorus_macros::{JsonField, SourceUrlField};
use serde::{Deserialize, Serialize};

use crate::types::{
    AutoModerationAction, AutoModerationRule, AutoModerationRuleTriggerType, Snowflake,
    WebSocketEvent,
};

#[cfg(feature = "client")]
use super::UpdateMessage;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create>
pub struct AutoModerationRuleCreate {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

impl WebSocketEvent for AutoModerationRuleCreate {}

#[derive(Debug, Deserialize, Serialize, Default, Clone, JsonField, SourceUrlField)]
/// See <https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-update>
pub struct AutoModerationRuleUpdate {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
#[cfg(not(tarpaulin_include))]
impl UpdateMessage<AutoModerationRule> for AutoModerationRuleUpdate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.rule.id)
    }
}

impl WebSocketEvent for AutoModerationRuleUpdate {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete>
pub struct AutoModerationRuleDelete {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

impl WebSocketEvent for AutoModerationRuleDelete {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#auto-moderation-action-execution>
pub struct AutoModerationActionExecution {
    pub guild_id: Snowflake,
    pub action: AutoModerationAction,
    pub rule_id: Snowflake,
    pub rule_trigger_type: AutoModerationRuleTriggerType,
    pub user_id: Snowflake,
    pub channel_id: Option<Snowflake>,
    pub message_id: Option<Snowflake>,
    pub alert_system_message_id: Option<Snowflake>,
    pub content: Option<String>,
    pub matched_keyword: Option<String>,
    pub matched_content: Option<String>,
}

impl WebSocketEvent for AutoModerationActionExecution {}
