// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Defines the UDP component of voice communications, sending and receiving raw rtp data.

/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#voice-packet-structure>
/// This always adds up to 12 bytes
const RTP_HEADER_SIZE: u8 = 12;

pub mod backends;
pub mod events;
pub mod handle;
pub mod handler;

pub use backends::*;
pub use handle::*;
pub use handler::*;
