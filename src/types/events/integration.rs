// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{Integration, Snowflake, WebSocketEvent};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#integration-create>
pub struct IntegrationCreate {
    #[serde(flatten)]
    pub integration: Integration,
    pub guild_id: Snowflake,
}

impl WebSocketEvent for IntegrationCreate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#integration-update>
pub struct IntegrationUpdate {
    #[serde(flatten)]
    pub integration: Integration,
    pub guild_id: Snowflake,
}

impl WebSocketEvent for IntegrationUpdate {}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/topics/gateway-events#integration-delete>
pub struct IntegrationDelete {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub application_id: Option<Snowflake>,
}

impl WebSocketEvent for IntegrationDelete {}
