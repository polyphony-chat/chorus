use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{Emoji, GuildMember, Message, PublicUser},
    Snowflake,
};

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#typing-start>
pub struct TypingStartEvent {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub timestamp: i64,
    pub member: Option<GuildMember>,
}

impl WebSocketEvent for TypingStartEvent {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#message-create>
pub struct MessageCreate {
    #[serde(flatten)]
    pub message: Message,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub mentions: Option<Vec<MessageCreateUser>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#message-create-message-create-extra-fields>
pub struct MessageCreateUser {
    #[serde(flatten)]
    pub user: PublicUser,
    pub member: Option<GuildMember>,
}

impl WebSocketEvent for MessageCreate {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-update>
pub struct MessageUpdate {
    #[serde(flatten)]
    pub message: Message,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub mentions: Option<Vec<MessageCreateUser>>,
}

impl WebSocketEvent for MessageUpdate {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-delete>
pub struct MessageDelete {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

impl WebSocketEvent for MessageDelete {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-delete-bulk>
pub struct MessageDeleteBulk {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

impl WebSocketEvent for MessageDeleteBulk {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-add>
pub struct MessageReactionAdd {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub emoji: Emoji,
}

impl WebSocketEvent for MessageReactionAdd {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove>
pub struct MessageReactionRemove {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: Emoji,
}

impl WebSocketEvent for MessageReactionRemove {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-all>
pub struct MessageReactionRemoveAll {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

impl WebSocketEvent for MessageReactionRemoveAll {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-emoji>
pub struct MessageReactionRemoveEmoji {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: Emoji,
}

impl WebSocketEvent for MessageReactionRemoveEmoji {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Officially Undocumented
///
/// Not documented anywhere unofficially
///
/// Apparently "Message ACK refers to marking a message as read for Discord's API." (<https://github.com/Rapptz/discord.py/issues/1851>)
/// I suspect this is sent and recieved from the gateway to let clients on other devices know the user has read a message
///
/// {"t":"MESSAGE_ACK","s":3,"op":0,"d":{"version":52,"message_id":"1107236673638633472","last_viewed":null,"flags":null,"channel_id":"967363950217936897"}}
pub struct MessageACK {
    /// ?
    pub version: u16,
    pub message_id: Snowflake,
    /// This is an integer???
    /// Not even unix, see '3070'???
    pub last_viewed: Option<u64>,
    /// What flags?
    pub flags: Option<serde_json::Value>,
    pub channel_id: Snowflake,
}

impl WebSocketEvent for MessageACK {}
