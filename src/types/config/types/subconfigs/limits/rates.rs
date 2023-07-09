use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    api::LimitType,
    types::config::types::subconfigs::limits::ratelimits::{
        route::RouteRateLimit, RateLimitOptions,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RateLimits {
    pub enabled: bool,
    pub ip: RateLimitOptions,
    pub global: RateLimitOptions,
    pub error: RateLimitOptions,
    pub routes: RouteRateLimit,
}

impl Default for RateLimits {
    fn default() -> Self {
        Self {
            enabled: false,
            ip: RateLimitOptions {
                bot: None,
                count: 500,
                window: 5,
                only_ip: false,
            },
            global: RateLimitOptions {
                bot: None,
                count: 250,
                window: 5,
                only_ip: false,
            },
            error: RateLimitOptions {
                bot: None,
                count: 10,
                window: 5,
                only_ip: false,
            },
            routes: RouteRateLimit::default(),
        }
    }
}

impl RateLimits {
    pub fn to_hash_map(&self) -> HashMap<LimitType, RateLimitOptions> {
        let mut map = HashMap::new();
        map.insert(LimitType::AuthLogin, self.routes.auth.login.clone());
        map.insert(LimitType::AuthRegister, self.routes.auth.register.clone());
        map.insert(LimitType::ChannelBaseline, self.routes.channel.clone());
        map.insert(LimitType::Error, self.error.clone());
        map.insert(LimitType::Global, self.global.clone());
        map.insert(LimitType::Ip, self.ip.clone());
        map.insert(LimitType::WebhookBaseline, self.routes.webhook.clone());
        map.insert(LimitType::GuildBaseline, self.routes.guild.clone());
        map
    }
}
