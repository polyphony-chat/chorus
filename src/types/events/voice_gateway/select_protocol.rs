// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use super::VoiceEncryptionMode;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// An event sent by the client to the voice gateway server,
/// detailing what protocol, address and encryption to use;
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#select-protocol-structure>
pub struct SelectProtocol {
    /// The protocol to use. The only option chorus supports is [VoiceProtocol::Udp].
    pub protocol: VoiceProtocol,
    pub data: SelectProtocolData,
    /// The UUID4 RTC connection ID, used for analytics.
    ///
    /// Note: Not recommended to set this
    pub rtc_connection_id: Option<String>,
    // TODO: Add codecs, what is a codec object
    /// The possible experiments we want to enable
    #[serde(rename = "experiments")]
    pub enabled_experiments: Vec<String>,
}

/// The possible protocol for sending a receiving voice data.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#select-protocol-structure>
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceProtocol {
    #[default]
    /// Sending data via UDP, documented and the only protocol chorus supports.
    Udp,
    // Possible value, yet NOT RECOMMENDED, AS CHORUS DOES NOT SUPPORT WEBRTC
    //Webrtc,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// The data field of the SelectProtocol Event
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#protocol-data-structure>
pub struct SelectProtocolData {
    /// Our external IP we got from IP discovery
    pub address: String,
    /// Our external UDP port we got from IP discovery
    pub port: u16,
    /// The mode of encryption to use
    pub mode: VoiceEncryptionMode,
}
