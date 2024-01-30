// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create or modify a Role.
/// See: [https://docs.spacebar.chat/routes/#cmp--schemas-rolemodifyschema](https://docs.spacebar.chat/routes/#cmp--schemas-rolemodifyschema)
pub struct RoleCreateModifySchema {
    pub name: Option<String>,
    pub permissions: Option<String>,
    pub color: Option<u32>,
    pub hoist: Option<bool>,
    pub icon: Option<Vec<u8>>,
    pub unicode_emoji: Option<String>,
    pub mentionable: Option<bool>,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to update a roles' position.
/// See: [https://docs.spacebar.chat/routes/#cmp--schemas-rolepositionupdateschema](https://docs.spacebar.chat/routes/#cmp--schemas-rolepositionupdateschema)
pub struct RolePositionUpdateSchema {
    pub id: String,
    pub position: u16,
}
