pub mod limits {
    use std::hash::{Hash, Hasher};

    use strum::EnumIter;

    #[derive(Clone, Copy, Eq, PartialEq, Debug, Default, EnumIter)]
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

    // A fast way to hash this basic Enum
    impl Hash for LimitType {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (*self as u32).hash(state);
        }
    }

    #[derive(Debug, Clone)]
    pub struct Limit {
        pub bucket: LimitType,
        pub limit: u64,
        pub remaining: u64,
        pub reset: u64,
    }
}
