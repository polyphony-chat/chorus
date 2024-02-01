// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, Relationship, RelationshipType, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
/// See <https://github.com/spacebarchat/server/issues/204>
pub struct RelationshipAdd {
    #[serde(flatten)]
    pub relationship: Relationship,
    pub should_notify: bool,
}

impl WebSocketEvent for RelationshipAdd {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://github.com/spacebarchat/server/issues/203>
pub struct RelationshipRemove {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub relationship_type: RelationshipType,
}

impl WebSocketEvent for RelationshipRemove {}
