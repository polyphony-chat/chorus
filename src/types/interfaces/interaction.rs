// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::entities::{AllowedMention, Embed};
use crate::types::utils::Snowflake;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Snowflake,
    pub r#type: InteractionType,
    pub data: Value,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub member_id: Snowflake,
    pub token: String,
    pub version: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    #[default]
    SelfCommand = 0,
    Ping = 1,
    ApplicationCommand = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionResponseType {
    SelfCommandResponse = 0,
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    AcknowledgeWithSource = 5,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionApplicationCommandCallbackData {
    pub tts: bool,
    pub content: String,
    pub embeds: Vec<Embed>,
    pub allowed_mentions: AllowedMention,
}
