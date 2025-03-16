// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{errors::ChorusError, types::Snowflake};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord.com/developers/docs/resources/stage-instance>
pub struct StageInstance {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    /// 1 - 120 characters
    pub topic: String,
    pub privacy_level: StageInstancePrivacyLevel,
    /// deprecated, apparently
    pub discoverable_disabled: Option<bool>,
    pub guild_scheduled_event_id: Option<Snowflake>,
}

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16), derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// See <https://discord.com/developers/docs/resources/stage-instance#stage-instance-object-privacy-level>
pub enum StageInstancePrivacyLevel {
    /// deprecated, apparently
    Public = 1,
    #[default]
    GuildOnly = 2,
}

impl TryFrom<u8> for StageInstancePrivacyLevel {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Public),
            2 => Ok(Self::GuildOnly),
            _ => Err(ChorusError::InvalidArguments {
                error: "Value is not a valid StageInstancePrivacyLevel".to_string(),
            }),
        }
    }
}
