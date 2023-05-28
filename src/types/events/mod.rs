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
mod voice;
mod session;
mod webhooks;
mod passive_update;
mod integration;
mod invite;
mod call;
mod lazy_request;
mod relationship;
mod auto_moderation;
mod stage_instance;
mod interaction;
mod application;

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
pub use voice::*;
pub use session::*;
pub use webhooks::*;
pub use passive_update::*;
pub use integration::*;
pub use invite::*;
pub use call::*;
pub use lazy_request::*;
pub use relationship::*;
pub use auto_moderation::*;
pub use stage_instance::*;
pub use interaction::*;
pub use application::*;

pub trait WebSocketEvent {}

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the gateway
/// 
/// Similar to [GatewayReceivePayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to send the event name
pub struct GatewaySendPayload {
    #[serde(rename = "op")]
    pub op_code: u8,

    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<serde_json::Value>,

    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u64>,
}

impl WebSocketEvent for GatewaySendPayload {}

#[derive(Debug, Default, Deserialize, Clone)]
/// The payload used for receiving events from the gateway
/// 
/// Similar to [GatewaySendPayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to sent the event name

pub struct GatewayReceivePayload<'a> {
    #[serde(rename = "op")]
    pub op_code: u8,

    #[serde(borrow)]
    #[serde(rename = "d")]
    pub event_data: Option<&'a serde_json::value::RawValue>,

    #[serde(rename = "s")]
    pub sequence_number: Option<u64>,

    #[serde(rename = "t")]
    pub event_name: Option<String>,
}

impl<'a> WebSocketEvent for GatewayReceivePayload<'a> {}
