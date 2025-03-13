// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{
    ApplicationCommand, AuditLogActionType, AuditLogEntry, AutoModerationRule, Channel,
    GuildScheduledEvent, Integration, Snowflake, User, Webhook,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuditLogObject {
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub application_commands: Vec<ApplicationCommand>,
    pub auto_moderation_rules: Vec<AutoModerationRule>,
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub integrations: Vec<Integration>,
    pub threads: Vec<Channel>,
    pub users: Vec<User>,
    pub webhooks: Vec<Webhook>,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default,
)]
pub struct GetAuditLogsQuery {
    pub before: Option<Snowflake>,
    pub after: Option<Snowflake>,
    pub limit: Option<u8>,
    pub user_id: Option<Snowflake>,
    pub action_type: Option<AuditLogActionType>,
}
