use serde::{Deserialize, Serialize};

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
