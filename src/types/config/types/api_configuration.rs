// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfiguration {
    pub default_version: String,
    pub active_versions: Vec<String>,
    pub endpoint_public: Option<String>,
}

impl Default for ApiConfiguration {
    fn default() -> Self {
        Self {
            default_version: String::from("9"),
            active_versions: vec![
                String::from("6"),
                String::from("7"),
                String::from("8"),
                String::from("9"),
            ],
            endpoint_public: None,
        }
    }
}
