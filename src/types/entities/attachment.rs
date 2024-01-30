// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://discord.com/developers/docs/resources/channel#attachment-object>
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    /// Max 1024 characters
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub ephemeral: Option<bool>,
    /// The duration of the audio file (only for voice messages)
    pub duration_secs: Option<f32>,
    /// A Base64 encoded bytearray representing a sampled waveform (only for voice messages)
    ///
    /// # Notes
    /// Note that this is computed on the client side.
    /// This means it can be spoofed and isn't necessarily accurate.
    pub waveform: Option<String>,
    #[serde(skip_serializing)]
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub content: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PartialDiscordFileAttachment {
    pub id: Option<i16>,
    pub filename: String,
    /// Max 1024 characters
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<i64>,
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub ephemeral: Option<bool>,
    /// The duration of the audio file (only for voice messages)
    pub duration_secs: Option<f32>,
    /// A Base64 encoded bytearray representing a sampled waveform (only for voice messages)
    ///
    /// # Notes
    /// Note that this is computed on the client side.
    /// This means it can be spoofed and isn't necessarily accurate.
    pub waveform: Option<String>,
    #[serde(skip_serializing)]
    pub content: Vec<u8>,
}
