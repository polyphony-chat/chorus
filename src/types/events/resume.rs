// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

/// Used to replay missed events when a disconnected client resumes.
///
/// # Reference
/// See <https://docs.discord.food/topics/gateway-events#resume>
#[derive(Debug, Clone, Deserialize, Serialize, Default, WebSocketEvent)]
pub struct GatewayResume {
    pub token: String,
    /// Existing session id
    pub session_id: String,
    /// Last sequence number received
    pub seq: String,
}

/// Sent in response to a [GatewayResume].
///
/// Signifies the end of event replaying.
///
/// # Reference
/// See <https://docs.discord.food/topics/gateway-events#resumed>
#[derive(Debug, Clone, Deserialize, Serialize, Default, WebSocketEvent)]
pub struct GatewayResumed {
    #[serde(rename = "_trace")]
    pub trace: Vec<String>,
}
