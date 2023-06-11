use serde::{Deserialize, Serialize};

use crate::types::entities::Channel;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
/// Represents the schema which needs to be sent to create a Guild.
/// See: [https://docs.spacebar.chat/routes/#cmp--schemas-guildcreateschema](https://docs.spacebar.chat/routes/#cmp--schemas-guildcreateschema)
pub struct GuildCreateSchema {
    pub name: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub guild_template_code: Option<String>,
    pub system_channel_id: Option<String>,
    pub rules_channel_id: Option<String>,
}
