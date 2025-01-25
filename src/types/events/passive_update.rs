// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use super::{ChannelUnreadUpdateObject, WebSocketEvent};
use crate::types::{GuildMember, Snowflake, VoiceState};

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Default, WebSocketEvent)]
/// Officially Undocumented
///
/// Seems to be passively set to update the client on guild details (though, why not just send the update events?)
pub struct PassiveUpdateV1 {
    #[serde(default)]
    pub voice_states: Vec<VoiceState>,
    #[serde(default)]
    pub members: Vec<GuildMember>,
    pub guild_id: Snowflake,
    #[serde(default)]
    pub channels: Vec<ChannelUnreadUpdateObject>,
}
