// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// What does this do?
///
/// {"op":15,"d":{"any":100}}
///
/// Opcode from <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes>
pub struct VoiceMediaSinkWants {
    pub any: u16,
}

impl WebSocketEvent for VoiceMediaSinkWants {}
