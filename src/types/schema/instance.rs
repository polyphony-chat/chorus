// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Contains schema for miscellaneous api routes, such as /version and /ping
//!
//! Implementations of those routes can be found in /api/instance.rs

use serde::{Deserialize, Serialize};

use crate::types::{GeneralConfiguration, Snowflake};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The return type of the spacebar-only /api/ping endpoint
pub struct PingReturn {
    /// Note: always "pong!"
    pub ping: String,
    pub instance: PingInstance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
/// [GeneralConfiguration] as returned from the /api/ping endpoint
pub struct PingInstance {
    pub id: Option<Snowflake>,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub correspondence_email: Option<String>,
    pub correspondence_user_id: Option<String>,
    pub front_page: Option<String>,
    pub tos_page: Option<String>,
}

impl PingInstance {
    /// Converts self into the [GeneralConfiguration] type
    pub fn into_general_configuration(self) -> GeneralConfiguration {
        GeneralConfiguration {
            instance_name: self.name,
            instance_description: self.description,
            front_page: self.front_page,
            tos_page: self.tos_page,
            correspondence_email: self.correspondence_email,
            correspondence_user_id: self.correspondence_user_id,
            image: self.image,
            instance_id: self.id,
        }
    }

    /// Converts the [GeneralConfiguration] type into self
    pub fn from_general_configuration(other: GeneralConfiguration) -> Self {
        Self {
            id: other.instance_id,
            name: other.instance_name,
            description: other.instance_description,
            image: other.image,
            correspondence_email: other.correspondence_email,
            correspondence_user_id: other.correspondence_user_id,
            front_page: other.front_page,
            tos_page: other.tos_page,
        }
    }
}

impl From<PingInstance> for GeneralConfiguration {
    fn from(value: PingInstance) -> Self {
        value.into_general_configuration()
    }
}

impl From<GeneralConfiguration> for PingInstance {
    fn from(value: GeneralConfiguration) -> Self {
        Self::from_general_configuration(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The return type of the symfonia-only /version endpoint
pub struct VersionReturn {
    /// The instance's software version, e. g. "0.1.0"
    pub version: String,
    /// The instance's software, e. g. "symfonia" or "spacebar"
    pub server: String,
}
