// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{Shared, Snowflake};

use super::{arc_rwlock_ptr_eq, PublicUser};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// See <https://discord-userdoccers.vercel.app/resources/user#relationship-structure>
pub struct Relationship {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
    pub nickname: Option<String>,
    pub user: Shared<PublicUser>,
    pub since: Option<DateTime<Utc>>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for Relationship {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.relationship_type == other.relationship_type
            && self.nickname == other.nickname
            && arc_rwlock_ptr_eq(&self.user, &other.user)
            && self.since == other.since
    }
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Clone,
    Default,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Copy,
    Hash,
)]
#[repr(u8)]
/// See <https://discord-userdoccers.vercel.app/resources/user#relationship-type>
pub enum RelationshipType {
    Suggestion = 6,
    Implicit = 5,
    Outgoing = 4,
    Incoming = 3,
    Blocked = 2,
    #[default]
    Friends = 1,
    None = 0,
}
