// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    Shared,
    entities::{
        Application, Attachment, Channel, Emoji, GuildMember, PublicUser, RoleSubscriptionData,
        Sticker, StickerItem, User,
    },
    utils::Snowflake,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Represents a message sent in a channel.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/message#message-object>
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub author: Option<PublicUser>,
    pub content: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub tts: Option<bool>,
    pub mention_everyone: bool,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mentions: Option<Vec<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mention_roles: Option<Vec<Snowflake>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mention_channels: Option<Vec<ChannelMention>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub attachments: Option<Vec<Attachment>>,
    #[cfg(feature = "sqlx")]
    pub embeds: sqlx::types::Json<Vec<Embed>>,
    #[cfg(not(feature = "sqlx"))]
    pub embeds: Option<Vec<Embed>>,
    #[cfg(feature = "sqlx")]
    pub reactions: Option<sqlx::types::Json<Vec<Reaction>>>,
    #[cfg(not(feature = "sqlx"))]
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<serde_json::Value>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub message_type: i32,
    #[cfg(feature = "sqlx")]
    pub activity: Option<sqlx::types::Json<MessageActivity>>,
    #[cfg(not(feature = "sqlx"))]
    pub activity: Option<MessageActivity>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub application: Option<Application>,
    pub application_id: Option<Snowflake>,
    #[cfg(feature = "sqlx")]
    pub message_reference: Option<sqlx::types::Json<MessageReference>>,
    #[cfg(not(feature = "sqlx"))]
    pub message_reference: Option<MessageReference>,
    pub flags: Option<MessageFlags>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub referenced_message: Option<Box<Message>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub interaction: Option<MessageInteraction>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub thread: Option<Channel>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub components: Option<Vec<Component>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub sticker_items: Option<Vec<StickerItem>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub stickers: Option<Vec<Sticker>>,
    pub position: Option<i32>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub role_subscription_data: Option<RoleSubscriptionData>,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.channel_id == other.channel_id
            && self.author == other.author
            && self.content == other.content
            && self.timestamp == other.timestamp
            && self.edited_timestamp == other.edited_timestamp
            && self.tts == other.tts
            && self.mention_everyone == other.mention_everyone
            && self.mentions == other.mentions
            && self.mention_roles == other.mention_roles
            && self.mention_channels == other.mention_channels
            && self.attachments == other.attachments
            && self.embeds == other.embeds
            && self.embeds == other.embeds
            && self.nonce == other.nonce
            && self.pinned == other.pinned
            && self.webhook_id == other.webhook_id
            && self.message_type == other.message_type
            && self.activity == other.activity
            && self.activity == other.activity
            && self.application_id == other.application_id
            && self.message_reference == other.message_reference
            && self.message_reference == other.message_reference
            && self.flags == other.flags
            && self.referenced_message == other.referenced_message
            && self.thread == other.thread
            && self.components == other.components
            && self.sticker_items == other.sticker_items
            && self.position == other.position
            && self.role_subscription_data == other.role_subscription_data
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Ord, PartialOrd)]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/message#message-reference-object>
pub struct MessageReference {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageInteraction {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub name: String,
    pub user: User,
    pub member: Option<Shared<GuildMember>>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct AllowedMention {
    parse: Vec<AllowedMentionType>,
    roles: Vec<Snowflake>,
    users: Vec<Snowflake>,
    replied_user: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AllowedMentionType {
    Roles,
    Users,
    Everyone,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    #[serde(rename = "type")]
    channel_type: i32,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<i32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct EmbedImage {
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct EmbedThumbnail {
    url: String,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reaction {
    pub count: u32,
    pub burst_count: u32,
    pub me: bool,
    pub burst_me: bool,
    pub burst_colors: Vec<String>,
    pub emoji: Emoji,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum Component {
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    TextInput = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/message#message-activity-object>
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: i64,
    pub party_id: Option<String>,
}

bitflags! {
    #[derive(Debug, Clone, Copy,  Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.sex/resources/message#message-type>
    pub struct MessageFlags: u64 {
        /// This message has been published to subscribed channels (via Channel Following)
        const CROSSPOSTED = 1 << 0;
        ///	This message originated from a message in another channel (via Channel Following)
        const IS_CROSSPOST = 1 << 1;
        /// Embeds will not be included when serializing this message
        const SUPPRESS_EMBEDS = 1 << 2;
        /// The source message for this crosspost has been deleted (via Channel Following)
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// This message came from the urgent message system
        const URGENT = 1 << 4;
        /// This message has an associated thread, with the same ID as the message
        const HAS_THREAD = 1 << 5;
        /// This message is only visible to the user who invoked the interaction
        const EPHEMERAL = 1 << 6;
        /// This message is an interaction response and the bot is "thinking"
        const LOADING = 1 << 7;
        /// Some roles were not mentioned and added to the thread
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        /// This message contains a link that impersonates Discord
        const SHOULD_SHOW_LINK_NOT_DISCORD_WARNING = 1 << 10;
        /// This message will not trigger push and desktop notifications
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        /// This message's audio attachments are rendered as voice messages
        const VOICE_MESSAGE = 1 << 13;
        /// This message has a forwarded message snapshot attached
        const HAS_SNAPSHOT = 1 << 14;
    }
}