pub mod limits {
    use std::hash::{Hash, Hasher};

    #[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash)]
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

    #[derive(Debug, Clone)]
    pub struct Limit {
        pub bucket: LimitType,
        pub limit: u64,
        pub remaining: u64,
        pub reset: u64,
    }
}
