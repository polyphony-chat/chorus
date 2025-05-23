// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct GuildLimits {
    pub max_roles: u16,
    pub max_emojis: u16,
    pub max_members: u64,
    pub max_channels: u32,
    pub max_channels_in_category: u32,
}

impl Default for GuildLimits {
    fn default() -> Self {
        Self {
            max_roles: 1000,
            max_emojis: 20_000,
            max_members: 25_000_000,
            max_channels: 65_535,
            max_channels_in_category: 65_535,
        }
    }
}
