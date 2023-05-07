pub mod limits {
    use std::{collections::HashMap};

    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    #[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Default)]
    pub enum LimitType {
        AuthRegister,
        AuthLogin,
        AbsoluteMessage,
        AbsoluteRegister,
        #[default]
        Global,
        Ip,
        Channel,
        Error,
        Guild,
        Webhook,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]
    pub struct User {
        pub maxGuilds: u64,
        pub maxUsername: u64,
        pub maxFriends: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Guild {
        pub maxRoles: u64,
        pub maxEmojis: u64,
        pub maxMembers: u64,
        pub maxChannels: u64,
        pub maxChannelsInCategory: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Message {
        pub maxCharacters: u64,
        pub maxTTSCharacters: u64,
        pub maxReactions: u64,
        pub maxAttachmentSize: u64,
        pub maxBulkDelete: u64,
        pub maxEmbedDownloadSize: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Channel {
        pub maxPins: u64,
        pub maxTopic: u64,
        pub maxWebhooks: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Rate {
        pub enabled: bool,
        pub ip: Window,
        pub global: Window,
        pub error: Window,
        pub routes: Routes,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Window {
        pub count: u64,
        pub window: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Routes {
        pub guild: Window,
        pub webhook: Window,
        pub channel: Window,
        pub auth: AuthRoutes,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct AuthRoutes {
        pub login: Window,
        pub register: Window,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct AbsoluteRate {
        pub register: AbsoluteWindow,
        pub sendMessage: AbsoluteWindow,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AbsoluteWindow {
        pub limit: u64,
        pub window: u64,
        pub enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Config {
        pub user: User,
        pub guild: Guild,
        pub message: Message,
        pub channel: Channel,
        pub rate: Rate,
        pub absoluteRate: AbsoluteRate,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct Limit {
        pub bucket: LimitType,
        pub limit: u64,
        pub remaining: u64,
        pub reset: u64,
    }

    impl std::fmt::Display for Limit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Bucket: {:?}, Limit: {}, Remaining: {}, Reset: {}",
                self.bucket, self.limit, self.remaining, self.reset
            )
        }
    }

    impl Limit {
        pub fn add_remaining(&mut self, remaining: i64) {
            if remaining < 0 {
                if (self.remaining as i64 + remaining) <= 0 {
                    self.remaining = 0;
                    return;
                }
                self.remaining -= remaining.unsigned_abs();
                return;
            }
            self.remaining += remaining.unsigned_abs();
        }
    }

    pub struct LimitsMutRef<'a> {
        pub limit_absolute_messages: &'a mut Limit,
        pub limit_absolute_register: &'a mut Limit,
        pub limit_auth_login: &'a mut Limit,
        pub limit_auth_register: &'a mut Limit,
        pub limit_ip: &'a mut Limit,
        pub limit_global: &'a mut Limit,
        pub limit_error: &'a mut Limit,
        pub limit_guild: &'a mut Limit,
        pub limit_webhook: &'a mut Limit,
        pub limit_channel: &'a mut Limit,
    }

    impl LimitsMutRef<'_> {
        pub fn combine_mut_ref<'a>(
            instance_rate_limits: &'a mut Limits,
            user_rate_limits: &'a mut Limits,
        ) -> LimitsMutRef<'a> {
            LimitsMutRef {
                limit_absolute_messages: &mut instance_rate_limits.limit_absolute_messages,
                limit_absolute_register: &mut instance_rate_limits.limit_absolute_register,
                limit_auth_login: &mut instance_rate_limits.limit_auth_login,
                limit_auth_register: &mut instance_rate_limits.limit_auth_register,
                limit_channel: &mut user_rate_limits.limit_channel,
                limit_error: &mut user_rate_limits.limit_error,
                limit_global: &mut instance_rate_limits.limit_global,
                limit_guild: &mut user_rate_limits.limit_guild,
                limit_ip: &mut instance_rate_limits.limit_ip,
                limit_webhook: &mut user_rate_limits.limit_webhook,
            }
        }

        pub fn get_limit_ref(&self, limit_type: &LimitType) -> &Limit {
            match limit_type {
                &LimitType::AbsoluteMessage => self.limit_absolute_messages,
                &LimitType::AbsoluteRegister => self.limit_absolute_register,
                &LimitType::AuthLogin => self.limit_auth_login,
                &LimitType::AuthRegister => self.limit_auth_register,
                &LimitType::Channel => self.limit_channel,
                &LimitType::Error => self.limit_error,
                &LimitType::Global => self.limit_global,
                &LimitType::Guild => self.limit_guild,
                &LimitType::Ip => self.limit_ip,
                &LimitType::Webhook => self.limit_webhook,
            }
        }

        pub fn get_limit_mut_ref(&mut self, limit_type: &LimitType) -> &mut Limit {
            match limit_type {
                &LimitType::AbsoluteMessage => self.limit_absolute_messages,
                &LimitType::AbsoluteRegister => self.limit_absolute_register,
                &LimitType::AuthLogin => self.limit_auth_login,
                &LimitType::AuthRegister => self.limit_auth_register,
                &LimitType::Channel => self.limit_channel,
                &LimitType::Error => self.limit_error,
                &LimitType::Global => self.limit_global,
                &LimitType::Guild => self.limit_guild,
                &LimitType::Ip => self.limit_ip,
                &LimitType::Webhook => self.limit_webhook,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Limits {
        pub limit_absolute_messages: Limit,
        pub limit_absolute_register: Limit,
        pub limit_auth_login: Limit,
        pub limit_auth_register: Limit,
        pub limit_ip: Limit,
        pub limit_global: Limit,
        pub limit_error: Limit,
        pub limit_guild: Limit,
        pub limit_webhook: Limit,
        pub limit_channel: Limit,
    }

    impl Limits {
        pub fn combine(instance_rate_limits: &Limits, user_rate_limits: &Limits) -> Limits {
            Limits {
                limit_absolute_messages: instance_rate_limits.limit_absolute_messages,
                limit_absolute_register: instance_rate_limits.limit_absolute_register,
                limit_auth_login: instance_rate_limits.limit_auth_login,
                limit_auth_register: instance_rate_limits.limit_auth_register,
                limit_channel: user_rate_limits.limit_channel,
                limit_error: user_rate_limits.limit_error,
                limit_global: instance_rate_limits.limit_global,
                limit_guild: user_rate_limits.limit_guild,
                limit_ip: instance_rate_limits.limit_ip,
                limit_webhook: user_rate_limits.limit_webhook,
            }
        }

        pub fn get_limit_ref(&self, limit_type: &LimitType) -> &Limit {
            match limit_type {
                &LimitType::AbsoluteMessage => &self.limit_absolute_messages,
                &LimitType::AbsoluteRegister => &self.limit_absolute_register,
                &LimitType::AuthLogin => &self.limit_auth_login,
                &LimitType::AuthRegister => &self.limit_auth_register,
                &LimitType::Channel => &self.limit_channel,
                &LimitType::Error => &self.limit_error,
                &LimitType::Global => &self.limit_global,
                &LimitType::Guild => &self.limit_guild,
                &LimitType::Ip => &self.limit_ip,
                &LimitType::Webhook => &self.limit_webhook,
            }
        }

        pub fn get_limit_mut_ref(&mut self, limit_type: &LimitType) -> &mut Limit {
            match limit_type {
                &LimitType::AbsoluteMessage => &mut self.limit_absolute_messages,
                &LimitType::AbsoluteRegister => &mut self.limit_absolute_register,
                &LimitType::AuthLogin => &mut self.limit_auth_login,
                &LimitType::AuthRegister => &mut self.limit_auth_register,
                &LimitType::Channel => &mut self.limit_channel,
                &LimitType::Error => &mut self.limit_error,
                &LimitType::Global => &mut self.limit_global,
                &LimitType::Guild => &mut self.limit_guild,
                &LimitType::Ip => &mut self.limit_ip,
                &LimitType::Webhook => &mut self.limit_webhook,
            }
        }

        pub fn to_hash_map(&self) -> HashMap<LimitType, Limit> {
            let mut map: HashMap<LimitType, Limit> = HashMap::new();
            map.insert(LimitType::AbsoluteMessage, self.limit_absolute_messages);
            map.insert(LimitType::AbsoluteRegister, self.limit_absolute_register);
            map.insert(LimitType::AuthLogin, self.limit_auth_login);
            map.insert(LimitType::AuthRegister, self.limit_auth_register);
            map.insert(LimitType::Ip, self.limit_ip);
            map.insert(LimitType::Global, self.limit_global);
            map.insert(LimitType::Error, self.limit_error);
            map.insert(LimitType::Guild, self.limit_guild);
            map.insert(LimitType::Webhook, self.limit_webhook);
            map.insert(LimitType::Channel, self.limit_channel);
            map
        }

        pub fn get_as_mut(&mut self) -> &mut Limits {
            self
        }

        /// check_limits uses the API to get the current request limits of the instance.
        /// It returns a `Limits` struct containing all the limits.
        /// If the rate limit is disabled, then the limit is set to `u64::MAX`.
        /// # Errors
        /// This function will panic if the request fails or if the response body cannot be parsed.
        /// TODO: Change this to return a Result and handle the errors properly.
        pub async fn check_limits(api_url: String) -> Limits {
            let client = Client::new();
            let url_parsed = crate::URLBundle::parse_url(api_url) + "/policies/instance/limits";
            let result = client
                .get(url_parsed)
                .send()
                .await
                .unwrap_or_else(|e| panic!("An error occured while performing the request: {}", e))
                .text()
                .await
                .unwrap_or_else(|e| {
                    panic!(
                        "An error occured while parsing the request body string: {}",
                        e
                    )
                });
            let config: Config = from_str(&result).unwrap();
            // If config.rate.enabled is false, then add return a Limits struct with all limits set to u64::MAX
            let mut limits: Limits;
            if !config.rate.enabled {
                limits = Limits {
                    limit_absolute_messages: Limit {
                        bucket: LimitType::AbsoluteMessage,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_absolute_register: Limit {
                        bucket: LimitType::AbsoluteRegister,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_auth_login: Limit {
                        bucket: LimitType::AuthLogin,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_auth_register: Limit {
                        bucket: LimitType::AuthRegister,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_ip: Limit {
                        bucket: LimitType::Ip,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_global: Limit {
                        bucket: LimitType::Global,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_error: Limit {
                        bucket: LimitType::Error,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_guild: Limit {
                        bucket: LimitType::Guild,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_webhook: Limit {
                        bucket: LimitType::Webhook,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                    limit_channel: Limit {
                        bucket: LimitType::Channel,
                        limit: u64::MAX,
                        remaining: u64::MAX,
                        reset: u64::MAX,
                    },
                };
            } else {
                limits = Limits {
                    limit_absolute_messages: Limit {
                        bucket: LimitType::AbsoluteMessage,
                        limit: config.absoluteRate.sendMessage.limit,
                        remaining: config.absoluteRate.sendMessage.limit,
                        reset: config.absoluteRate.sendMessage.window,
                    },
                    limit_absolute_register: Limit {
                        bucket: LimitType::AbsoluteRegister,
                        limit: config.absoluteRate.register.limit,
                        remaining: config.absoluteRate.register.limit,
                        reset: config.absoluteRate.register.window,
                    },
                    limit_auth_login: Limit {
                        bucket: LimitType::AuthLogin,
                        limit: config.rate.routes.auth.login.count,
                        remaining: config.rate.routes.auth.login.count,
                        reset: config.rate.routes.auth.login.window,
                    },
                    limit_auth_register: Limit {
                        bucket: LimitType::AuthRegister,
                        limit: config.rate.routes.auth.register.count,
                        remaining: config.rate.routes.auth.register.count,
                        reset: config.rate.routes.auth.register.window,
                    },
                    limit_ip: Limit {
                        bucket: LimitType::Ip,
                        limit: config.rate.ip.count,
                        remaining: config.rate.ip.count,
                        reset: config.rate.ip.window,
                    },
                    limit_global: Limit {
                        bucket: LimitType::Global,
                        limit: config.rate.global.count,
                        remaining: config.rate.global.count,
                        reset: config.rate.global.window,
                    },
                    limit_error: Limit {
                        bucket: LimitType::Error,
                        limit: config.rate.error.count,
                        remaining: config.rate.error.count,
                        reset: config.rate.error.window,
                    },
                    limit_guild: Limit {
                        bucket: LimitType::Guild,
                        limit: config.rate.routes.guild.count,
                        remaining: config.rate.routes.guild.count,
                        reset: config.rate.routes.guild.window,
                    },
                    limit_webhook: Limit {
                        bucket: LimitType::Webhook,
                        limit: config.rate.routes.webhook.count,
                        remaining: config.rate.routes.webhook.count,
                        reset: config.rate.routes.webhook.window,
                    },
                    limit_channel: Limit {
                        bucket: LimitType::Channel,
                        limit: config.rate.routes.channel.count,
                        remaining: config.rate.routes.channel.count,
                        reset: config.rate.routes.channel.window,
                    },
                };
            }

            if !config.absoluteRate.register.enabled {
                limits.limit_absolute_register = Limit {
                    bucket: LimitType::AbsoluteRegister,
                    limit: u64::MAX,
                    remaining: u64::MAX,
                    reset: u64::MAX,
                };
            }

            if !config.absoluteRate.sendMessage.enabled {
                limits.limit_absolute_messages = Limit {
                    bucket: LimitType::AbsoluteMessage,
                    limit: u64::MAX,
                    remaining: u64::MAX,
                    reset: u64::MAX,
                };
            }

            limits
        }
    }
}

#[cfg(test)]
mod instance_limits {
    use crate::api::limits::{Limit, LimitType};

    #[test]
    fn limit_below_zero() {
        let mut limit = Limit {
            bucket: LimitType::AbsoluteMessage,
            limit: 0,
            remaining: 1,
            reset: 0,
        };
        limit.add_remaining(-2);
        assert_eq!(0_u64, limit.remaining);
        limit.add_remaining(-2123123);
        assert_eq!(0_u64, limit.remaining);
    }
}
