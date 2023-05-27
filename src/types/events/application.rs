use serde::{Deserialize, Serialize};

use crate::types::{WebSocketEvent, GuildApplicationCommandPermissions};

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#application-command-permissions-update
pub struct ApplicationCommandPermissionsUpdate {
    #[serde(flatten)]
    pub permissions: GuildApplicationCommandPermissions,
}

impl WebSocketEvent for ApplicationCommandPermissionsUpdate  {}