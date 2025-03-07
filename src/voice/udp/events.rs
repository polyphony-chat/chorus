// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use discortp::{rtcp::Rtcp, rtp::Rtp};
use pubserve::Publisher;

use crate::types::WebSocketEvent;

impl WebSocketEvent for Rtp {}
impl WebSocketEvent for Rtcp {}

#[derive(Debug)]
pub struct VoiceUDPEvents {
    pub rtp: Publisher<Rtp>,
    pub rtcp: Publisher<Rtcp>,
}

impl Default for VoiceUDPEvents {
    fn default() -> Self {
        Self {
            rtp: Publisher::new(),
            rtcp: Publisher::new(),
        }
    }
}
