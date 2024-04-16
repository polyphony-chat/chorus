// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::Snowflake;

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Officially Undocumented
///
/// Sent to the server to signify lazy loading of a guild;
/// Sent by the official client when switching to a guild or channel;
/// After this, you should receive message updates
///
/// See <https://luna.gitlab.io/discord-unofficial-docs/docs/lazy_guilds#op-14-lazy-request>
///
/// {"op":14,"d":{"guild_id":"848582562217590824","typing":true,"activities":true,"threads":true}}
pub struct LazyRequest {
    pub guild_id: Snowflake,
    pub typing: bool,
    pub activities: bool,
    pub threads: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<HashMap<String, Vec<Vec<u64>>>>,
}

impl WebSocketEvent for LazyRequest {}
