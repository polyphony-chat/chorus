// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
    pub session_id: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Another Client info object
/// {"client":"web","os":"other","version":0}
// Note: I don't think this one exists yet? Though I might've made a mistake and this might be a duplicate
pub struct ClientInfo {
    pub client: Option<String>,
    pub os: Option<String>,
    pub version: u8,
}

