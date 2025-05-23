// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, Relationship, RelationshipType, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://github.com/spacebarchat/server/issues/204>
pub struct RelationshipAdd {
    #[serde(flatten)]
    pub relationship: Relationship,
    pub should_notify: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
/// See <https://github.com/spacebarchat/server/issues/203>
pub struct RelationshipRemove {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
}

