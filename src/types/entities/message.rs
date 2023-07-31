

use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{
        Application, Attachment, Channel, Emoji, GuildMember, PublicUser, RoleSubscriptionData,
        Sticker, StickerItem, User,
    },
    utils::Snowflake,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub author: PublicUser,
    pub content: Option<String>,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: Option<bool>,
    pub mention_everyone: bool,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mentions: Option<Vec<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mention_roles: Vec<Snowflake>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mention_channels: Option<Vec<ChannelMention>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub attachments: Vec<Attachment>,
    #[cfg(feature = "sqlx")]
    pub embeds: Vec<sqlx::types::Json<Embed>>,
    #[cfg(not(feature = "sqlx"))]
    pub embeds: Vec<Embed>,
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
    pub flags: Option<u64>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<Channel>,
    pub components: Option<Vec<Component>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub stickers: Option<Vec<Sticker>>,
    pub position: Option<i32>,
    pub role_subscription_data: Option<RoleSubscriptionData>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct MessageInteraction {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub name: String,
    pub user: User,
    pub member: Option<GuildMember>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AllowedMention {
    parse: Vec<AllowedMentionType>,
    roles: Vec<Snowflake>,
    users: Vec<Snowflake>,
    replied_user: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbedImage {
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    url: String,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub count: u32,
    pub burst_count: u32,
    pub me: bool,
    pub burst_me: bool,
    pub burst_colors: Vec<String>,
    pub emoji: Emoji,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: i64,
    pub party_id: Option<String>,
}
