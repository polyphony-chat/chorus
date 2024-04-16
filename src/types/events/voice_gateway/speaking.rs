// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::types::{Snowflake, WebSocketEvent};

/// Event that tells the server we are speaking;
///
/// Essentially, what allows us to send UDP data and lights up the green circle around your avatar.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#speaking-structure>
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Speaking {
    /// Data about the audio we're transmitting.
    ///
    /// See [SpeakingBitflags]
    pub speaking: u8,
    pub ssrc: u32,
    /// The user id of the speaking user, only sent by the server
    #[serde(skip_serializing)]
    pub user_id: Option<Snowflake>,
    /// Delay in milliseconds, not sent by the server
    #[serde(default)]
    pub delay: u64,
}

impl WebSocketEvent for Speaking {}

bitflags! {
    /// Bitflags of speaking types;
    ///
    /// See <https://discord.com/developers/docs/topics/voice-connections#speaking>
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
    pub struct SpeakingBitflags: u8 {
        /// Whether we'll be transmitting normal voice audio
        const MICROPHONE = 1 << 0;
        /// Whether we'll be transmitting context audio for video, no speaking indicator
        const SOUNDSHARE = 1 << 1;
        /// Whether we are a priority speaker, lowering audio of other speakers
        const PRIORITY = 1 << 2;
    }
}

impl Default for SpeakingBitflags {
    /// Returns the default value for these flags, assuming normal microphone audio and not being a priority speaker
    fn default() -> Self {
        Self::MICROPHONE
    }
}
