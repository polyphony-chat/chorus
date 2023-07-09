pub mod limits {
    use std::hash::Hash;

    use crate::types::{types::subconfigs::limits::ratelimits::RateLimitOptions, Snowflake};

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

    impl Limit {
        pub(crate) fn from_rate_limit_options(
            limit_type: LimitType,
            rate_limit_options: &RateLimitOptions,
        ) -> Limit {
            let time: u64 = chrono::Utc::now().timestamp() as u64;
            Limit {
                bucket: limit_type,
                limit: rate_limit_options.count,
                remaining: rate_limit_options.count,
                reset: rate_limit_options.window + time,
                window: rate_limit_options.window,
            }
        }
    }
}
