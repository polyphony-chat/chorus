pub mod gateway;
pub mod handle;
pub mod heartbeat;
use super::*;
pub use gateway::*;
pub use handle::*;
pub use heartbeat::*;
use ws_stream_wasm::WsMessage;

impl crate::gateway::MessageCapable for WsMessage {
    fn as_string(&self) -> Option<String> {
        match self {
            WsMessage::Text(text) => Some(text.clone()),
            _ => None,
        }
    }

    fn as_bytes(&self) -> Option<Vec<u8>> {
        match self {
            WsMessage::Binary(bytes) => Some(bytes.clone()),
            _ => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            WsMessage::Text(text) => text.is_empty(),
            WsMessage::Binary(bytes) => bytes.is_empty(),
            _ => false,
        }
    }

    fn from_str(s: &str) -> Self {
        WsMessage::Text(s.to_string())
    }
}
