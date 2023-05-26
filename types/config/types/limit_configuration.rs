use serde::{Deserialize, Serialize};

use crate::config::types::subconfigs::limits::{
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
