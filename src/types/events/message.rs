use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::entities::{Emoji, GuildMember, Message, PublicUser};

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TypingStartEvent {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub user_id: String,
    pub timestamp: i64,
    pub member: Option<GuildMember>,
}

impl WebSocketEvent for TypingStartEvent {}

#[derive(Debug, Serialize, Deserialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#message-create
pub struct MessageCreate {
    #[serde(flatten)]
    message: Message,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Option<Vec<MessageCreateUser>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#message-create-message-create-extra-fields
pub struct MessageCreateUser {
    #[serde(flatten)]
    user: PublicUser,
    member: Option<GuildMember>,
}

impl WebSocketEvent for MessageCreate {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageUpdate {
    #[serde(flatten)]
    message: Message,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Option<Vec<MessageCreateUser>>,
}

impl WebSocketEvent for MessageUpdate {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageDelete {
    id: String,
    channel_id: String,
    guild_id: Option<String>,
}

impl WebSocketEvent for MessageDelete {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageDeleteBulk {
    ids: Vec<String>,
    channel_id: String,
    guild_id: Option<String>,
}

impl WebSocketEvent for MessageDeleteBulk {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionAdd {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    emoji: Emoji,
}

impl WebSocketEvent for MessageReactionAdd {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionRemove {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}

impl WebSocketEvent for MessageReactionRemove {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionRemoveAll {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
}

impl WebSocketEvent for MessageReactionRemoveAll {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionRemoveEmoji {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}

impl WebSocketEvent for MessageReactionRemoveEmoji {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
///
/// Not documented anywhere unofficially
///
/// Apparently "Message ACK refers to marking a message as read for Discord's API." (https://github.com/Rapptz/discord.py/issues/1851)
/// I suspect this is sent and recieved from the gateway to let clients on other devices know the user has read a message
///
/// {"t":"MESSAGE_ACK","s":3,"op":0,"d":{"version":52,"message_id":"1107236673638633472","last_viewed":null,"flags":null,"channel_id":"967363950217936897"}}
pub struct MessageACK {
    /// ?
    pub version: u16,
    pub message_id: String,
    /// This is an integer???
    /// Not even unix, see '3070'???
    pub last_viewed: Option<u64>,
    /// What flags?
    pub flags: Option<serde_json::Value>,
    pub channel_id: String,
}
impl WebSocketEvent for MessageACK {}
