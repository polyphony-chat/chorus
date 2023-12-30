use discortp::discord::IpDiscovery;

use crate::types::{SessionDescription, Snowflake, VoiceReady, VoiceServerUpdate};

#[derive(Debug, Default)]
/// Saves data shared between parts of the voice architecture;
///
/// Struct used to give the Udp connection data received from the gateway.
pub struct VoiceData {
    pub server_data: Option<VoiceServerUpdate>,
    pub ready_data: Option<VoiceReady>,
    pub session_description: Option<SessionDescription>,
    pub user_id: Snowflake,
    pub session_id: String,
    /// The last sequence number we used, has to be incremeted by one every time we send a message
    pub last_sequence_number: u16,
    pub ip_discovery: Option<IpDiscovery>,
}
