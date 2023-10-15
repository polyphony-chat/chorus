use super::WebSocketEvent;
use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value};

pub use client_connect::*;
pub use client_disconnect::*;
pub use hello::*;
pub use identify::*;
pub use ready::*;
pub use select_protocol::*;
pub use session_description::*;
pub use speaking::*;
pub use voice_backend_version::*;

mod client_connect;
mod client_disconnect;
mod hello;
mod identify;
mod ready;
mod select_protocol;
mod session_description;
mod speaking;
mod voice_backend_version;

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the webrtc gateway.
///
/// Similar to [VoiceGatewayReceivePayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
pub struct VoiceGatewaySendPayload {
    #[serde(rename = "op")]
    pub op_code: u8,

    #[serde(rename = "d")]
    pub data: Value,
}

impl WebSocketEvent for VoiceGatewaySendPayload {}

#[derive(Debug, Deserialize, Clone)]
/// The payload used for receiving events from the webrtc gateway.
///
/// Note that this is similar to the regular gateway, except we no longer have s or t
///
/// Similar to [VoiceGatewaySendPayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
pub struct VoiceGatewayReceivePayload<'a> {
    #[serde(rename = "op")]
    pub op_code: u8,

    #[serde(borrow)]
    #[serde(rename = "d")]
    pub data: &'a RawValue,
}

impl<'a> WebSocketEvent for VoiceGatewayReceivePayload<'a> {}

/// The modes of encryption available in webrtc connections;
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#encryption-mode> and <https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection-encryption-modes>
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceEncryptionMode {
    #[default]
    // Officially Documented
    Xsalsa20Poly1305,
    Xsalsa20Poly1305Suffix,
    Xsalsa20Poly1305Lite,
    // Officially Undocumented
    Xsalsa20Poly1305LiteRtpsize,
    AeadAes256Gcm,
    AeadAes256GcmRtpsize,
    AeadXchacha20Poly1305Rtpsize,
}

/// The possible audio codecs to use
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioCodec {
    #[default]
    Opus,
}

/// The possible video codecs to use
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum VideoCodec {
    #[default]
    VP8,
    VP9,
    H264,
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
/// See <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes>
pub const VOICE_VIDEO: u8 = 12;
pub const VOICE_CLIENT_DISCONNECT: u8 = 13;
pub const VOICE_SESSION_UPDATE: u8 = 14;
/// See <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes>
/// Sent with empty data from the client, the server responds with the voice backend version;
pub const VOICE_BACKEND_VERSION: u8 = 16;

// These two get simultaenously fired when a user joins, one has flags and one has a platform
pub const VOICE_CLIENT_CONNECT_FLAGS: u8 = 18;
pub const VOICE_CLIENT_CONNECT_PLATFORM: u8 = 20;
