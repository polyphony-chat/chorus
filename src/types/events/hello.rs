// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::WebSocketEvent;
use chorus_macros::WebSocketEvent;
use serde::{Deserialize, Serialize};

/// Received on gateway init, tells the client how often to send heartbeats;
#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, WebSocketEvent, Copy, Hash, PartialOrd, Ord,
)]
pub struct GatewayHello {
    pub op: i32,
    pub d: HelloData,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Copy, WebSocketEvent, Hash, PartialOrd, Ord,
)]
/// Contains info on how often the client should send heartbeats to the server;
pub struct HelloData {
    /// How often a client should send heartbeats, in milliseconds
    pub heartbeat_interval: u64,
}

impl std::default::Default for GatewayHello {
    fn default() -> Self {
        Self {
            // "HELLO" opcode is 10
            op: 10,
            d: Default::default(),
        }
    }
}

impl std::default::Default for HelloData {
    fn default() -> Self {
        Self {
            // Discord docs mention 45000 seconds - discord.food mentions 41250. Defaulting to 45s
            heartbeat_interval: 45000,
        }
    }
}
