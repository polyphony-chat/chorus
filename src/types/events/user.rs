use crate::types::entities::PublicUser;
use crate::types::events::WebSocketEvent;
use crate::types::utils::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#user-update
pub struct UserUpdate {
    #[serde(flatten)]
    pub user: PublicUser,
}

impl WebSocketEvent for UserUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Undocumented
///
/// Possibly an update for muted guild / channel settings for the current user
///
/// {"version":2,"suppress_roles":false,"suppress_everyone":false,"notify_highlights":0,"muted":false,"mute_scheduled_events":false,"mute_config":null,"mobile_push":true,"message_notifications":1,"hide_muted_channels":false,"guild_id":"848582562217590824","flags":0,"channel_overrides":[{"muted":false,"mute_config":null,"message_notifications":3,"flags":4096,"collapsed":false,"channel_id":"1042689182893604885"}]}
pub struct UserGuildSettingsUpdate {
    pub version: u8,
    pub suppress_roles: bool,
    pub suppress_everyone: bool,
    pub notify_highlights: u8,
    pub muted: bool,
    pub mute_scheduled_events: bool,
    /// ??
    pub mute_config: Option<serde_json::Value>,
    pub mobile_push: bool,
    pub message_notifications: u8,
    pub hide_muted_channels: bool,
    pub guild_id: Snowflake,
    pub flags: i32,
    pub channel_overrides: Vec<UserGuildSettingsChannelOverride>,
}

impl WebSocketEvent for UserGuildSettingsUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// Undocumented
///
/// Received in [UserGuildSettingsUpdate]
///
/// {"muted":false,"mute_config":null,"message_notifications":3,"flags":4096,"collapsed":false,"channel_id":"1042689182893604885"}
pub struct UserGuildSettingsChannelOverride {
    pub muted: bool,
    /// ??
    pub mute_config: Option<serde_json::Value>,
    pub message_notifications: u8,
    pub flags: i32,
    pub collapsed: bool,
    pub channel_id: Snowflake,
}
