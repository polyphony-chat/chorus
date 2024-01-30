// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GeneralConfiguration {
    pub instance_name: String,
    pub instance_description: Option<String>,
    pub front_page: Option<String>,
    pub tos_page: Option<String>,
    pub correspondence_email: Option<String>,
    pub correspondence_user_id: Option<String>,
    pub image: Option<String>,
    pub instance_id: Option<Snowflake>,
}

impl Default for GeneralConfiguration {
    fn default() -> Self {
        Self {
            instance_name: String::from("Spacebar-compatible Instance"),
            instance_description: Some(String::from("This is a spacebar-compatible instance.")),
            front_page: None,
            tos_page: None,
            correspondence_email: None,
            correspondence_user_id: None,
            image: None,
            instance_id: Some(Snowflake::generate()),
        }
    }
}
