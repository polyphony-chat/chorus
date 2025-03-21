// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverConfiguration {
    pub show_all_guilds: bool,
    pub use_recommendation: bool,
    pub offset: u16,
    pub limit: u16,
}

impl Default for DiscoverConfiguration {
    fn default() -> Self {
        Self {
            show_all_guilds: false,
            use_recommendation: false,
            offset: 0,
            limit: 24,
        }
    }
}
