use crate::types::{MfaAuthenticator, MfaAuthenticatorType, Snowflake, WebSocketEvent};
use chorus_macros::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, WebSocketEvent)]
/// See <https://docs.discord.sex/topics/gateway-events#authenticator-create>;
///
/// Sent when an [MfaAuthenticator] is created.
pub struct AuthenticatorCreate {
    #[serde(flatten)]
    pub authenticator: MfaAuthenticator,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, WebSocketEvent)]
/// See <https://docs.discord.sex/topics/gateway-events#authenticator-update>;
///
/// Sent when an [MfaAuthenticator] is modified.
pub struct AuthenticatorUpdate {
    #[serde(flatten)]
    pub authenticator: MfaAuthenticator,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, WebSocketEvent)]
/// See <https://docs.discord.sex/topics/gateway-events#authenticator-delete>;
///
/// Sent when an [MfaAuthenticator] is deleted.
pub struct AuthenticatorDelete {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub authenticator_type: MfaAuthenticatorType,
}