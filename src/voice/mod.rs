// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Module for all voice functionality within chorus.

mod crypto;
#[cfg(feature = "voice_gateway")]
pub mod gateway;
#[cfg(feature = "voice_udp")]
pub mod udp;
#[cfg(feature = "voice_udp")]
pub mod voice_data;

// Pub use this so users can interact with packet types if they want
#[cfg(feature = "voice_udp")]
pub use discortp;
