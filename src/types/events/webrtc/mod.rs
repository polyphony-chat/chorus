use super::WebSocketEvent;
use serde::{Deserialize, Serialize};

pub use identify::*;
pub use ready::*;
pub use select_protocol::*;
pub use session_description::*;
pub use speaking::*;

mod identify;
mod ready;
mod select_protocol;
mod session_description;
mod speaking;

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the webrtc gateway
/// Not tha this is very similar to the regular gateway, except we no longer have a sequence number
///
/// Similar to [WebrtcReceivePayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to send the event name
pub struct VoiceGatewaySendPayload {
    #[serde(rename = "op")]
    pub op_code: u8,

    #[serde(rename = "d")]
    pub data: serde_json::Value,
}

impl WebSocketEvent for VoiceGatewaySendPayload {}

#[derive(Debug, Deserialize, Clone)]
/// The payload used for receiving events from the webrtc gateway
/// Note that this is very similar to the regular gateway, except we no longer have s or t
///
/// Similar to [WebrtcSendPayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to sent the event name
pub struct VoiceGatewayReceivePayload<'a> {
    #[serde(rename = "op")]
    pub op_code: u8,

    #[serde(borrow)]
    #[serde(rename = "d")]
    pub data: &'a serde_json::value::RawValue,
}

impl<'a> WebSocketEvent for VoiceGatewayReceivePayload<'a> {}

/// The modes of encryption available in webrtc connections;
/// See https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection-encryption-modes;
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebrtcEncryptionMode {
    #[default]
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
/// See https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes
pub const VOICE_VIDEO: u8 = 12;
pub const VOICE_CLIENT_DISCONENCT: u8 = 13;
/// See https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes;
/// Sent with empty data from the client, the server responds with the voice backend version;
pub const VOICE_BACKEND_VERSION: u8 = 16;
