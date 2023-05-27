use crate::types::entities::User;
use crate::types::events::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#user-update
pub struct UserUpdate {
    #[serde(flatten)]
    pub user: User,
}

impl WebSocketEvent for UserUpdate {}
