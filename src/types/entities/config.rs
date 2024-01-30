// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct ConfigEntity {
    pub key: String,
    pub value: Option<Value>,
}

impl ConfigEntity {
    pub fn as_string(&self) -> Option<String> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        let Some(v) = v.as_str() else {
            return None;
        };
        Some(v.to_string())
    }

    pub fn as_bool(&self) -> Option<bool> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        let Some(v) = v.as_bool() else {
            return None;
        };
        Some(v)
    }

    pub fn as_int(&self) -> Option<i64> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        let Some(v) = v.as_i64() else {
            return None;
        };
        Some(v)
    }
}
