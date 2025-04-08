// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::events::WebSocketEvent;
use crate::types::{entities::Channel, JsonField, Snowflake, SourceUrlField};
use chorus_macros::{JsonField, SourceUrlField};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "client")]
use super::UpdateMessage;

#[cfg(feature = "client")]
use crate::types::Shared;

#[cfg(feature = "client")]
use crate::types::IntoShared;

#[cfg(feature = "client")]
use crate::types::Guild;

#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    WebSocketEvent,
    Copy,
    PartialEq,
    Clone,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-pins-update>
pub struct ChannelPinsUpdate {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Snowflake,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}

#[derive(
    Debug, Default, Deserialize, Serialize, Clone, JsonField, SourceUrlField, WebSocketEvent,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-create>
pub struct ChannelCreate {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Guild> for ChannelCreate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        self.channel.guild_id
    }

    fn update(&mut self, write: &mut Guild) {
        let update = self.channel.clone().into_shared();
        write.channels.push(update);
    }
}

#[derive(
    Debug, Default, Deserialize, Serialize, Clone, JsonField, SourceUrlField, WebSocketEvent,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-update>
pub struct ChannelUpdate {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Channel> for ChannelUpdate {
    fn update(&mut self, write: &mut Channel) {
        *write = self.channel.clone();
    }

    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.channel.id)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent)]
/// Officially undocumented.
/// Sends updates to client about a new message with its id
/// {"channel_unread_updates": [{"id": "816412869766938648", "last_message_id": "1085892012085104680"}}
pub struct ChannelUnreadUpdate {
    pub channel_unread_updates: Vec<ChannelUnreadUpdateObject>,
    pub guild_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Contains very few fields from [Channel]
/// See also [ChannelUnreadUpdate]
pub struct ChannelUnreadUpdateObject {
    pub id: Snowflake,
    pub last_message_id: Snowflake,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}

#[derive(
    Debug, Default, Deserialize, Serialize, Clone, JsonField, SourceUrlField, WebSocketEvent,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#channel-delete>
pub struct ChannelDelete {
    #[serde(flatten)]
    pub channel: Channel,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Guild> for ChannelDelete {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        self.channel.guild_id
    }

    fn update(&mut self, write: &mut Guild) {
        if self.id().is_none() {
            return;
        }
        if write.channels.is_empty() {
            return;
        }
        for (iteration, item) in (0_u32..).zip(write.channels.iter()) {
            if item.read().unwrap().id == self.id().unwrap() {
                write.channels.remove(iteration as usize);
                return;
            }
        }
    }
}
