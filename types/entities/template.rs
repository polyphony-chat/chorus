use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{Guild, User},
    utils::Snowflake,
};

/// See https://docs.spacebar.chat/routes/#cmp--schemas-template
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildTemplate {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub usage_count: Option<u64>,
    pub creator_id: Snowflake,
    pub creator: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source_guild_id: String,
    pub source_guild: Vec<Guild>, // Unsure how a {recursive: Guild} looks like, might be a Vec?
    pub serialized_source_guild: Vec<Guild>,
    id: Snowflake,
}
