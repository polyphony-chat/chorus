// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{PartialEmoji, Snowflake};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-object>
pub struct Activity {
    /// The ID of the activity
    ///
    /// Only unique across a single user's activities.
    ///
    /// This field is only received and cannot be sent.
    pub id: String,

    /// The name of the activity (2-128 characters)
    ///
    /// The name of a [ActivityType::Custom] activity should always be "Custom Status"
    pub name: String,

    #[serde(rename = "type")]
    pub activity_type: ActivityType,

    /// The stream URL (max 512 characters)
    ///
    /// Must start with http:// or https://
    #[serde(default)]
    pub url: Option<String>,

    /// Unix timestamp (sent in milliseconds) of when the activity was added to the user's session
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,

    /// The ID of the session associated with the activity
    #[serde(default)]
    pub session_id: Option<String>,

    /// The platform the activity is being played on.
    ///
    /// This field is not commonly used for traditional presences (i.e. presences sent by regular clients over the Gateway)
    /// and is instead used to differentiate between various headless and embedded activities.
    #[serde(default)]
    pub platform: Option<ActivityPlatformType>,

    /// The platforms the activity is supported on
    #[serde(default)]
    pub supported_platforms: Option<Vec<ActivityPlatformType>>,

    /// Unix timestamps (sent in milliseconds) for start and/or end of the game
    #[serde(default)]
    pub timestamps: Option<ActivityTimestamps>,

    /// The ID of the application representing the game the user is playing
    #[serde(default)]
    pub application_id: Option<Snowflake>,

    /// What the user is currently doing (max 128 characters)
    #[serde(default)]
    pub details: Option<String>,

    #[serde(default)]
    pub state: Option<String>,

    #[serde(default)]
    pub sync_id: Option<String>,

    #[serde(default)]
    pub flags: Option<ActivityFlags>,

    /// Custom buttons shown in rich presence (max 2)
    #[serde(default)]
    pub buttons: Option<Vec<String>>,

    /// The emoji used for a custom status
    #[serde(default)]
    pub emoji: Option<PartialEmoji>,

    /// Information about the current party of the user
    #[serde(default)]
    pub party: Option<ActivityParty>,

    /// Images for the presence and their hover texts
    #[serde(default)]
    pub assets: Option<ActivityAssets>,

    /// Secrets for rich presence joining.
    ///
    /// This field is send-only, but can be retrieved with its own route.
    #[serde(default)]
    pub secrets: Option<ActivitySecrets>,

    /// Additional metadata for the activity
    ///
    /// # Notes
    /// Activity metadata can consist of arbitrary data, and is not sanitized by the API.
    ///
    /// Treat data within this object carefully.
    ///
    /// The official clients follow a convention: <https://docs.discord.sex/resources/presence#activity-metadata-structure>
    ///
    /// This field is send-only, but can be retrieved with its own route.
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    // Note: is this a spacebar only thing / an old discord thing?
    // This is now in activity flags
    //pub instance: Option<bool>,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-type>
pub enum ActivityType {
    #[default]
    /// "Playing {name}"
    Playing = 0,
    /// "Streaming {name}"
    Streaming = 1,
    /// "Listening to {name}"
    Listening = 2,
    /// "Watching {name}"
    Watching = 3,
    /// "{emoji} {state}"
    Custom = 4,
    /// "Competing in {name}"
    Competing = 5,
    /// Deprecated; "{state} or {emoji} {details}"
    Hang = 6,
}

#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
/// Platform an [Activity] is being played on
///
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-platform-type>
pub enum ActivityPlatformType {
    #[default]
    Desktop,
    Xbox,
    Samsung,
    IOS,
    Android,
    /// Embedded session
    Embedded,
    /// PlayStation 4 intergration
    Ps4,
    /// PlayStation 5 integration
    Ps5,
}

/// Unix timestamps (sent in milliseconds) for start and/or end of the game
///
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-timestamps-structure>
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub struct ActivityTimestamps {
    /// Unix time (sent in milliseconds) of when the activity starts
    #[serde(default)]
    #[serde(with = "ts_milliseconds_option")]
    pub start: Option<DateTime<Utc>>,

    /// Unix time (sent in milliseconds) of when the activity ends
    #[serde(default)]
    #[serde(with = "ts_milliseconds_option")]
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-party-structure>
pub struct ActivityParty {
    /// The ID of the party (max 128 characters)
    #[serde(default)]
    pub id: Option<String>,

    /// The party's current and maximum size (current_size, max_size)
    #[serde(default)]
    pub size: ActivityPartySize,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg(feature = "sqlx-pg-uint")]
pub struct ActivityPartySize(Vec<(sqlx_pg_uint::PgU16, sqlx_pg_uint::PgU16)>);

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg(not(feature = "sqlx-pg-uint"))]
pub struct ActivityPartySize(Vec<(u16, u16)>);

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for ActivityPartySize {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for ActivityPartySize {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let data = self
            .0
            .iter()
            .map(|x| format!("{},{}", x.0, x.1))
            .collect::<Vec<String>>()
            .join("|");
        <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&data, buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Decode<'q, sqlx::Postgres> for ActivityPartySize {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'q>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <String as sqlx::Decode<'q, sqlx::Postgres>>::decode(value).map(|x| {
            ActivityPartySize(
                x.split('|')
                    .map(|x| {
                        let mut split = x.split(',');
                        (
                            split
                                .next()
                                .unwrap()
                                .parse::<sqlx_pg_uint::PgU16>()
                                .unwrap(),
                            split
                                .next()
                                .unwrap()
                                .parse::<sqlx_pg_uint::PgU16>()
                                .unwrap(),
                        )
                    })
                    .collect(),
            )
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-assets-structure>
pub struct ActivityAssets {
    /// The large activity asset image (max 313 characters)
    #[serde(default)]
    pub large_image: Option<String>,

    /// Text displayed when hovering over the large image of the activity (max 128 characters)
    #[serde(default)]
    pub large_text: Option<String>,

    /// The small activity asset image (max 313 characters)
    #[serde(default)]
    pub small_image: Option<String>,

    /// Text displayed when hovering over the small image of the activity (max 128 characters)
    #[serde(default)]
    pub small_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.sex/resources/presence#activity-secrets-structure>
pub struct ActivitySecrets {
    /// The secret for joining a party (max 128 characters)
    #[serde(default)]
    pub join: Option<String>,

    /// Deprecated; the secret for spectating a game (max 128 characters)
    #[serde(default)]
    pub spectate: Option<String>,

    /// Deprecated; the secret for a specific instanced match (max 128 characters)
    #[serde(default)]
    #[serde(rename = "match")]
    pub match_string: Option<String>,
}

// Note: this is now documented as just a string? is this a spacebar thing?
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct ActivityButton {
    pub label: String,
    pub url: String,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.sex/resources/presence#activity-flags>
    pub struct ActivityFlags: u64 {
        /// The activity is an instanced game session; a match that will end
        const INSTANCE = 1 << 0;
        /// The activity can be joined by other users
        const JOIN = 1 << 1;
        /// Deprecated; The activity can be spectated by other users
        const SPECTATE = 1 << 2;
        /// Deprecated - Activities no longer need to be explicitly flagged as join requestable
        const JOIN_REQUEST = 1 << 3;
        /// The activity can be synced
        const SYNC = 1 << 4;
        /// The activity can be played
        const PLAY = 1 << 5;
        /// The activity's party can be joined by friends
        const PARTY_PRIVACY_FRIENDS = 1 << 6;
        /// The activity's party can be joined by users in the same voice channel
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;
        /// Thie activity is embedded within the Discord client
        const EMBEDDED = 1 << 8;
    }
}
