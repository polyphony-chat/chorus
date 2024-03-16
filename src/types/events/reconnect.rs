use serde::{Deserialize, Serialize};

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// "The reconnect event is dispatched when a client should reconnect to the Gateway (and resume their existing session, if they have one). This event usually occurs during deploys to migrate sessions gracefully off old hosts"
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#reconnect>
pub struct GatewayReconnect {}

impl WebSocketEvent for GatewayReconnect {}
