// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GifProvider {
    #[default]
    Tenor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifConfiguration {
    pub enabled: bool,
    pub provider: GifProvider,
    pub api_key: Option<String>,
}

impl Default for GifConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: GifProvider::Tenor,
            api_key: Some(String::from("LIVDSRZULELA")),
        }
    }
}
