// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrationEmailConfiguration {
    pub required: bool,
    pub allowlist: bool,
    #[serde(default)]
    pub blacklist: bool,
    #[serde(default)]
    pub domains: Vec<String>,
}

impl Default for RegistrationEmailConfiguration {
    fn default() -> Self {
        Self {
            required: false,
            allowlist: false,
            blacklist: true,
            domains: Vec::new(),
        }
    }
}
