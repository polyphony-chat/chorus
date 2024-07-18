// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::entities::User;
use crate::types::Shared;
use crate::types::Snowflake;

use super::arc_rwlock_ptr_eq;

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

#[cfg(not(tarpaulin_include))]
impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.icon == other.icon
            && self.id == other.id
            && self.members == other.members
            && self.name == other.name
            && self.owner_user_id == other.owner_user_id
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamMember {
    pub membership_state: u8,
    pub permissions: Vec<String>,
    pub team_id: Snowflake,
    pub user: Shared<User>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for TeamMember {
    fn eq(&self, other: &Self) -> bool {
        self.membership_state == other.membership_state
            && self.permissions == other.permissions
            && self.team_id == other.team_id
            && arc_rwlock_ptr_eq(&self.user, &other.user)
    }
}
