// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{JsonField, SourceUrlField, WebSocketEvent};
use chorus_macros::{JsonField, SourceUrlField, WebSocketEvent};
use serde::{Deserialize, Serialize};

use crate::types::{
    AutoModerationAction, AutoModerationRule, AutoModerationRuleTriggerType, Snowflake,
};

#[cfg(feature = "client")]
use super::UpdateMessage;

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-create>
pub struct AutoModerationRuleCreate {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

#[derive(
    Debug, Deserialize, Serialize, Default, Clone, JsonField, SourceUrlField, WebSocketEvent,
)]
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

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#auto-moderation-rule-delete>
pub struct AutoModerationRuleDelete {
    #[serde(flatten)]
    pub rule: AutoModerationRule,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
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
