use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// Officially Undocumented
///
/// Sent to the server to signify lazy loading of a guild;
/// Sent by the official client when switching to a guild or channel;
/// After this, you should recieve message updates
///
/// See https://luna.gitlab.io/discord-unofficial-docs/lazy_guilds.html#op-14-lazy-request
///
/// {"op":14,"d":{"guild_id":"848582562217590824","typing":true,"activities":true,"threads":true}}
pub struct LazyRequest {
    pub guild_id: String,
    pub typing: bool,
    pub activities: bool,
    pub threads: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<HashMap<String, Vec<Vec<u64>>>>,
}
impl WebSocketEvent for LazyRequest {}
