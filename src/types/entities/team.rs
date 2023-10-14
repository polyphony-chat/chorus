use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::types::entities::User;
use crate::types::Snowflake;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Team {
    pub icon: Option<String>,
    pub id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Snowflake,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamMember {
    pub membership_state: u8,
    pub permissions: Vec<String>,
    pub team_id: Snowflake,
    pub user: Arc<RwLock<User>>,
}
