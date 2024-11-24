// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::{Snowflake, VoiceState, WebSocketEvent};
use chorus_macros::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default, Clone, WebSocketEvent)]
/// Officially Undocumented;
/// Is sent to a client by the server to signify a new call being created;
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#call-create>
pub struct CallCreate {
    /// Id of the private channel this call is in
    pub channel_id: Snowflake,
    /// Id of the messsage which created the call
    pub message_id: Snowflake,

    /// The IDs of users that are being rung to join the call
    pub ringing: Vec<Snowflake>,

    // milan
    pub region: String,

    /// The voice states of the users already in the call
    pub voice_states: Vec<VoiceState>,
    // What is this?
    //pub embedded_activities: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, WebSocketEvent)]
/// Updates the client when metadata about a call changes.
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#call-update>
pub struct CallUpdate {
    /// Id of the private channel this call is in
    pub channel_id: Snowflake,
    /// Id of the messsage which created the call
    pub message_id: Snowflake,

    /// The IDs of users that are being rung to join the call
    pub ringing: Vec<Snowflake>,

    // milan
    pub region: String,
}

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    PartialEq,
    Eq,
    WebSocketEvent,
    Copy,
    PartialOrd,
    Ord,
)]
/// Sent when a call is deleted, or becomes unavailable due to an outage.
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#call-delete>
pub struct CallDelete {
    pub channel_id: Snowflake,
	 /// Whether the call is unavailable due to an outage
	 pub unavailable: Option<bool>,
}

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    PartialEq,
    Eq,
    WebSocketEvent,
    Copy,
    PartialOrd,
    Ord,
)]
/// Used to request a private channel's pre-existing call data,
/// created before the connection was established.
///
/// Fires a [CallCreate] event if a call is found.
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#request-call-connect>;
pub struct CallSync {
    pub channel_id: Snowflake,
}
