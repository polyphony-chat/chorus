use std::sync::{Arc, RwLock};

use chorus_macros::Updateable;
use serde::{Deserialize, Serialize};

use crate::gateway::Updateable;
use crate::types::entities::User;
use crate::types::Snowflake;

#[derive(Debug, Clone, Deserialize, Serialize, Default, Updateable)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/emoji#emoji-object>
pub struct Emoji {
    pub id: Snowflake,
    pub name: Option<String>,
    #[cfg(feature = "sqlx")]
    pub roles: Option<sqlx::types::Json<Vec<Snowflake>>>,
    #[cfg(not(feature = "sqlx"))]
    pub roles: Option<Vec<Snowflake>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Arc<RwLock<User>>>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
