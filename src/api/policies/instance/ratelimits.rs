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

    impl Ratelimits {
        pub fn is_exhausted(&self, limit_type: &LimitType) -> bool {
            let all_limits = [
                self.auth_login,
                self.auth_register,
                self.global,
                self.ip,
                self.channel,
                self.error,
                self.guild,
                self.webhook,
            ];
            for limit in all_limits {
                if &limit.bucket == limit_type && limit.remaining == 0 {
                    true;
                }
                if (limit.bucket == LimitType::Global || limit.bucket == LimitType::Ip)
                    && limit.remaining == 0
                {
                    true;
                }
            }
            false
        }

        pub fn get_mut(&self, limit_type: LimitType) -> &mut Limit {
            match limit_type {
                LimitType::AuthRegister => &mut self.auth_register,
                LimitType::AuthLogin => &mut self.auth_login,
                LimitType::Global => &mut self.global,
                LimitType::Channel => &mut self.channel,
                LimitType::Error => &mut self.error,
                LimitType::Guild => &mut self.guild,
                LimitType::Ip => &mut self.ip,
                LimitType::Webhook => &mut self.webhook,
            }
        }
    }

    pub struct Limit {
        pub bucket: LimitType,
        pub limit: u64,
        pub remaining: u64,
        pub reset: u64,
    }
}
