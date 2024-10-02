// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;
use crate::UInt64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct SecurityKey {
    pub id: String,
    pub user_id: String,
    pub key_id: String,
    pub public_key: String,
    pub counter: UInt64,
    pub name: String,
}

impl Default for SecurityKey {
    fn default() -> Self {
        Self {
            id: Snowflake::generate().to_string(),
            user_id: String::new(),
            key_id: String::new(),
            public_key: String::new(),
            #[allow(clippy::useless_conversion)]
            counter: 0u64.into(),
            name: String::new(),
        }
    }
}
