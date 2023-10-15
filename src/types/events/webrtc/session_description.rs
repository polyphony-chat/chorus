use super::{AudioCodec, VideoCodec, VoiceEncryptionMode};
use crate::types::WebSocketEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// Event that describes our encryption mode and secret key for encryption
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#session-description-structure>
pub struct SessionDescription {
    pub audio_codec: AudioCodec,
    pub video_codec: VideoCodec,
    pub media_session_id: String,
    /// The encryption mode to use
    #[serde(rename = "mode")]
    pub encryption_mode: VoiceEncryptionMode,
    /// The secret key we'll use for encryption
    pub secret_key: [u8; 32],
    /// The keyframe interval in milliseconds
    pub keyframe_interval: Option<u64>,
}

impl WebSocketEvent for SessionDescription {}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
/// Event that might be sent to update session parameters
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#session-update-structure>
pub struct SessionUpdate {
    #[serde(rename = "audio_codec")]
    pub new_audio_codec: Option<AudioCodec>,

    #[serde(rename = "video_codec")]
    pub new_video_codec: Option<VideoCodec>,

    #[serde(rename = "media_session_id")]
    pub new_media_session_id: Option<String>,
}

impl WebSocketEvent for SessionUpdate {}
