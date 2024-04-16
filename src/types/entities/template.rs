// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::gateway::Shared;
use crate::types::{
    entities::{Guild, User},
    utils::Snowflake,
};

/// See <https://docs.spacebar.chat/routes/#cmp--schemas-template>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct GuildTemplate {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub usage_count: Option<u64>,
    pub creator_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub creator: Shared<User>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source_guild_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub source_guild: Vec<Shared<Guild>>,
    // Unsure how a {recursive: Guild} looks like, might be a Vec?
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub serialized_source_guild: Vec<Shared<Guild>>,
}
