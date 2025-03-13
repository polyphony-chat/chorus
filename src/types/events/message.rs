// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{Emoji, GuildMember, Message, PublicUser},
    Snowflake, WebSocketEvent,
};

use chorus_macros::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#typing-start>
pub struct TypingStartEvent {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub timestamp: i64,
    pub member: Option<GuildMember>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#message-create>
pub struct MessageCreate {
    #[serde(flatten)]
    pub message: Message,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub mentions: Option<Vec<MessageCreateUser>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#message-create-message-create-extra-fields>
pub struct MessageCreateUser {
    #[serde(flatten)]
    pub user: PublicUser,
    pub member: Option<GuildMember>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, WebSocketEvent)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-update>
pub struct MessageUpdate {
    #[serde(flatten)]
    pub message: Message,
    pub guild_id: Option<Snowflake>,
    pub member: Option<GuildMember>,
    pub mentions: Option<Vec<MessageCreateUser>>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    WebSocketEvent,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-delete>
pub struct MessageDelete {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    WebSocketEvent,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-delete-bulk>
pub struct MessageDeleteBulk {
    pub ids: Vec<Snowflake>,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, WebSocketEvent)]
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

#[derive(Debug, Serialize, Deserialize, Default, Clone, WebSocketEvent)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove>
pub struct MessageReactionRemove {
    pub user_id: Snowflake,
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: Emoji,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    WebSocketEvent,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-all>
pub struct MessageReactionRemoveAll {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, WebSocketEvent)]
/// # Reference
/// See <https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove-emoji>
pub struct MessageReactionRemoveEmoji {
    pub channel_id: Snowflake,
    pub message_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub emoji: Emoji,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, WebSocketEvent)]
/// Sent when a message that mentioned the current user in the last week is acknowledged and deleted.
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#recent-mention-delete>
pub struct RecentMentionDelete {
    pub message_id: Snowflake,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Officially Undocumented
///
/// Not documented anywhere unofficially
///
/// Apparently "Message ACK refers to marking a message as read for Discord's API." (<https://github.com/Rapptz/discord.py/issues/1851>)
/// I suspect this is sent and received from the gateway to let clients on other devices know the user has read a message
///
/// {"t":"MESSAGE_ACK","s":3,"op":0,"d":{"version":52,"message_id":"1107236673638633472","last_viewed":null,"flags":null,"channel_id":"967363950217936897"}}
pub struct MessageACK {
    // No ideas. See 206933
    pub version: u32,
    pub message_id: Snowflake,
    /// This is an integer???
    /// Not even unix, see '3070'???
    pub last_viewed: Option<u64>,
    /// What flags?
    pub flags: Option<serde_json::Value>,
    pub channel_id: Snowflake,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Used to request the last messages from channels.
///
/// Fires a [LastMessages] events with up to 100 messages that match the request.
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#request-last-messages>
pub struct RequestLastMessages {
	/// The ID of the guild the channels are in
	pub guild_id: Snowflake,
	/// The IDs of the channels to request last messages for (max 100)
	pub channel_ids: Vec<Snowflake>
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Sent as a response to [RequestLastMessages].
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#last-messages>
pub struct LastMessages {
	/// The ID of the guild the channels are in
	pub guild_id: Snowflake,
	pub messages: Vec<Message>
}
