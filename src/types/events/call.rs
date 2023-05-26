use serde::{Deserialize, Serialize};

use crate::types::{VoiceState, WebSocketEvent};

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Is sent to a client by the server to signify a new being created
/// {"t":"CALL_CREATE","s":2,"op":0,"d":{"voice_states":[],"ringing":[],"region":"milan","message_id":"1107187514906775613","embedded_activities":[],"channel_id":"837609115475771392"}}
pub struct CallCreate {
    pub voice_states: Vec<VoiceState>,
    /// Seems like a vec of channel ids
    pub ringing: Vec<String>,
    pub region: String, // milan
    pub message_id: String,
    /// What is this?
    pub embedded_activities: Vec<serde_json::Value>,
    pub channel_id: String,
}
impl WebSocketEvent for CallCreate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Updates the status of calls
/// {"t":"CALL_UPDATE","s":5,"op":0,"d":{"ringing":["837606544539254834"],"region":"milan","message_id":"1107191540234846308","guild_id":null,"channel_id":"837609115475771392"}}
pub struct CallUpdate {
    /// Seems like a vec of channel ids
    pub ringing: Vec<String>,
    pub region: String, // milan
    pub message_id: String,
    pub guild_id: Option<String>,
    pub channel_id: String,
}
impl WebSocketEvent for CallUpdate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Deletes a ringing call
/// {"t":"CALL_DELETE","s":8,"op":0,"d":{"channel_id":"837609115475771392"}}
pub struct CallDelete {
    pub channel_id: String,
}
impl WebSocketEvent for CallDelete {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// See https://unofficial-discord-docs.vercel.app/gateway/op13
/// {"op":13,"d":{"channel_id":"837609115475771392"}}
pub struct CallSync {
    pub channel_id: String,
}
impl WebSocketEvent for CallSync {}