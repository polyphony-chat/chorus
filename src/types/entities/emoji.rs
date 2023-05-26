use serde::{Deserialize, Serialize};

use crate::types::entities::User;
use crate::types::{Guild, Snowflake};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    #[cfg(feature = "sqlx")]
    pub roles: Option<sqlx::types::Json<Vec<Snowflake>>>,
    #[cfg(not(feature = "sqlx"))]
    pub roles: Option<Vec<Snowflake>>,
    #[cfg(feature = "sqlx")]
    #[serde(skip)]
    pub guild_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub guild: Guild,
    #[cfg(feature = "sqlx")]
    #[serde(skip)]
    pub user_id: Option<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
