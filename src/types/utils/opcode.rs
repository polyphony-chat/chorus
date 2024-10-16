#![allow(deprecated)] // Required to suppress warnings about deprecated opcodes

use serde::{Deserialize, Serialize};

use crate::errors::ChorusError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
#[non_exhaustive]
#[repr(u8)]
/// Gateway opcodes used in the Spacebar Gateway Protocol.
pub enum Opcode {
    /// An event was dispatched.
    Dispatch = 0,
    /// Keep the WebSocket connection alive.
    Heartbeat = 1,
    /// Start a new session during the initial handshake.
    Identify = 2,
    /// Update the client's presence.
    PresenceUpdate = 3,
    /// Join/leave or move between voice channels and calls.
    VoiceStateUpdate = 4,
    /// Ping the Discord voice servers.
    VoiceServerPing = 5,
    /// Resume a previous session that was disconnected.
    Resume = 6,
    /// You should attempt to reconnect and resume immediately.
    Reconnect = 7,
    /// Request information about guild members.
    RequestGuildMembers = 8,
    /// The session has been invalidated. You should reconnect and identify/resume accordingly.
    InvalidSession = 9,
    /// Sent immediately after connecting, contains the heartbeat_interval to use.
    Hello = 10,
    /// Acknowledge a received heartbeat.
    HeartbeatAck = 11,
    /// Request all members and presences for guilds.
    #[deprecated]
    GuildSync = 12,
    /// Request a private channel's pre-existing call data.
    CallConnect = 13,
    /// Update subscriptions for a guild.
    GuildSubscriptions = 14,
    /// Join a lobby.
    LobbyConnect = 15,
    /// Leave a lobby.
    LobbyDisconnect = 16,
    /// Update the client's voice state in a lobby.
    LobbyVoiceStates = 17,
    /// Create a stream for the client.
    StreamCreate = 18,
    /// End a client stream.
    StreamDelete = 19,
    /// Watch a user's stream.
    StreamWatch = 20,
    /// Ping a user stream's voice server.
    StreamPing = 21,
    /// Pause/resume a client stream.
    StreamSetPaused = 22,
    /// Update subscriptions for an LFG lobby.
    #[deprecated]
    LfgSubscriptions = 23,
    /// Request guild application commands.
    #[deprecated]
    RequestGuildApplicationCommands = 24,
    /// Launch an embedded activity in a voice channel or call.
    EmbeddedActivityCreate = 25,
    /// Stop an embedded activity.
    EmbeddedActivityDelete = 26,
    /// Update an embedded activity.
    EmbeddedActivityUpdate = 27,
    /// Request forum channel unread counts.
    RequestForumUnreads = 28,
    /// Send a remote command to an embedded (Xbox, PlayStation) voice session.
    RemoteCommand = 29,
    /// Request deleted entity IDs not matching a given hash for a guild.
    RequestDeletedEntityIDs = 30,
    /// Request soundboard sounds for guilds.
    RequestSoundboardSounds = 31,
    /// Create a voice speed test.
    SpeedTestCreate = 32,
    /// Delete a voice speed test.
    SpeedTestDelete = 33,
    /// Request last messages for a guild's channels.
    RequestLastMessages = 34,
    /// Request information about recently-joined guild members.
    SearchRecentMembers = 35,
    /// Request voice channel statuses for a guild.
    RequestChannelStatuses = 36,
}

impl TryFrom<u8> for Opcode {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dispatch),
            1 => Ok(Self::Heartbeat),
            2 => Ok(Self::Identify),
            3 => Ok(Self::PresenceUpdate),
            4 => Ok(Self::VoiceStateUpdate),
            5 => Ok(Self::VoiceServerPing),
            6 => Ok(Self::Resume),
            7 => Ok(Self::Reconnect),
            8 => Ok(Self::RequestGuildMembers),
            9 => Ok(Self::InvalidSession),
            10 => Ok(Self::Hello),
            11 => Ok(Self::HeartbeatAck),
            12 => Ok(Self::GuildSync),
            13 => Ok(Self::CallConnect),
            14 => Ok(Self::GuildSubscriptions),
            15 => Ok(Self::LobbyConnect),
            16 => Ok(Self::LobbyDisconnect),
            17 => Ok(Self::LobbyVoiceStates),
            18 => Ok(Self::StreamCreate),
            19 => Ok(Self::StreamDelete),
            20 => Ok(Self::StreamWatch),
            21 => Ok(Self::StreamPing),
            22 => Ok(Self::StreamSetPaused),
            23 => Ok(Self::LfgSubscriptions),
            24 => Ok(Self::RequestGuildApplicationCommands),
            25 => Ok(Self::EmbeddedActivityCreate),
            26 => Ok(Self::EmbeddedActivityDelete),
            27 => Ok(Self::EmbeddedActivityUpdate),
            28 => Ok(Self::RequestForumUnreads),
            29 => Ok(Self::RemoteCommand),
            30 => Ok(Self::RequestDeletedEntityIDs),
            31 => Ok(Self::RequestSoundboardSounds),
            32 => Ok(Self::SpeedTestCreate),
            33 => Ok(Self::SpeedTestDelete),
            34 => Ok(Self::RequestLastMessages),
            35 => Ok(Self::SearchRecentMembers),
            36 => Ok(Self::RequestChannelStatuses),
            e => Err(ChorusError::InvalidArguments {
                error: format!("Provided value {e} is not a valid opcode"),
            }),
        }
    }
}
