// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::config::types::subconfigs::limits::ratelimits::RateLimitOptions;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct AuthRateLimit {
    pub login: RateLimitOptions,
    pub register: RateLimitOptions,
}

impl Default for AuthRateLimit {
    fn default() -> Self {
        Self {
            login: RateLimitOptions {
                bot: None,
                count: 5,
                window: 60,
                only_ip: false,
            },
            register: RateLimitOptions {
                bot: None,
                count: 2,
                window: 60 * 60 * 12,
                only_ip: false,
            },
        }
    }
}
