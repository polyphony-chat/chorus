use serde::{Deserialize, Serialize};

use crate::{
    entities::{Application, Channel, Guild, User},
    utils::Snowflake,
};

/// See https://docs.spacebar.chat/routes/#cmp--schemas-webhook
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Webhook {
    #[serde(rename = "type")]
    pub webhook_type: i32,
    pub name: String,
    pub avatar: String,
    pub token: String,
    pub guild_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Guild>,
    pub channel_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    pub application_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    pub user_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub source_guild_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guild: Option<Guild>,
    pub id: Snowflake,
}
