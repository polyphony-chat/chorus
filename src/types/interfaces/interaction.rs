use crate::types::entities::{AllowedMention, Embed};
use crate::types::utils::Snowflake;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    SelfCommand = 0,
    Ping = 1,
    ApplicationCommand = 2,
}

pub enum InteractionResponseType {
    SelfCommandResponse = 0,
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    AcknowledgeWithSource = 5,
}

pub struct InteractionApplicationCommandCallbackData {
    pub tts: bool,
    pub content: String,
    pub embeds: Vec<Embed>,
    pub allowed_mentions: AllowedMention,
}
