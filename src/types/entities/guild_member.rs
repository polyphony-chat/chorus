use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::types::{entities::PublicUser, Snowflake};

#[derive(Debug, Deserialize, Default, Serialize, Clone)]
/// Represents a participating user in a guild.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/guild#guild-member-object>
pub struct GuildMember {
    pub user: Option<Arc<Mutex<PublicUser>>>,
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
