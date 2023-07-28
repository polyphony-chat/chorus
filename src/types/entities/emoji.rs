use serde::{Deserialize, Serialize};

use crate::types::entities::User;
use crate::types::Snowflake;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>,
    #[cfg(feature = "sqlx")]
    pub roles: Option<sqlx::types::Json<Vec<Snowflake>>>,
    #[cfg(not(feature = "sqlx"))]
    pub roles: Option<Vec<Snowflake>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
