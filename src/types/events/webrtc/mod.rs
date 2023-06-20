pub use identify::*;
pub use ready::*;
pub use select_protocol::*;
use serde::{Deserialize, Serialize};

mod identify;
mod ready;
mod select_protocol;

/// The modes of encryption available in webrtc connections;
/// See https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection-encryption-modes;
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum WebrtcEncryptionMode {
    XSalsa20Poly1305,
    XSalsa20Poly1305Suffix,
    XSalsa20Poly1305Lite,
}

// The various voice opcodes
pub const VOICE_IDENTIFY: u8 = 0;
pub const VOICE_SELECT_PROTOCOL: u8 = 1;
pub const VOICE_READY: u8 = 2;
pub const VOICE_HEARTBEAT: u8 = 3;
pub const VOICE_SESSION_DESCRIPTION: u8 = 4;
pub const VOICE_SPEAKING: u8 = 5;
pub const VOICE_HEARTBEAT_ACK: u8 = 6;
pub const VOICE_RESUME: u8 = 7;
pub const VOICE_HELLO: u8 = 8;
pub const VOICE_RESUMED: u8 = 9;
pub const VOICE_CLIENT_DISCONENCT: u8 = 13;
