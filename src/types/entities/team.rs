use serde::{Deserialize, Serialize};

use crate::types::entities::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    pub icon: Option<String>,
    pub id: u64,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamMember {
    pub membership_state: u8,
    pub permissions: Vec<String>,
    pub team_id: u64,
    pub user: User,
}
