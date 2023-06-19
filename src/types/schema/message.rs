use serde::{Deserialize, Serialize};

use crate::types::entities::{
    AllowedMention, Component, Embed, MessageReference, PartialDiscordFileAttachment,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageSendSchema {
    #[serde(rename = "type")]
    pub message_type: Option<i32>,
    pub content: Option<String>,
    pub nonce: Option<String>,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Embed>>,
    pub allowed_mentions: Option<AllowedMention>,
    pub message_reference: Option<MessageReference>,
    pub components: Option<Vec<Component>>,
    pub sticker_ids: Option<Vec<String>>,
    pub attachments: Option<Vec<PartialDiscordFileAttachment>>,
}
