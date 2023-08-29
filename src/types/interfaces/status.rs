use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#client-status-object>
pub struct ClientStatusObject {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}
