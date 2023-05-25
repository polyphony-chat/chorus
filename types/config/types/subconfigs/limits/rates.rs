use serde::{Deserialize, Serialize};

use crate::config::types::subconfigs::limits::ratelimits::{
    route::RouteRateLimit, RateLimitOptions,
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
