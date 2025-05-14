// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus_macros::WebSocketEvent;
use serde::{Deserialize, Serialize};

use crate::types::{Activity, WebSocketEvent};

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Officially Undocumented
/// Seems like it sends active session info to users on connect
/// [{"activities":[],"client_info":{"client":"web","os":"other","version":0},"session_id":"ab5941b50d818b1f8d93b4b1b581b192","status":"online"}]
pub struct SessionsReplace {
    pub sessions: Vec<Session>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Session info for the current user
pub struct Session {
    pub activities: Option<Vec<Activity>>,
    pub client_info: ClientInfo,
    pub session_id: String, // Snowflake, but headless sessions start with 'h:'.  Should that be baked into the Snowflake struct?
    pub status: ClientStatusType,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
/// Another Client info object
/// {"client":"web","os":"other","version":0}
// Note: I don't think this one exists yet? Though I might've made a mistake and this might be a duplicate
pub struct ClientInfo {
    pub client: Option<ClientType>,
    pub os: Option<OperatingSystem>,
    pub version: u8,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OperatingSystem {
    Windows,
    #[serde(rename = "osx")]
    MacOs,
    Linux,
    Android,
    IOS,
    Playstation,
    #[default]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Desktop,
    Web,
    Mobile,
    #[default]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
pub enum ClientStatusType {
    Online,
    Idle,
    Dnd,
    /// Can only be sent, and never received
    Invisible,
    /// Can only be received, and never sent
    Offline,
    /// This value can only be sent, and is used when the user's initial presence is unknown and should be assigned by the Gateway.
    #[default]
    Unknown,
}
