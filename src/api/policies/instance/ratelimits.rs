use std::hash::Hash;

use crate::types::Snowflake;

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

#[derive(Debug, Clone)]
pub struct Limit {
    pub bucket: LimitType,
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
    pub window: u64,
}
