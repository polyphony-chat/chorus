use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::Snowflake;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
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