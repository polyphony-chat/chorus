// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlobalRateLimit {
    pub limit: u16,
    pub window: u64,
    pub enabled: bool,
}

impl Default for GlobalRateLimit {
    fn default() -> Self {
        Self {
            limit: 100,
            window: 60 * 60 * 1000,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalRateLimits {
    pub register: GlobalRateLimit,
    pub send_message: GlobalRateLimit,
}

impl Default for GlobalRateLimits {
    fn default() -> Self {
        Self {
            register: GlobalRateLimit {
                limit: 25,
                ..Default::default()
            },
            send_message: GlobalRateLimit {
                limit: 200,
                window: 60 * 1000,
                ..Default::default()
            },
        }
    }
}
