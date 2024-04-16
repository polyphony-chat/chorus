// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use discortp::discord::IpDiscovery;

use crate::types::{SessionDescription, Snowflake, VoiceReady, VoiceServerUpdate};

#[derive(Debug, Default)]
/// Saves data shared between parts of the voice architecture;
///
/// Struct used to give the UDP connection data received from the gateway.
pub struct VoiceData {
    pub server_data: Option<VoiceServerUpdate>,
    pub ready_data: Option<VoiceReady>,
    pub session_description: Option<SessionDescription>,
    pub user_id: Snowflake,
    pub session_id: String,
    /// The last sequence number we used, has to be incremented by one every time we send a message
    pub last_sequence_number: u16,
    pub ip_discovery: Option<IpDiscovery>,

    /// The last UDP encryption nonce, if we are using an encryption mode with incremental nonces.
    pub last_udp_encryption_nonce: Option<u32>,
}
