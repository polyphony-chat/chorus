use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::types::Snowflake;

/// The different types of ratelimits that can be applied to a request. Includes "Baseline"-variants
/// for when the Snowflake is not yet known.
/// See <https://discord.com/developers/docs/topics/rate-limits#rate-limits> for more information.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
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
/// See <https://discord.com/developers/docs/topics/rate-limits#rate-limits> for more information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Limit {
    pub bucket: LimitType,
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
    pub window: u64,
}
