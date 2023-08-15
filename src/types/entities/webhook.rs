use std::sync::{Arc, RwLock};

use chorus_macros::Updateable;
use serde::{Deserialize, Serialize};

use crate::gateway::Updateable;
use crate::types::{
    entities::{Guild, User},
    utils::Snowflake,
};

/// See <https://docs.spacebar.chat/routes/#cmp--schemas-webhook>
#[derive(Serialize, Deserialize, Debug, Default, Clone, Updateable)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Webhook {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub webhook_type: i32,
    pub name: String,
    pub avatar: String,
    pub token: String,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub application_id: Snowflake,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Arc<RwLock<User>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub source_guild: Option<Arc<RwLock<Guild>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
