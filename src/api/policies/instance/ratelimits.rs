use std::hash::Hash;

use crate::types::Snowflake;

/// The different types of ratelimits that can be applied to a request. Includes "Baseline"-variants
/// for when the Snowflake is not yet known.
/// See <https://discord.com/developers/docs/topics/rate-limits#rate-limits> for more information.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash)]
pub enum LimitType {
    AuthRegister,
    AuthLogin,
    #[default]
    Global,
    Ip,
    Channel(Snowflake),
    ChannelBaseline,
    Error,
    Guild(Snowflake),
    GuildBaseline,
    Webhook(Snowflake),
    WebhookBaseline,
}

/// A struct that represents the current ratelimits, either instance-wide or user-wide.
/// Unlike [`RateLimits`], this struct shows the current ratelimits, not the rate limit
/// configuration for the instance.
/// See <https://discord.com/developers/docs/topics/rate-limits#rate-limits> for more information.
#[derive(Debug, Clone)]
pub struct Limit {
    pub bucket: LimitType,
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
    pub window: u64,
}
