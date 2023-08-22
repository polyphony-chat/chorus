use serde::{Deserialize, Serialize};

use crate::types::entities::{
    AllowedMention, Component, Embed, MessageReference, PartialDiscordFileAttachment,
};
use crate::types::{Attachment, Snowflake};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
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

pub enum MessageSearchEndpoint {
    GuildChannel(Snowflake),
    Channel(Snowflake),
}

impl std::fmt::Display for MessageSearchEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageSearchEndpoint::Channel(id) => {
                write!(f, "channels/{}", &id.to_string())
            }
            MessageSearchEndpoint::GuildChannel(id) => {
                write!(f, "guilds/{}", &id.to_string())
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a Message Search Query JSON Body.
/// The `channel_id` field is not applicable when using the `GET /channels/{channel.id}/messages/search` endpoint.
///
/// # Reference:
/// See <https://discord-userdoccers.vercel.app/resources/message#search-messages>
pub struct MessageSearchQuery {
    pub attachment_extension: Option<Vec<String>>,
    pub attachment_filename: Option<Vec<String>>,
    pub author_id: Option<Vec<Snowflake>>,
    pub author_type: Option<Vec<String>>,
    pub channel_id: Option<Vec<Snowflake>>,
    pub command_id: Option<Vec<Snowflake>>,
    pub content: Option<String>,
    pub embed_provider: Option<Vec<String>>,
    pub embed_type: Option<Vec<String>>,
    pub has: Option<Vec<String>>,
    pub include_nsfw: Option<bool>,
    pub limit: Option<i32>,
    pub link_hostname: Option<Vec<String>>,
    pub max_id: Option<String>,
    pub mention_everyone: Option<bool>,
    pub mentions: Option<Vec<Snowflake>>,
    pub min_id: Option<String>,
    pub offset: Option<i32>,
    pub pinned: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl std::default::Default for MessageSearchQuery {
    fn default() -> Self {
        Self {
            attachment_extension: Default::default(),
            attachment_filename: Default::default(),
            author_id: Default::default(),
            author_type: Default::default(),
            channel_id: Default::default(),
            command_id: Default::default(),
            content: Default::default(),
            embed_provider: Default::default(),
            embed_type: Default::default(),
            has: Default::default(),
            include_nsfw: Some(false),
            limit: Some(25),
            link_hostname: Default::default(),
            max_id: Default::default(),
            mention_everyone: Default::default(),
            mentions: Default::default(),
            min_id: Default::default(),
            offset: Some(0),
            pinned: Default::default(),
            sort_by: Default::default(),
            sort_order: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateGreetMessage {
    pub sticker_ids: Vec<Snowflake>,
    pub allowed_mentions: Option<AllowedMention>,
    pub message_reference: Option<MessageReference>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageAck {
    pub token: Option<String>,
    pub manual: Option<bool>,
    pub mention_count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct MessageModifySchema {
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    embed: Option<Embed>,
    allowed_mentions: Option<AllowedMention>,
    components: Option<Vec<Component>>,
    flags: Option<i32>,
    files: Option<Vec<u8>>,
    payload_json: Option<String>,
    attachments: Option<Vec<Attachment>>,
}
