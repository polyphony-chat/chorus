use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

use super::WebrtcEncryptionMode;

#[derive(Debug, Deserialize, Serialize, Clone)]
/// An event sent by the client to the webrtc server, detailing what protocol, address and encryption to use;
///
/// See <https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection-example-select-protocol-payload>
pub struct SelectProtocol {
    /// The protocol to use. The only option detailed in discord docs is "udp"
    pub protocol: String,
    pub data: SelectProtocolData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// The data field of the SelectProtocol Event
///
/// See <https://discord.com/developers/docs/topics/voice-connections#establishing-a-voice-udp-connection-example-select-protocol-payload>
pub struct SelectProtocolData {
    /// Our external ip
    pub address: Ipv4Addr,
    /// Our external udp port
    pub port: u32,
    /// The mode of encryption to use
    pub mode: WebrtcEncryptionMode,
}
