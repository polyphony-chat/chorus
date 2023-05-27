use serde::{Deserialize, Serialize};
use crate::entities::User;
use crate::events::WebSocketEvent;

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#user-update
/// Not directly serialized, as the inner payload is the user object
pub struct UserUpdate {
    pub user: User,
}

impl WebSocketEvent for UserUpdate {}