use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::entities::Channel;
use crate::events::WebSocketEvent;

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
/// Not directly serialized, as the inner payload is a channel object
pub struct ChannelCreate {
    pub channel: Channel,
}

impl WebSocketEvent for ChannelCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-update
/// Not directly serialized, as the inner payload is a channel object
pub struct ChannelUpdate {
    pub channel: Channel,
}

impl WebSocketEvent for ChannelUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-delete
/// Not directly serialized, as the inner payload is a channel object
pub struct ChannelDelete {
    pub channel: Channel,
}

impl WebSocketEvent for ChannelDelete {}