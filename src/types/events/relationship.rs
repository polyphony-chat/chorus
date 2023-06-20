use crate::types::{Relationship, RelationshipType, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://github.com/spacebarchat/server/issues/204
pub struct RelationshipAdd {
    #[serde(flatten)]
    pub relationship: Relationship,
    pub should_notify: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://github.com/spacebarchat/server/issues/203
pub struct RelationshipRemove {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
}
