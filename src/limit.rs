use crate::api::limits::Config;

use reqwest::{Body, Client, Request, RequestBuilder};
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
    last_reset_epoch: i64,
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
            last_reset_epoch: chrono::Utc::now().timestamp(),
        }
    }

    /// `can_send_request` checks if a request can be sent. It returns a `bool` that indicates if
    /// the request can be sent.
    fn can_send_request(&mut self) -> bool {
        let mut can_send = true;
        for limit in self.limits.iter_mut() {
            if limit.remaining == 0 {
                can_send = false;
            }
        }
        can_send
    }

    /// `update_limits` updates the `Limit`s of the `LimitedRequester` based on the response headers
    /// of the request that was just sent.
    fn update_limits(&mut self, response: &reqwest::Response) {
        let headers = response.headers();
        let mut reset_epoch = 0;
        for limit in self.limits.iter_mut() {
            if let Some(value) = headers.get(limit.bucket.clone()) {
                limit.remaining = value.to_str().unwrap().parse().unwrap();
            }
            if let Some(value) = headers.get("X-RateLimit-Reset".to_string()) {
                reset_epoch = value.to_str().unwrap().parse().unwrap();
            }
        }
        if reset_epoch != 0 {
            self.last_reset_epoch = reset_epoch;
        }

        let httpclient = reqwest::Client::new();
    }

    /// `send_request` sends a request to the server, if `can_send_request()` is true.
    /// It will then update the `Limit`s by calling `update_limits()`.
    /// # Example
    pub async fn send_request(&mut self, request: RequestBuilder) -> reqwest::Response {
        if !self.can_send_request() {
            panic!("429: Rate limited");
        }
        let response = request.send().await.unwrap();
        self.update_limits(&response);
        // TODO: This does not use the request queue at all. Implement! >:3
        response
    }

    /// check_limits uses the API to get the current request limits of the instance.
    /// It returns a `Vec` of `Limit`s, which can be used to check if a request can be sent.
    pub async fn check_limits(api_url: String) -> Vec<Limit> {
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
