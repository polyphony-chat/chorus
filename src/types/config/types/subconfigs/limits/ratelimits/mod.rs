// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod route;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitOptions {
    pub bot: Option<u64>,
    pub count: u64,
    pub window: u64,
    #[serde(default)]
    pub only_ip: bool,
}
