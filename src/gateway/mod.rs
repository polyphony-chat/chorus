// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod backends;
pub mod events;
pub mod gateway;
pub mod handle;
pub mod heartbeat;
pub mod message;
pub mod options;

pub use backends::*;
pub use gateway::*;
pub use handle::*;
use heartbeat::*;
pub use message::*;
pub use options::*;

use crate::errors::GatewayError;
use crate::types::{Opcode, Snowflake};

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::sync::Mutex;

// Gateway opcodes
/// Opcode received when the server dispatches a [crate::types::WebSocketEvent]
const GATEWAY_DISPATCH: u8 = Opcode::Dispatch as u8;
/// Opcode sent when sending a heartbeat
const GATEWAY_HEARTBEAT: u8 = Opcode::Heartbeat as u8;
/// Opcode sent to initiate a session
///
/// See [types::GatewayIdentifyPayload]
const GATEWAY_IDENTIFY: u8 = Opcode::Identify as u8;
/// Opcode sent to update our presence
///
/// See [types::GatewayUpdatePresence]
const GATEWAY_UPDATE_PRESENCE: u8 = Opcode::PresenceUpdate as u8;
/// Opcode sent to update our state in vc
///
/// Like muting, deafening, leaving, joining..
///
/// See [types::UpdateVoiceState]
const GATEWAY_UPDATE_VOICE_STATE: u8 = Opcode::VoiceStateUpdate as u8;
/// Opcode sent to resume a session
///
/// See [types::GatewayResume]
const GATEWAY_RESUME: u8 = Opcode::Resume as u8;
/// Opcode received to tell the client to reconnect
const GATEWAY_RECONNECT: u8 = Opcode::Reconnect as u8;
/// Opcode sent to request guild member data
///
/// See [types::GatewayRequestGuildMembers]
const GATEWAY_REQUEST_GUILD_MEMBERS: u8 = Opcode::RequestGuildMembers as u8;
/// Opcode received to tell the client their token / session is invalid
const GATEWAY_INVALID_SESSION: u8 = Opcode::InvalidSession as u8;
/// Opcode received when initially connecting to the gateway, starts our heartbeat
///
/// See [types::HelloData]
const GATEWAY_HELLO: u8 = Opcode::Hello as u8;
/// Opcode received to acknowledge a heartbeat
const GATEWAY_HEARTBEAT_ACK: u8 = Opcode::HeartbeatAck as u8;
/// Opcode sent to get the voice state of users in a given DM/group channel
///
/// See [types::CallSync]
const GATEWAY_CALL_SYNC: u8 = Opcode::CallConnect as u8;
/// Opcode sent to get data for a server (Lazy Loading request)
///
/// Sent by the official client when switching to a server
///
/// See [types::LazyRequest]
const GATEWAY_LAZY_REQUEST: u8 = Opcode::GuildSync as u8;

pub type ObservableObject = dyn Send + Sync + Any;

/// Note: this is a reexport of [pubserve::Subscriber],
/// exported not to break the public api and make development easier
pub use pubserve::Subscriber as Observer;

/// An entity type which is supposed to be updateable via the Gateway. This is implemented for all such types chorus supports, implementing it for your own types is likely a mistake.
pub trait Updateable: 'static + Send + Sync {
    fn id(&self) -> Snowflake;
}
