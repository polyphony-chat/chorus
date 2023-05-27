use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{
    entities::{Application, Channel, Guild, User},
    utils::Snowflake,
};

/// See https://docs.spacebar.chat/routes/#cmp--schemas-webhook
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub webhook_type: WebhookType,
    pub name: String,
    pub avatar: String,
    pub token: String,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub application_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub source_guild: Option<Guild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize_repr, Deserialize_repr, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(i32)]
pub enum WebhookType {
    #[default]
    Incoming = 1,
    ChannelFollower = 2,
    Application = 3,
}
