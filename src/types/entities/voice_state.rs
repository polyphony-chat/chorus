use std::sync::{Arc, RwLock};

#[cfg(feature = "client")]
use chorus_macros::Composite;

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::types::{
    entities::{Guild, GuildMember},
    utils::Snowflake,
};

/// The VoiceState struct. Note, that Discord does not have an `id` field for this, whereas Spacebar
/// does.
///
/// See <https://docs.spacebar.chat/routes/#cmp--schemas-voicestate>
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "client", derive(Composite))]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    pub guild: Option<Guild>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub member: Option<Arc<RwLock<GuildMember>>>,
    pub session_id: String,
    pub token: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
    pub id: Option<Snowflake>, // Only exists on Spacebar
}

impl Updateable for VoiceState {
    fn id(&self) -> Snowflake {
        if let Some(id) = self.id {
            id // ID exists: Only the case for Spacebar Server impls
        } else {
            self.user_id // ID doesn't exist: Discord does not have the ID field - ID is void
        }
    }
}
