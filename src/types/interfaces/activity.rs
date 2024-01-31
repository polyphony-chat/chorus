// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{entities::Emoji, Snowflake};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: i32,
    url: Option<String>,
    created_at: i64,
    timestamps: Option<ActivityTimestamps>,
    application_id: Option<Snowflake>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<ActivityParty>,
    assets: Option<ActivityAssets>,
    secrets: Option<ActivitySecrets>,
    instance: Option<bool>,
    flags: Option<i32>,
    buttons: Option<Vec<ActivityButton>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
struct ActivityTimestamps {
    start: Option<i64>,
    end: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ActivityParty {
    id: Option<String>,
    size: Option<Vec<(i32, i32)>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    #[serde(rename = "match")]
    match_string: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ActivityButton {
    label: String,
    url: String,
}
