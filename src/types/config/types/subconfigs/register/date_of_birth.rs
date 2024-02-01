// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateOfBirthConfiguration {
    pub required: bool,
    pub minimum: u8,
}

impl Default for DateOfBirthConfiguration {
    fn default() -> Self {
        Self {
            required: true,
            minimum: 13,
        }
    }
}
