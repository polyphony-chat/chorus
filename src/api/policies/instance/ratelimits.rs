pub mod limits {
    #[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Default)]
    pub enum LimitType {
        AuthRegister,
        AuthLogin,
        #[default]
        Global,
        Ip,
        Channel,
        Error,
        Guild,
        Webhook,
    }

    pub struct Ratelimits {
        pub auth_register: Limit,
        pub auth_login: Limit,
        pub global: Limit,
        pub ip: Limit,
        pub channel: Limit,
        pub error: Limit,
        pub guild: Limit,
        pub webhook: Limit,
    }

    pub struct Limit {
        pub bucket: LimitType,
        pub limit: u64,
        pub remaining: u64,
        pub reset: u64,
    }
}
