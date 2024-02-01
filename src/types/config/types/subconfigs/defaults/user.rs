// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDefaults {
    pub premium: bool,
    pub premium_type: u8,
    pub verified: bool,
}

impl Default for UserDefaults {
    fn default() -> Self {
        Self {
            premium: true,
            premium_type: 2,
            verified: true,
        }
    }
}
