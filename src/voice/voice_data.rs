use discortp::discord::IpDiscovery;

use crate::types::{SessionDescription, Snowflake, VoiceReady, VoiceServerUpdate};

#[derive(Debug, Default)]
/// Saves data shared between parts of the voice architecture
pub struct VoiceData {
    pub server_data: Option<VoiceServerUpdate>,
    pub ready_data: Option<VoiceReady>,
    pub session_description: Option<SessionDescription>,
    pub user_id: Snowflake,
    pub session_id: String,
    pub ip_discovery: Option<IpDiscovery>,
}
