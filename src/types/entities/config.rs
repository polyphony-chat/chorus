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
        Some(v.as_str().expect("value is not a string").to_string())
    }

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
