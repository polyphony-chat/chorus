use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    api::limits::LimitType,
    types::config::types::subconfigs::limits::{
        channel::ChannelLimits, global::GlobalRateLimits, guild::GuildLimits,
        message::MessageLimits, rates::RateLimits, user::UserLimits,
    },
};

use super::subconfigs::limits::ratelimits::RateLimitOptions;

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

impl RateLimits {
    pub fn to_hash_map(&self) -> HashMap<LimitType, RateLimitOptions> {
        let mut map = HashMap::new();
        map.insert(LimitType::AuthLogin, self.routes.auth.login);
        map.insert(LimitType::AuthRegister, self.routes.auth.register);
        map.insert(LimitType::Channel, self.routes.channel);
        map.insert(LimitType::Error, self.error);
        map.insert(LimitType::Global, self.global);
        map.insert(LimitType::Ip, self.ip);
        map.insert(LimitType::Webhook, self.routes.webhook);
        map.insert(LimitType::Guild, self.routes.guild);
        map
    }
}
