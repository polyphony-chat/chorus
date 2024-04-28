// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientReleaseConfiguration {
    pub use_local_release: bool,
    pub upstream_version: String,
}

impl Default for ClientReleaseConfiguration {
    fn default() -> Self {
        Self {
            use_local_release: true,
            upstream_version: String::from("0.0.264"),
        }
    }
}
