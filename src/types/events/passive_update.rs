use serde::{Deserialize, Serialize};

use crate::types::{VoiceState, GuildMember};
use super::{ChannelUnreadUpdateObject, WebSocketEvent};

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// 
/// Seems to be passively set to update the client on guild details (though, why not just send the update events?)
pub struct PassiveUpdateV1 {
    pub voice_states: Vec<VoiceState>,
    pub members: Vec<GuildMember>,
    pub guild_id: String,
    pub channels: Vec<ChannelUnreadUpdateObject>,
}

impl WebSocketEvent for PassiveUpdateV1 {}