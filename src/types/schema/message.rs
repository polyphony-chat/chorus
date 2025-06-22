// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::entities::{
    AllowedMention, Component, Embed, MessageReference, PartialDiscordFileAttachment,
};
use crate::types::{
    Attachment, CloudAttachment, EmbedType, Message, MessageFlags, MessageType, ReactionType,
    Snowflake,
};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct MessageSendSchema {
    #[serde(rename = "type")]
    pub message_type: Option<MessageType>,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
/// See <https://docs.discord.food/resources/message#search-messages>
pub struct MessageSearchQuery {
    pub attachment_extension: Option<Vec<String>>,
    pub attachment_filename: Option<Vec<String>>,
    pub author_id: Option<Vec<Snowflake>>,
    pub author_type: Option<Vec<AuthorType>>,
    pub channel_id: Option<Vec<Snowflake>>,
    pub command_id: Option<Vec<Snowflake>>,
    pub content: Option<String>,
    pub embed_provider: Option<Vec<String>>,
    pub embed_type: Option<Vec<EmbedType>>,
    pub has: Option<Vec<HasType>>,
    pub include_nsfw: Option<bool>,
    pub limit: Option<i32>,
    pub link_hostname: Option<Vec<String>>,
    pub max_id: Option<String>,
    pub mention_everyone: Option<bool>,
    pub mentions: Option<Vec<Snowflake>>,
    pub min_id: Option<String>,
    pub offset: Option<i32>,
    pub pinned: Option<bool>,
    pub sort_by: Option<SortType>,
    pub sort_order: Option<SortOrder>,
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AuthorType {
    User,
    #[serde(rename = "-user")]
    NotUser,
    Bot,
    #[serde(rename = "-bot")]
    NotBot,
    Webhook,
    #[serde(rename = "-webhook")]
    NotWebhook,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
#[serde(rename_all = "snake_case")]
pub enum HasType {
    Image,
    #[serde(rename = "-image")]
    NotImage,
    Sound,
    #[serde(rename = "-sound")]
    NotSound,
    Video,
    #[serde(rename = "-video")]
    NotVideo,
    File,
    #[serde(rename = "-file")]
    NotFile,
    Sticker,
    #[serde(rename = "-sticker")]
    NotSticker,
    Embed,
    #[serde(rename = "-embed")]
    NotEmbed,
    Link,
    #[serde(rename = "-link")]
    NotLink,
    Poll,
    #[serde(rename = "-poll")]
    NotPoll,
    Snapshot,
    #[serde(rename = "-snapshot")]
    NotSnapshot,
}

#[derive(
    Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash,
)]
#[serde(rename_all = "snake_case")]
pub enum SortType {
    #[default]
    Timestamp,
    Relevance,
}

#[derive(
    Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash,
)]
pub enum SortOrder {
    #[default]
    #[serde(rename = "desc")]
    Descending,
    #[serde(rename = "asc")]
    Ascending,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct MessageSearchResponse {
    pub messages: Vec<Message>,
    pub total_results: u64,
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
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    pub embed: Option<Embed>,
    pub allowed_mentions: Option<AllowedMention>,
    pub components: Option<Vec<Component>>,
    pub flags: Option<MessageFlags>,
    pub files: Option<Vec<u8>>,
    pub payload_json: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Copy, Eq, Hash, Ord)]
pub struct ReactionQuerySchema {
    pub after: Option<Snowflake>,
    pub limit: Option<u32>,
    #[serde(rename = "type")]
    pub reaction_type: Option<ReactionType>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Internal return schema for create_cloud_attachment_urls
///
/// See src/api/channels/attachments.rs
pub(crate) struct CreateCloudAttachmentURLsReturn {
    pub attachments: Vec<CloudAttachment>,
}
