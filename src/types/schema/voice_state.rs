// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::Snowflake;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Copy)]
/// # Reference:
/// See <https://docs.discord.sex/resources/voice#json-params>
pub struct VoiceStateUpdateSchema {
    /// The ID of the channel the user is currently in
    pub channel_id: Option<Snowflake>,
    /// Whether to suppress the user
    pub suppress: Option<bool>,
    /// The time at which the user requested to speak
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
}
