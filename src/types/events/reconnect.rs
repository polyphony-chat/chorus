// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use super::WebSocketEvent;

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
/// "The reconnect event is dispatched when a client should reconnect to the Gateway (and resume their existing session, if they have one). This event usually occurs during deploys to migrate sessions gracefully off old hosts"
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#reconnect>
pub struct GatewayReconnect {}
