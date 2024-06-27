use serde::{Deserialize, Serialize};
use crate::types::{ApplicationCommand, AuditLogActionType, AuditLogEntry, AutoModerationRule, Channel, GuildScheduledEvent, Integration, Snowflake, User, Webhook};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetAuditLogsQuery {
    pub before: Option<Snowflake>,
    pub after: Option<Snowflake>,
    pub limit: Option<u8>,
    pub user_id: Option<Snowflake>,
    pub action_type: Option<AuditLogActionType>
}