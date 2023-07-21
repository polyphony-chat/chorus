use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::Snowflake;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct UserModifySchema {
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub accent_color: Option<u64>,
    pub banner: Option<String>,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
    pub code: Option<String>,
    pub email: Option<String>,
    pub discriminator: Option<i16>,
}

/// # Attributes:
/// - recipients: The users to include in the private channel
/// - access_tokens: The access tokens of users that have granted your app the `gdm.join` scope. Only usable for OAuth2 requests (which can only create group DMs).
/// - nicks: A mapping of user IDs to their respective nicknames. Only usable for OAuth2 requests (which can only create group DMs).
///
/// # Reference:
/// Read: <https://discord-userdoccers.vercel.app/resources/channel#json-params>
#[derive(Debug, Deserialize, Serialize)]
pub struct PrivateChannelCreateSchema {
    pub recipients: Option<Vec<Snowflake>>,
    pub access_tokens: Option<Vec<String>>,
    pub nicks: Option<HashMap<Snowflake, String>>,
}
