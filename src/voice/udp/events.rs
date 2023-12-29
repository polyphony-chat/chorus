use discortp::{rtcp::Rtcp, rtp::Rtp};

use crate::{gateway::GatewayEvent, types::WebSocketEvent};

impl WebSocketEvent for Rtp {}
impl WebSocketEvent for Rtcp {}

#[derive(Debug)]
pub struct VoiceUDPEvents {
    pub rtp: GatewayEvent<Rtp>,
    pub rtcp: GatewayEvent<Rtcp>,
}

impl Default for VoiceUDPEvents {
    fn default() -> Self {
        Self {
            rtp: GatewayEvent::new(),
            rtcp: GatewayEvent::new(),
        }
    }
}
