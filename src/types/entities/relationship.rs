use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::Snowflake;

use super::PublicUser;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// See <https://discord-userdoccers.vercel.app/resources/user#relationship-structure>
pub struct Relationship {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
    pub nickname: Option<String>,
    pub user: PublicUser,
    pub since: Option<DateTime<Utc>>,
}

impl PartialEq for Relationship {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.relationship_type == other.relationship_type
            && self.since == other.since
            && self.nickname == other.nickname
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default, Eq, PartialEq)]
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
