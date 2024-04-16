// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::WebSocketEvent;
use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value};

pub use client_connect::*;
pub use client_disconnect::*;
pub use hello::*;
pub use identify::*;
pub use media_sink_wants::*;
pub use ready::*;
pub use select_protocol::*;
pub use session_description::*;
pub use speaking::*;
pub use ssrc_definition::*;
pub use voice_backend_version::*;

mod client_connect;
mod client_disconnect;
mod hello;
mod identify;
mod media_sink_wants;
mod ready;
mod select_protocol;
mod session_description;
mod speaking;
mod ssrc_definition;
mod voice_backend_version;

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the voice gateway.
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
/// The payload used for receiving events from the voice gateway.
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

/// The modes of encryption available in voice UDP connections;
///
/// Not all encryption modes are implemented; it is generally recommended
/// to use either [[VoiceEncryptionMode::Xsalsa20Poly1305]] or
/// [[VoiceEncryptionMode::Xsalsa20Poly1305Suffix]]
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#encryption-mode> and <https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection-encryption-modes>
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceEncryptionMode {
    #[default]
    // Officially Documented
    /// Use XSalsa20Poly1305 encryption, using the rtp header as a nonce.
    ///
    /// Fully implemented
    Xsalsa20Poly1305,
    /// Use XSalsa20Poly1305 encryption, using a random 24 byte suffix as a nonce.
    ///
    /// Fully implemented
    Xsalsa20Poly1305Suffix,
    /// Use XSalsa20Poly1305 encryption, using a 4 byte incremental value as a nonce.
    ///
    /// Fully implemented
    Xsalsa20Poly1305Lite,
    // Officially Undocumented
    /// Not implemented yet, we have no idea what the rtpsize nonces are.
    Xsalsa20Poly1305LiteRtpsize,
    /// Not implemented yet, we have no idea what the nonce is.
    AeadAes256Gcm,
    /// Not implemented yet, we have no idea what the rtpsize nonces are.
    AeadAes256GcmRtpsize,
    /// Not implemented yet, we have no idea what the rtpsize nonces are.
    AeadXchacha20Poly1305Rtpsize,
}

impl VoiceEncryptionMode {
    /// Returns whether this encryption mode uses Xsalsa20Poly1305 encryption.
    pub fn is_xsalsa20_poly1305(&self) -> bool {
        matches!(
            *self,
            VoiceEncryptionMode::Xsalsa20Poly1305
                | VoiceEncryptionMode::Xsalsa20Poly1305Lite
                | VoiceEncryptionMode::Xsalsa20Poly1305Suffix
                | VoiceEncryptionMode::Xsalsa20Poly1305LiteRtpsize
        )
    }

    /// Returns whether this encryption mode uses AeadAes256Gcm encryption.
    pub fn is_aead_aes256_gcm(&self) -> bool {
        matches!(
            *self,
            VoiceEncryptionMode::AeadAes256Gcm | VoiceEncryptionMode::AeadAes256GcmRtpsize
        )
    }

    /// Returns whether this encryption mode uses AeadXchacha20Poly1305 encryption.
    pub fn is_aead_xchacha20_poly1305(&self) -> bool {
        *self == VoiceEncryptionMode::AeadXchacha20Poly1305Rtpsize
    }
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
pub const VOICE_SSRC_DEFINITION: u8 = 12;
pub const VOICE_CLIENT_DISCONNECT: u8 = 13;
pub const VOICE_SESSION_UPDATE: u8 = 14;

/// What is this?
///
/// {"op":15,"d":{"any":100}}
///
/// Opcode from <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes>
pub const VOICE_MEDIA_SINK_WANTS: u8 = 15;
/// See <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#voice-opcodes>
/// Sent with empty data from the client, the server responds with the voice backend version;
pub const VOICE_BACKEND_VERSION: u8 = 16;

// These two get simultaenously fired when a user joins, one has flags and one has a platform
pub const VOICE_CLIENT_CONNECT_FLAGS: u8 = 18;
pub const VOICE_CLIENT_CONNECT_PLATFORM: u8 = 20;
