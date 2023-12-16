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
