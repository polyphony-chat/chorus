use crate::api::limits::Config;

use reqwest::{Client, Request};
use serde_json::from_str;
use std::collections::VecDeque;

// Note: There seem to be some overlapping request limiters. We need to make sure that sending a
// request checks for all the request limiters that apply, and blocks if any of the limiters are 0

pub struct Limit {
    limit: u64,
    remaining: u64,
    reset: u64,
    bucket: String,
}

impl std::fmt::Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Bucket: {}, Limit: {}, Remaining: {}, Reset: {}",
            self.bucket, self.limit, self.remaining, self.reset
        )
    }
}

pub struct LimitedRequester {
    http: Client,
    limits: Vec<Limit>,
    requests: VecDeque<Request>,
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    pub async fn new(api_url: String) -> Self {
        LimitedRequester {
            limits: LimitedRequester::check_limits(api_url).await,
            http: Client::new(),
            requests: VecDeque::new(),
        }
    }

    pub async fn check_limits(url: String) -> Vec<Limit> {
        let client = Client::new();
        let url_parsed = crate::URLBundle::parse_url(url) + "/api/policies/instance/limits";
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

        let mut limit_vector = Vec::new();
        if !config.rate.enabled {
            let types = [
                "rate.ip",
                "rate.routes.auth.login",
                "rate.routes.auth.register",
            ];
            for type_ in types.iter() {
                limit_vector.push(Limit {
                    limit: u64::MAX,
                    remaining: u64::MAX,
                    reset: 1,
                    bucket: String::from(*type_),
                });
            }
        } else {
            limit_vector.push(Limit {
                limit: config.rate.ip.count,
                remaining: config.rate.ip.count,
                reset: config.rate.ip.window,
                bucket: String::from("rate.ip"),
            });
            limit_vector.push(Limit {
                limit: config.rate.global.count,
                remaining: config.rate.global.count,
                reset: config.rate.global.window,
                bucket: String::from("rate.global"),
            });
            limit_vector.push(Limit {
                limit: config.rate.error.count,
                remaining: config.rate.error.count,
                reset: config.rate.error.window,
                bucket: String::from("rate.error"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.guild.count,
                remaining: config.rate.routes.guild.count,
                reset: config.rate.routes.guild.window,
                bucket: String::from("rate.routes.guild"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.webhook.count,
                remaining: config.rate.routes.webhook.count,
                reset: config.rate.routes.webhook.window,
                bucket: String::from("rate.routes.webhook"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.channel.count,
                remaining: config.rate.routes.channel.count,
                reset: config.rate.routes.channel.window,
                bucket: String::from("rate.routes.channel"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.auth.login.count,
                remaining: config.rate.routes.auth.login.count,
                reset: config.rate.routes.auth.login.window,
                bucket: String::from("rate.routes.auth.login"),
            });

            limit_vector.push(Limit {
                limit: config.rate.routes.auth.register.count,
                remaining: config.rate.routes.auth.register.count,
                reset: config.rate.routes.auth.register.window,
                bucket: String::from("rate.routes.auth.register"),
            });
        }

        if config.absoluteRate.register.enabled {
            limit_vector.push(Limit {
                limit: config.absoluteRate.register.limit,
                remaining: config.absoluteRate.register.limit,
                reset: config.absoluteRate.register.window,
                bucket: String::from("absoluteRate.register"),
            });
        }
        if config.absoluteRate.sendMessage.enabled {
            limit_vector.push(Limit {
                limit: config.absoluteRate.sendMessage.limit,
                remaining: config.absoluteRate.sendMessage.limit,
                reset: config.absoluteRate.sendMessage.window,
                bucket: String::from("absoluteRate.sendMessage"),
            });
        }

        limit_vector
    }
}
