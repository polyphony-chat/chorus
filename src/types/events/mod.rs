use serde::{Deserialize, Serialize};

mod application;
mod auto_moderation;
mod call;
mod channel;
mod guild;
mod heartbeat;
mod hello;
mod identify;
mod integration;
mod interaction;
mod invite;
mod lazy_request;
mod message;
mod passive_update;
mod presence;
mod ready;
mod relationship;
mod request_members;
mod resume;
mod session;
mod stage_instance;
mod thread;
mod user;
mod voice;
mod webhooks;

pub use application::*;
pub use auto_moderation::*;
pub use call::*;
pub use channel::*;
pub use guild::*;
pub use heartbeat::*;
pub use hello::*;
pub use identify::*;
pub use integration::*;
pub use interaction::*;
pub use invite::*;
pub use lazy_request::*;
pub use message::*;
pub use passive_update::*;
pub use presence::*;
pub use ready::*;
pub use relationship::*;
pub use request_members::*;
pub use resume::*;
pub use session::*;
pub use stage_instance::*;
pub use thread::*;
pub use user::*;
pub use voice::*;
pub use webhooks::*;

pub trait WebSocketEvent {}

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the gateway
///
/// Similar to [GatewayReceivePayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to send the event name
pub struct GatewaySendPayload {
    pub op: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
}

impl WebSocketEvent for GatewaySendPayload {}

#[derive(Debug, Default, Deserialize, Clone)]
/// The payload used for receiving events from the gateway
///
/// Similar to [GatewaySendPayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to sent the event name

pub struct GatewayReceivePayload<'a> {
    pub op: u8,
    #[serde(borrow)]
    pub d: Option<&'a serde_json::value::RawValue>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

impl<'a> WebSocketEvent for GatewayReceivePayload<'a> {}
