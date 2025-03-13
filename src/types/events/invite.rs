// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{GuildInvite, Snowflake, WebSocketEvent};
use chorus_macros::WebSocketEvent;

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#invite-create>
pub struct InviteCreate {
    #[serde(flatten)]
    pub invite: GuildInvite,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#invite-delete>
pub struct InviteDelete {
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub code: String,
}

