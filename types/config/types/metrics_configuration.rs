use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsConfiguration {
    pub timeout: u64,
}

impl Default for MetricsConfiguration {
    fn default() -> Self {
        Self { timeout: 30000 }
    }
}
