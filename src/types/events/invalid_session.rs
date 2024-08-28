use serde::{Deserialize, Serialize};

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
/// Your session is now invalid.
///
/// Either reauthenticate and reidentify or resume if possible.
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#invalid-session>
pub struct GatewayInvalidSession {
    #[serde(rename = "d")]
    pub resumable: bool,
}

