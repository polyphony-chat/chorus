// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deserialize, Serialize, WebSocketEvent, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct GatewayHeartbeat {
    pub op: u8,
    pub d: Option<u64>,
}

impl GatewayHeartbeat {
    /// The Heartbeat packet a server would receive from a new or fresh Gateway connection.
    pub fn first() -> Self {
        Self::default()
    }

    /// Quickly create a [GatewayHeartbeat] with the correct `opcode` and the given `sequence_number`.
    ///
    /// Shorthand for
    /// ```
    /// # use chorus::types::GatewayHeartbeat;
    /// # let sequence_number: u64 = 1;
    /// let heatbeat = GatewayHeartbeat {
    ///     op: 1,
    ///     d: Some(sequence_number)
    /// };
    /// ```
    pub fn new(sequence_number: u64) -> Self {
        Self {
            op: 1,
            d: Some(sequence_number),
        }
    }
}

impl std::default::Default for GatewayHeartbeat {
    fn default() -> Self {
        Self { op: 1, d: None }
    }
}

#[derive(
    Debug, Deserialize, Serialize, Clone, WebSocketEvent, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct GatewayHeartbeatAck {
    pub op: i32,
}

impl std::default::Default for GatewayHeartbeatAck {
    fn default() -> Self {
        Self { op: 11 }
    }
}
