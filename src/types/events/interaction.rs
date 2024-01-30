// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{Interaction, WebSocketEvent};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#interaction-create>
pub struct InteractionCreate {
    #[serde(flatten)]
    pub interaction: Interaction,
}

impl WebSocketEvent for InteractionCreate {}
