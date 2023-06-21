use serde::{Deserialize, Serialize};
use crate::types::WebSocketEvent;
use super::WebrtcEncryptionMode;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// Event that describes our encryption mode and secret key for encryption
pub struct SessionDescription {
    /// The encryption mode we're using in webrtc
    pub mode: WebrtcEncryptionMode,
    /// The secret key we'll use for encryption
    pub secret_key: [u8; 32],
}

impl WebSocketEvent for SessionDescription {}