// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLimits {
    pub max_guilds: u64,
    pub max_username: u16,
    pub max_friends: u64,
}

impl Default for UserLimits {
    fn default() -> Self {
        Self {
            max_guilds: 1048576,
            max_username: 32,
            max_friends: 5000,
        }
    }
}
