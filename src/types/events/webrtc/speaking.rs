use bitflags::bitflags;
use serde::{Deserialize, Serialize};

/// Event that tells the server we are speaking;
///
/// Essentially, what allows us to send udp data and lights up the green circle around your avatar.
///
/// See <https://discord.com/developers/docs/topics/voice-connections#speaking-example-speaking-payload>
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Speaking {
    /// Data about the audio we're transmitting, its type
    pub speaking: SpeakingBitflags,
    /// Assuming delay in milliseconds for the audio, should be 0 most of the time
    pub delay: u64,
    pub ssrc: i32,
}

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
