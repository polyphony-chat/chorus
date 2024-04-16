// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
/// Received from the voice gateway server to describe the backend version.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#voice-backend-version>
pub struct VoiceBackendVersion {
    /// The voice backend's version
    #[serde(rename = "voice")]
    pub voice_version: String,
    /// The WebRTC worker's version
    #[serde(rename = "rtc_worker")]
    pub rtc_worker_version: String,
}

impl WebSocketEvent for VoiceBackendVersion {}
