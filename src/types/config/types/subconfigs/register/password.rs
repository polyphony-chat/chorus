// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordConfiguration {
    pub required: bool,
    pub min_length: u8,
    pub min_numbers: u8,
    pub min_upper_case: u8,
    pub min_symbols: u8,
}

impl Default for PasswordConfiguration {
    fn default() -> Self {
        Self {
            required: false,
            min_length: 8,
            min_numbers: 2,
            min_upper_case: 2,
            min_symbols: 0,
        }
    }
}
