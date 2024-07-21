// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
pub use invalid_session::*;
pub use invite::*;
pub use lazy_request::*;
pub use message::*;
pub use passive_update::*;
pub use presence::*;
pub use ready::*;
pub use reconnect::*;
pub use relationship::*;
pub use request_members::*;
pub use resume::*;
pub use session::*;
pub use stage_instance::*;
pub use thread::*;
pub use user::*;
pub use voice::*;
pub use voice_gateway::*;
pub use webhooks::*;

use chorus_macros::WebSocketEvent;

#[cfg(feature = "client")]
use super::Snowflake;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use serde_json::{from_str, from_value, to_value, Value};

#[cfg(feature = "client")]
use std::collections::HashMap;

#[cfg(feature = "client")]
use crate::types::Shared;
use std::fmt::Debug;

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
mod invalid_session;
mod invite;
mod lazy_request;
mod message;
mod passive_update;
mod presence;
mod ready;
mod reconnect;
mod relationship;
mod request_members;
mod resume;
mod session;
mod stage_instance;
mod thread;
mod user;
mod voice;
mod webhooks;

mod voice_gateway;

pub trait WebSocketEvent: Send + Sync + Debug {}

#[derive(Debug, Default, Serialize, Clone, WebSocketEvent)]
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
    fn update(&mut self, object_to_update: Shared<T>) {
        update_object(self.get_json(), object_to_update)
    }
    #[cfg(not(tarpaulin_include))]
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
    object: Shared<(impl Updateable + Serialize + DeserializeOwned + Clone)>,
) {
    let data_from_event: HashMap<String, Value> = from_str(&value).unwrap();
    let mut original_data: HashMap<String, Value> =
        from_value(to_value(object.clone()).unwrap()).unwrap();
    for (updated_entry_key, updated_entry_value) in data_from_event.into_iter() {
        original_data.insert(updated_entry_key.clone(), updated_entry_value);
    }
    *object.write().unwrap() = from_value(to_value(original_data).unwrap()).unwrap();
}

#[derive(
    Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, PartialOrd, Eq, Ord, Copy,
)]
/// Enum representing all possible* event types that can be received from or sent to the gateway.
///
/// *: This list might not be exhaustive. If you notice an event type is missing,
/// please open a PR.
pub enum EventType {
    Hello,
    Ready,
    Resumed,
    InvalidSession,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    RequestMembers,
}

#[derive(Debug, Clone)]
/// Enum representing all possible* events that can be received from or sent to the gateway.
///
/// *: This list might not be exhaustive. If you notice an event is missing,
/// please open a PR.
pub enum Event {
    Hello(GatewayHello),
    Ready(GatewayReady),
    Resumed(GatewayResume),
    InvalidSession(GatewayInvalidSession),
    ChannelCreate(ChannelCreate),
    ChannelUpdate(ChannelUpdate),
    ChannelDelete(ChannelDelete),
    ChannelPinsUpdate(ChannelPinsUpdate),
    ThreadCreate(ThreadCreate),
    ThreadUpdate(ThreadUpdate),
    ThreadDelete(ThreadDelete),
    ThreadListSync(ThreadListSync),
    ThreadMemberUpdate(ThreadMemberUpdate),
    ThreadMembersUpdate(ThreadMembersUpdate),
    GuildCreate(GuildCreate),
    GuildUpdate(GuildUpdate),
    GuildDelete(GuildDelete),
    GuildBanAdd(GuildBanAdd),
    GuildBanRemove(GuildBanRemove),
    GuildEmojisUpdate(GuildEmojisUpdate),
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    GuildMemberAdd(GuildMemberAdd),
    GuildMemberRemove(GuildMemberRemove),
    GuildMemberUpdate(GuildMemberUpdate),
    GuildMembersChunk(GuildMembersChunk),
    InteractionCreate(InteractionCreate),
    InviteCreate(InviteCreate),
    InviteDelete(InviteDelete),
    MessageCreate(MessageCreate),
    MessageUpdate(MessageUpdate),
    MessageDelete(MessageDelete),
    MessageDeleteBulk(MessageDeleteBulk),
    MessageReactionAdd(MessageReactionAdd),
    MessageReactionRemove(MessageReactionRemove),
    MessageReactionRemoveAll(MessageReactionRemoveAll),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmoji),
    PresenceUpdate(PresenceUpdate),
    TypingStart(TypingStartEvent),
    UserUpdate(UserUpdate),
    VoiceStateUpdate(VoiceStateUpdate),
    VoiceServerUpdate(VoiceServerUpdate),
    WebhooksUpdate(WebhooksUpdate),
    StageInstanceCreate(StageInstanceCreate),
    StageInstanceUpdate(StageInstanceUpdate),
    StageInstanceDelete(StageInstanceDelete),
    RequestMembers(GatewayRequestGuildMembers),
}
