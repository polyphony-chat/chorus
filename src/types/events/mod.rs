use serde::{Deserialize, Serialize};

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

use crate::gateway::Updateable;

use super::Snowflake;

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

/// An [`UpdateMessage<T>`] represents a received Gateway Message which contains updated
/// information for an [`Updateable`] of Type T.
/// # Example:
/// ```rs
/// impl UpdateMessage<Channel> for ChannelUpdate {
///     fn update(...) {...}
///     fn id(...) {...}
/// }
/// ```
/// This would imply, that the [`WebSocketEvent`] "[`ChannelUpdate`]" contains new/updated information
/// about a [`Channel`]. The update method describes how this new information will be turned into
/// a [`Channel`] object.
pub(crate) trait UpdateMessage<T>: Clone
where
    T: Updateable,
{
    fn update(&self, object_to_update: &mut T);
    fn id(&self) -> Snowflake;
}
