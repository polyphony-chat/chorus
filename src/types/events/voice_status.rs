use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#update-voice-state-gateway-voice-state-update-structure
pub struct GatewayVoiceStateUpdate {
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

impl WebSocketEvent for GatewayVoiceStateUpdate {}
