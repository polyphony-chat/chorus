use crate::types::{Snowflake, WebSocketEvent};
use serde::{Serialize, Deserialize};

/// Defines an event which provides ssrcs for voice and video for a user id.
///
/// This event is sent via opcode 12.
///
/// Examples of the event:
///
/// When receiving:
/// ```
/// {"op":12,"d":{"video_ssrc":0,"user_id":"463640391196082177","streams":[{"ssrc":26595,"rtx_ssrc":26596,"rid":"100","quality":100,"max_resolution":{"width":1280,"type":"fixed","height":720},"max_framerate":30,"active":false}],"audio_ssrc":26597}}{"op":12,"d":{"video_ssrc":0,"user_id":"463640391196082177","streams":[{"ssrc":26595,"rtx_ssrc":26596,"rid":"100","quality":100,"max_resolution":{"width":1280,"type":"fixed","height":720},"max_framerate":30,"active":false}],"audio_ssrc":26597}}
/// ```
///
/// When sending:
/// ```
/// {"op":12,"d":{"audio_ssrc":2307250864,"video_ssrc":0,"rtx_ssrc":0,"streams":[{"type":"video","rid":"100","ssrc":26595,"active":false,"quality":100,"rtx_ssrc":26596,"max_bitrate":2500000,"max_framerate":30,"max_resolution":{"type":"fixed","width":1280,"height":720}}]}}
/// ```
#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SsrcDefinition {
    /// The ssrc used for video communications.
    ///
    /// Is always sent and received, though is 0 if describing only the audio ssrc.
    #[serde(default)]
    pub video_ssrc: usize,
    /// The ssrc used for audio communications.
    ///
    /// Is always sent and received, though is 0 if describing only the video ssrc.
    #[serde(default)]
    pub audio_ssrc: usize,
    /// The user id these ssrcs apply to.
    ///
    /// Is never sent by the user and is filled in by the server
    #[serde(skip_serializing)]
    pub user_id: Option<Snowflake>,
    // TODO: Add video streams
}

impl WebSocketEvent for SsrcDefinition {}
