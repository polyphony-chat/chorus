// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{Snowflake, WebSocketEvent};
use chorus_macros::WebSocketEvent;

#[derive(
    Debug,
    Deserialize,
    Serialize,
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
/// See <https://discord.com/developers/docs/topics/gateway-events#webhooks-update>
pub struct WebhooksUpdate {
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
}
