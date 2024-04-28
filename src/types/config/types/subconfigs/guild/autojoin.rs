// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoJoinConfiguration {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guilds: Option<Vec<Snowflake>>,
    pub can_leave: bool,
}

impl Default for AutoJoinConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            guilds: None,
            can_leave: true,
        }
    }
}
