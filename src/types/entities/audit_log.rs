// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::gateway::Shared;
use crate::types::utils::Snowflake;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See <https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object>
pub struct AuditLogEntry {
    pub target_id: Option<String>,
    pub changes: Option<Vec<Shared<AuditLogChange>>>,
    pub user_id: Option<Snowflake>,
    pub id: Snowflake,
    // to:do implement an enum for these types
    pub action_type: u8,
    // to:do add better options type
    pub options: Option<serde_json::Value>,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See <https://discord.com/developers/docs/resources/audit-log#audit-log-change-object>
pub struct AuditLogChange {
    pub new_value: Option<serde_json::Value>,
    pub old_value: Option<serde_json::Value>,
    pub key: String,
}
