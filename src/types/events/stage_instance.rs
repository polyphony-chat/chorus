// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{StageInstance, WebSocketEvent};
use chorus_macros::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#stage-instance-create>
pub struct StageInstanceCreate {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#stage-instance-update>
pub struct StageInstanceUpdate {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// See <https://discord.com/developers/docs/topics/gateway-events#stage-instance-delete>
pub struct StageInstanceDelete {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

