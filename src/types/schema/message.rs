use serde::{Deserialize, Serialize};

use crate::types::entities::{
    AllowedMention, Component, Embed, MessageReference, PartialDiscordFileAttachment,
};
use crate::types::Snowflake;

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

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a Message Search Query JSON Body.
/// The `channel_id` field is not applicable when using the `GET /channels/{channel.id}/messages/search` endpoint.
///
/// # Reference:
/// See <https://discord-userdoccers.vercel.app/resources/message#search-messages>
pub struct MessageSearchQuery {
    attachment_extension: Option<Vec<String>>,
    attachment_filename: Option<Vec<String>>,
    author_id: Option<Vec<Snowflake>>,
    author_type: Option<Vec<String>>,
    channel_id: Option<Vec<Snowflake>>,
    command_id: Option<Vec<Snowflake>>,
    content: Option<String>,
    embed_provider: Option<Vec<String>>,
    embed_type: Option<Vec<String>>,
    has: Option<Vec<String>>,
    include_nsfw: Option<bool>,
    limit: Option<i32>,
    link_hostname: Option<Vec<String>>,
    max_id: Option<String>,
    mention_everyone: Option<bool>,
    mentions: Option<Vec<Snowflake>>,
    min_id: Option<String>,
    offset: Option<i32>,
    pinned: Option<bool>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}
