// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::config::types::subconfigs::region::Region;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionConfiguration {
    pub default: String,
    pub use_default_as_optimal: bool,
    pub available: Vec<Region>,
}

impl Default for RegionConfiguration {
    fn default() -> Self {
        Self {
            default: String::from("spacebar"),
            use_default_as_optimal: true,
            available: vec![Region {
                id: String::from("spacebar"),
                name: String::from("spacebar"),
                endpoint: String::from("127.0.0.1:3004"),
                location: None,
                vip: false,
                custom: false,
                deprecated: false,
            }],
        }
    }
}
