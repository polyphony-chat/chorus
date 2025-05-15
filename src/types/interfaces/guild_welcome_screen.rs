// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
/// # Reference
/// See <https://docs.discord.food/resources/guild#welcome-screen-structure>
pub struct WelcomeScreenObject {
    /// Whether or not the guild has the welcome screen enabled
    pub enabled: bool,

    /// The welcome message shown in the welcome screen (max 140 characters)
    pub description: Option<String>,

    /// The channels shown in the welcome screen (max 5)
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

impl From<WelcomeScreenObject> for PublicGuildWelcomeScreen {
    fn from(value: WelcomeScreenObject) -> Self {
        Self {
            description: value.description,
            welcome_channels: value.welcome_channels,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
/// A [Guild](crate::types::Guild) [WelcomeScreenObject] as returned by the api.
///
/// # Reference
/// See <https://docs.discord.food/resources/guild#welcome-screen-structure>
pub struct PublicGuildWelcomeScreen {
    /// The welcome message shown in the welcome screen (max 140 characters)
    pub description: Option<String>,

    /// The channels shown in the welcome screen (max 5)
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
/// A [Channel](crate::types::Channel) as shown in the guild's [Welcome Screen](WelcomeScreenObject)
///
/// # Reference
/// See <https://docs.discord.food/resources/guild#welcome-screen-channel-structure>
pub struct WelcomeScreenChannel {
    /// The id of the channel
    pub channel_id: Snowflake,

    /// The description shown for the channel (1-50 characters)
    pub description: String,

    /// The emoji id, if it is a custom emoji
    #[serde(default)]
    pub emoji_id: Option<Snowflake>,

    /// The emoji name if a custom emoji, the unicode character if a standard emoji or [None] if no emoji is set
    #[serde(default)]
    pub emoji_name: Option<String>,
}
