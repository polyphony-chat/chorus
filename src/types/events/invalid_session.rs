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
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Copy,
)]
/// Your session is now invalid.
///
/// Either reauthenticate and reidentify or resume if possible.
///
/// # Reference
/// See <https://docs.discord.food/topics/gateway-events#invalid-session>
pub struct GatewayInvalidSession {
    #[serde(rename = "d")]
    pub resumable: bool,
}
