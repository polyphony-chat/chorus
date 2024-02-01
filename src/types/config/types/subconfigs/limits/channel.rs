// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelLimits {
    pub max_pins: u16,
    pub max_topic: u16,
    pub max_webhooks: u16,
}

impl Default for ChannelLimits {
    fn default() -> Self {
        Self {
            max_pins: 500,
            max_topic: 1024,
            max_webhooks: 100,
        }
    }
}
