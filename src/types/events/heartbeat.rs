// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, WebSocketEvent, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GatewayHeartbeat {
    pub op: u8,
    pub d: Option<u64>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GatewayHeartbeatAck {
    pub op: i32,
}

