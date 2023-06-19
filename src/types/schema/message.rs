use serde::{Deserialize, Serialize};

use crate::types::entities::{
    AllowedMention, Component, Embed, MessageReference, PartialDiscordFileAttachment,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageSendSchema {
    #[serde(rename = "type")]
    message_type: Option<i32>,
    content: Option<String>,
    nonce: Option<String>,
    tts: Option<bool>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMention>,
    message_reference: Option<MessageReference>,
    components: Option<Vec<Component>>,
    sticker_ids: Option<Vec<String>>,
    pub attachments: Option<Vec<PartialDiscordFileAttachment>>,
}

impl MessageSendSchema {
    pub fn new(
        message_type: Option<i32>,
        content: Option<String>,
        nonce: Option<String>,
        tts: Option<bool>,
        embeds: Option<Vec<Embed>>,
        allowed_mentions: Option<AllowedMention>,
        message_reference: Option<MessageReference>,
        components: Option<Vec<Component>>,
        sticker_ids: Option<Vec<String>>,
        attachments: Option<Vec<PartialDiscordFileAttachment>>,
    ) -> MessageSendSchema {
        MessageSendSchema {
            message_type,
            content,
            nonce,
            tts,
            embeds,
            allowed_mentions,
            message_reference,
            components,
            sticker_ids,
            attachments,
        }
    }
}
