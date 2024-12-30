// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::{events::WebSocketEvent, OneOrMoreSnowflakes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, Serialize, WebSocketEvent, Clone)]
/// Used to request members for a guild or a list of guilds.
///
/// Fires multiple [crate::types::events::GuildMembersChunk] events (each with up to 1000 members)
/// until all members that match the request have been sent.
///
/// # Notes
/// One of `query` or `user_ids` is required.
///
/// If `query` is set, `limit` is required (if requesting all members, set `limit` to 0)
///
/// # Reference
/// See <https://docs.discord.sex/topics/gateway-events#request-guild-members>
pub struct GatewayRequestGuildMembers {
    /// Id(s) of the guild(s) to get members for
    pub guild_id: OneOrMoreSnowflakes,

    /// The user id(s) to request (0 - 100)
    pub user_ids: Option<OneOrMoreSnowflakes>,

    /// String that the username / nickname starts with, or an empty string for all members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Maximum number of members to send matching the query (0 - 100)
    ///
    /// Must be 0 with an empty query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    /// Whether to return the [Presence](crate::types::events::PresenceUpdate) of the matched
    /// members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<bool>,

    /// Unique string to identify the received event for this specific request.
    ///
    /// Up to 32 bytes. If you send a longer nonce, it will be ignored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
