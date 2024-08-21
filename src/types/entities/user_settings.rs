// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{serde::ts_milliseconds_option, Utc};
use serde::{Deserialize, Serialize};

use crate::types::Shared;
use crate::{UInt16, UInt32, UInt8};
use serde_aux::field_attributes::deserialize_option_number_from_string;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Copy, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    #[default]
    Online,
    Offline,
    Dnd,
    Idle,
    Invisible,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Copy, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
pub enum UserTheme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct UserSettings {
    pub afk_timeout: Option<UInt16>,
    pub allow_accessibility_detection: bool,
    pub animate_emoji: bool,
    pub animate_stickers: UInt8,
    pub contact_sync_enabled: bool,
    pub convert_emoticons: bool,
    pub custom_status: Option<CustomStatus>,
    pub default_guilds_restricted: bool,
    pub detect_platform_accounts: bool,
    pub developer_mode: bool,
    pub disable_games_tab: bool,
    pub enable_tts_command: bool,
    pub explicit_content_filter: UInt8,
    pub friend_source_flags: FriendSourceFlags,
    pub gateway_connected: Option<bool>,
    pub gif_auto_play: bool,
    pub guild_folders: Vec<GuildFolder>,
    #[serde(default)]
    pub guild_positions: Vec<String>,
    pub inline_attachment_media: bool,
    pub inline_embed_media: bool,
    pub locale: String,
    pub message_display_compact: bool,
    pub native_phone_integration_enabled: bool,
    pub render_embeds: bool,
    pub render_reactions: bool,
    pub restricted_guilds: Vec<String>,
    pub show_current_game: bool,
    pub status: Shared<UserStatus>,
    pub stream_notifications_enabled: bool,
    pub theme: UserTheme,
    pub timezone_offset: i16,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            afk_timeout: Some(3600.into()),
            allow_accessibility_detection: true,
            animate_emoji: true,
            #[cfg(not(feature = "sqlx"))]
            animate_stickers: 0,
            #[cfg(feature = "sqlx")]
            animate_stickers: 0.into(),
            contact_sync_enabled: false,
            convert_emoticons: false,
            custom_status: None,
            default_guilds_restricted: false,
            detect_platform_accounts: false,
            developer_mode: true,
            disable_games_tab: true,
            enable_tts_command: false,
            #[cfg(not(feature = "sqlx"))]
            explicit_content_filter: 0,
            #[cfg(feature = "sqlx")]
            explicit_content_filter: 0.into(),
            friend_source_flags: Default::default(),
            gateway_connected: Some(false),
            gif_auto_play: false,
            guild_folders: Default::default(),
            guild_positions: Default::default(),
            inline_attachment_media: true,
            inline_embed_media: true,
            locale: "en-US".to_string(),
            message_display_compact: false,
            native_phone_integration_enabled: true,
            render_embeds: true,
            render_reactions: true,
            restricted_guilds: Default::default(),
            show_current_game: true,
            status: Default::default(),
            stream_notifications_enabled: false,
            theme: UserTheme::Dark,
            timezone_offset: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow, sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "interface_type"))]
pub struct CustomStatus {
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
    #[serde(with = "ts_milliseconds_option")]
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow, sqlx::Type))]
pub struct FriendSourceFlags {
    pub all: bool,
}

impl Default for FriendSourceFlags {
    fn default() -> Self {
        Self { all: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow, sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "interface_type"))]
pub struct GuildFolder {
    pub color: Option<UInt32>,
    pub guild_ids: Vec<String>,
    // FIXME: What is this thing?
    // It's not a snowflake, and it's sometimes a string and sometimes an integer.
    //
    // Ex: 1249181105
    //
    // It can also be negative somehow? Ex: -1176643795
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub settings: Shared<UserSettings>,
}
