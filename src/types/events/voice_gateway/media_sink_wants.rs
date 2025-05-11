// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::WebSocketEvent;
use chorus_macros::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy, WebSocketEvent)]
/// What does this do?
///
/// {"op":15,"d":{"any":100}}
///
/// Opcode from <https://docs.discord.food/topics/opcodes-and-status-codes#voice-opcodes>
pub struct VoiceMediaSinkWants {
    pub any: u16,
}

