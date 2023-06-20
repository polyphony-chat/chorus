use serde::{Deserialize, Serialize};

use super::{ChannelUnreadUpdateObject, WebSocketEvent};
use crate::types::{GuildMember, Snowflake, VoiceState};

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
///
/// Seems to be passively set to update the client on guild details (though, why not just send the update events?)
pub struct PassiveUpdateV1 {
    pub voice_states: Vec<VoiceState>,
    pub members: Option<Vec<GuildMember>>,
    pub guild_id: Snowflake,
    pub channels: Vec<ChannelUnreadUpdateObject>,
}

impl WebSocketEvent for PassiveUpdateV1 {}
