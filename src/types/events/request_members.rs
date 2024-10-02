// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, Snowflake};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, WebSocketEvent, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#request-guild-members-request-guild-members-structure>
pub struct GatewayRequestGuildMembers {
    pub guild_id: Snowflake,
    pub query: Option<String>,
    pub limit: u64,
    pub presences: Option<bool>,
    // TODO: allow array
    pub user_ids: Option<Snowflake>,
    pub nonce: Option<String>,
}
