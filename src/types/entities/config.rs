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
    // RAGC: not sure about this, but it performs an "expensive" to_string operation, resulting in
    // "borrowed -> owned" ownership
    pub fn as_string(&self) -> Option<String> {
        let v = self.value.as_ref()?;
        let v = v.as_str()?;
        Some(v.to_string())
    }

    // RAGC: Is this proper naming?
    // If you check https://rust-lang.github.io/api-guidelines/naming.html#c-conv
    //
    // as_* should be "borrowed -> borrowed" ownership;
    // This has "borrowed -> owned" ownership, yet isn't a to_*, because it isn't expensive.
    // It seems the inner serde type has the same issue, so I am happy to just leave this be
    pub fn as_bool(&self) -> Option<bool> {
        let v = self.value.as_ref()?;
        let v = v.as_bool()?;
        Some(v)
    }

    pub fn as_int(&self) -> Option<i64> {
        let v = self.value.as_ref()?;
        let v = v.as_i64()?;
        Some(v)
    }
}
