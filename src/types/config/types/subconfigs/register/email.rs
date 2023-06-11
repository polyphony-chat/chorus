use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrationEmailConfiguration {
    pub required: bool,
    pub allowlist: bool,
    #[serde(default)]
    pub blacklist: bool,
    #[serde(default)]
    pub domains: Vec<String>,
}

impl Default for RegistrationEmailConfiguration {
    fn default() -> Self {
        Self {
            required: false,
            allowlist: false,
            blacklist: true,
            domains: Vec::new(),
        }
    }
}
