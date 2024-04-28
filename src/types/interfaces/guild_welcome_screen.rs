// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Hash)]
pub struct WelcomeScreenObject {
    pub enabled: bool,
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Hash)]
pub struct WelcomeScreenChannel {
    pub channel_id: Snowflake,
    pub description: String,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}
