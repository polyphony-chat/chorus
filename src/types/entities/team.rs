use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::entities::User;
use crate::types::Snowflake;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Team {
    pub icon: Option<String>,
    pub id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Snowflake,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TeamMember {
    pub membership_state: MembershipState,
    pub permissions: Vec<String>,
    pub team_id: Snowflake,
    pub user: User,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(i32)]
pub enum MembershipState {
    Invited = 1,
    Accepted = 2,
}
