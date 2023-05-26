use serde::{Deserialize, Serialize};

mod channel;
mod guild;
mod heartbeat;
mod hello;
mod identify;
mod message;
mod presence;
mod ready;
mod request_members;
mod resume;
mod thread;
mod user;
mod voice_status;

pub use channel::*;
pub use guild::*;
pub use heartbeat::*;
pub use hello::*;
pub use identify::*;
pub use message::*;
pub use presence::*;
pub use ready::*;
pub use request_members::*;
pub use resume::*;
pub use thread::*;
pub use user::*;
pub use voice_status::*;

pub trait WebSocketEvent {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Option<serde_json::Value>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

impl WebSocketEvent for GatewayPayload {}
