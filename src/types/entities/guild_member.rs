use serde::{Deserialize, Serialize};

use crate::types::{entities::PublicUser, Snowflake};

#[derive(Debug, Deserialize, Default, Serialize, Clone, PartialEq, Eq)]
pub struct GuildMember {
    pub user: Option<PublicUser>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: Option<i32>,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
}
