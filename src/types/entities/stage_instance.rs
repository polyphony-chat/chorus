use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::types::Snowflake;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/resources/stage-instance
pub struct StageInstance {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    /// 1 - 120 chars
    pub topic: String,
    pub privacy_level: StageInstancePrivacyLevel,
    /// deprecated, apparently
    pub discoverable_disabled: Option<bool>,
    pub guild_scheduled_event_id: Option<Snowflake>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Default)]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] 
/// See https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level
pub enum StageInstancePrivacyLevel {
    /// deprecated, apparently
    Public = 1,
    #[default]
    GuildOnly = 2
}