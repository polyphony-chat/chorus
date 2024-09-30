// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::PremiumType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct UserDefaults {
    pub premium: bool,
    pub premium_type: PremiumType,
    pub verified: bool,
}

impl Default for UserDefaults {
    fn default() -> Self {
        Self {
            premium: true,
            premium_type: PremiumType::Tier2,
            verified: true,
        }
    }
}
