// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Copy)]
/// Contains info on how often the client should send heartbeats to the server;
///
/// Differs from the normal hello data in that discord sends heartbeat interval as a float.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#heartbeating>
pub struct VoiceHelloData {
    /// The voice gateway version.
    ///
    /// Note: no idea why this is sent, we already specify the version when establishing a connection.
    #[serde(rename = "v")]
    pub version: u8,
    /// How often a client should send heartbeats, in milliseconds
    pub heartbeat_interval: f64,
}

impl WebSocketEvent for VoiceHelloData {}
