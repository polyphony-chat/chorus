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
    // RAGC: not sure about this, but it performs an "expensive" to_string opeartion, resulting in
    // "borrowed -> owned" ownership
    pub fn to_string(&self) -> Option<String> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        Some(v.as_str().expect("value is not a string").to_string())
    }

    // RAGC: Is this proper naming?
    // If you check https://rust-lang.github.io/api-guidelines/naming.html#c-conv
    //
    // as_* should be "borrowed -> borrowed" ownership;
    // This has "borrowed -> owned" ownership, yet isn't a to_*, because it isn't expensive.
    // It seems the inner serde type has the same issue, so I am happy to just leave this be
    pub fn as_bool(&self) -> Option<bool> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        Some(v.as_bool().expect("value is not a boolean"))
    }

    pub fn as_int(&self) -> Option<i64> {
        let Some(v) = self.value.as_ref() else {
            return None;
        };
        Some(v.as_i64().expect("value is not a number"))
    }
}
