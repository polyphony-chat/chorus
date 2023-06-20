use serde::{Deserialize, Serialize};

use crate::types::{
    AutoModerationAction, AutoModerationRule, AutoModerationRuleTriggerType, Snowflake,
};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create
pub struct AutoModerationRuleCreate {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-update
pub struct AutoModerationRuleUpdate {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete
pub struct AutoModerationRuleDelete {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#auto-moderation-action-execution
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
