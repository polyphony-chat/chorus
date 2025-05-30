// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::config::types::subconfigs::limits::{
    channel::ChannelLimits, global::GlobalRateLimits, guild::GuildLimits, message::MessageLimits,
    rates::RateLimits, user::UserLimits,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitsConfiguration {
    pub user: UserLimits,
    pub guild: GuildLimits,
    pub message: MessageLimits,
    pub channel: ChannelLimits,
    pub rate: RateLimits,
    pub absolute_rate: GlobalRateLimits,
}
