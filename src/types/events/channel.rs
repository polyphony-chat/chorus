use crate::types::entities::Channel;
use crate::types::events::WebSocketEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-pins-update
pub struct ChannelPinsUpdate {
    pub guild_id: Option<String>,
    pub channel_id: String,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}

impl WebSocketEvent for ChannelPinsUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-create
pub struct ChannelCreate {
    #[serde(flatten)]
    pub channel: Channel,
}

impl WebSocketEvent for ChannelCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-update
pub struct ChannelUpdate {
    #[serde(flatten)]
    pub channel: Channel,
}

impl WebSocketEvent for ChannelUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// Officially undocumented.
/// Sends updates to client about a new message with its id
/// {"channel_unread_updates": [{"id": "816412869766938648", "last_message_id": "1085892012085104680"}}
pub struct ChannelUnreadUpdate {
    pub channel_unread_updates: Vec<ChannelUnreadUpdateObject>,
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
/// Contains very few fields from [Channel]
/// See also [ChannelUnreadUpdates]
pub struct ChannelUnreadUpdateObject {
    pub id: String,
    pub last_message_id: String,
    pub last_pin_timestamp: Option<String>,
}

impl WebSocketEvent for ChannelUnreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-delete
pub struct ChannelDelete {
    #[serde(flatten)]
    pub channel: Channel,
}

impl WebSocketEvent for ChannelDelete {}
