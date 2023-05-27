use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        Application, Attachment, Channel, Emoji, GuildMember, RoleSubscriptionData, Sticker,
        StickerItem, User,
    },
    utils::Snowflake,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    id: Snowflake,
    pub channel_id: Snowflake,
    author: User,
    content: String,
    timestamp: String,
    edited_timestamp: Option<String>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<String>,
    mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<serde_json::Value>,
    pinned: bool,
    webhook_id: Option<String>,
    #[serde(rename = "type")]
    message_type: i32,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<String>,
    message_reference: Option<MessageReference>,
    flags: Option<i32>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
    components: Option<Vec<Component>>,
    sticker_items: Option<Vec<StickerItem>>,
    stickers: Option<Vec<Sticker>>,
    position: Option<i32>,
    role_subscription_data: Option<RoleSubscriptionData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageReference {
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageInteraction {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub name: String,
    pub user: User,
    pub member: Option<GuildMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedMention {
    parse: Vec<AllowedMentionType>,
    roles: Vec<Snowflake>,
    users: Vec<Snowflake>,
    replied_user: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedMentionType {
    Roles,
    Users,
    Everyone,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    #[serde(rename = "type")]
    channel_type: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedImage {
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedThumbnail {
    url: String,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]

struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub count: i32,
    pub me: bool,
    pub emoji: Emoji,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: i64,
    pub party_id: Option<String>,
}
