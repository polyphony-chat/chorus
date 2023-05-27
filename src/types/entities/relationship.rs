use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::types::Snowflake;

use super::PublicUser;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// See https://docs.spacebar.chat/routes/#get-/users/@me/relationships/
pub struct Relationship {
    pub id: Snowflake,
    #[serde(rename = "type")] 
    pub relationship_type: RelationshipType,
    pub nickname: Option<String>,
    pub user: PublicUser
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default)]
#[repr(u8)]
/// See https://github.com/spacebarchat/server/blob/60394d8c43904ff17935d6edbbfb09ecd479570a/src/util/entities/Relationship.ts#L30
pub enum RelationshipType {
    Outgoing = 4,
    Incoming = 3,
    Blocked = 2,
    #[default]
    Friends = 1,
}