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
pub use webrtc::*;

#[cfg(feature = "client")]
use super::Snowflake;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use serde_json::{from_str, from_value, to_value, Value};

#[cfg(feature = "client")]
use std::collections::HashMap;

#[cfg(feature = "client")]
use std::sync::{Arc, RwLock};

#[cfg(feature = "client")]
use serde::de::DeserializeOwned;

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

mod webrtc;

pub trait WebSocketEvent {}

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the gateway
///
/// Similar to [GatewayReceivePayload], except we send a [serde_json::value::Value] for d whilst we receive a [serde_json::value::RawValue]
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

#[cfg(feature = "client")]
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
pub(crate) trait UpdateMessage<T>: Clone + JsonField + SourceUrlField
where
    T: Updateable + Serialize + DeserializeOwned + Clone,
{
    fn update(&mut self, object_to_update: Arc<RwLock<T>>) {
        update_object(self.get_json(), object_to_update)
    }
    fn id(&self) -> Option<Snowflake>;
}

pub(crate) trait JsonField: Clone {
    fn set_json(&mut self, json: String);
    fn get_json(&self) -> String;
}

pub trait SourceUrlField: Clone {
    fn set_source_url(&mut self, url: String);
    fn get_source_url(&self) -> String;
}

#[cfg(feature = "client")]
/// Only applicable for events where the Update struct is the same as the Entity struct
pub(crate) fn update_object(
    value: String,
    object: Arc<RwLock<(impl Updateable + Serialize + DeserializeOwned + Clone)>>,
) {
    let data_from_event: HashMap<String, Value> = from_str(&value).unwrap();
    let mut original_data: HashMap<String, Value> =
        from_value(to_value(object.clone()).unwrap()).unwrap();
    for (updated_entry_key, updated_entry_value) in data_from_event.into_iter() {
        original_data.insert(updated_entry_key.clone(), updated_entry_value);
    }
    *object.write().unwrap() = from_value(to_value(original_data).unwrap()).unwrap();
}
