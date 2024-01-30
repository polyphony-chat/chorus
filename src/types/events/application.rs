// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{GuildApplicationCommandPermissions, WebSocketEvent};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#application-command-permissions-update>
pub struct ApplicationCommandPermissionsUpdate {
    #[serde(flatten)]
    pub permissions: GuildApplicationCommandPermissions,
}

impl WebSocketEvent for ApplicationCommandPermissionsUpdate {}
