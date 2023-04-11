use crate::api::limits::Config;

use reqwest::{Client, Request, RequestBuilder};
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
    limits: Vec<Limit>, // TODO: Replace with all Limits
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

    fn get_limit(&self, bucket: &str) -> Option<&Limit> {
        for limit in self.limits.iter() {
            if limit.bucket == bucket {
                return Some(limit.to_owned());
            }
        }
        None
    }

    /// `last_match` returns the last match of a `Vec<&str>` in a `String`. It returns a `Option<(usize, String)>`
    /// that contains the index of the match and the match itself.
    /// If no match is found, it returns `None`.
    /// # Example
    /// ```rs
    /// let string = "https://discord.com/api/v8/channels/1234567890/messages";
    /// let matches = ["channels", "messages"];
    /// let index_match = last_match(string, matches.to_vec());
    /// assert_eq!(index_match, Some((43, String::from("messages"))));
    /// ```
    fn last_match(string: &str, search_for: Vec<&str>) -> Option<(usize, String)> {
        let mut index_match: (usize, &str) = (0, "");
        for _match in search_for.iter() {
            if !string.contains(_match) {
                continue;
            }
            // Get the index of the match
            let temp_index_match = string.match_indices(_match).next().unwrap();
            // As only the last match is relevant, we only update the index_match if the index is
            // higher than the previous one.
            if temp_index_match.0 > index_match.0 {
                index_match = temp_index_match;
            }
        }
        if index_match.0 == 0 {
            return None;
        } else {
            return Some((index_match.0, String::from(index_match.1)));
        }
    }

    /// `can_send_request` checks if a request can be sent. It returns a `bool` that indicates if
    /// the request can be sent.
    fn can_send_request(&mut self, request: RequestBuilder) -> bool {
        // get the url from request
        let global_limit = self.get_limit("global").unwrap();
        let ip_limit = self.get_limit("ip").unwrap();

        if ip_limit.remaining == 0 || global_limit.remaining == 0 {
            return false;
        }

        let url_path = request
            .try_clone()
            .unwrap()
            .build()
            .unwrap()
            .url()
            .path()
            .to_string();
        // Define the different rate limit buckets as they would appear in the URL
        let matches = [
            "login", "register", "webhooks", "channels", "messages", "guilds",
        ];
        let index_match_string = LimitedRequester::last_match(&url_path, matches.to_vec())
            .unwrap_or_else(|| (0, String::from("")));
        if index_match_string.0 == 0 {
            return true;
        }
        let index_match = (index_match_string.0, index_match_string.1.as_str());
        match index_match.1 {
            "login" => {
                let auth_limit = self.get_limit("login").unwrap();
                if auth_limit.remaining != 0 {
                    return true;
                }
                return false;
            }
            "register" => {
                let auth_limit = self.get_limit("auth.register").unwrap();
                let absolute_limit = self.get_limit("absoluteRate.register").unwrap();
                if auth_limit.remaining != 0 && absolute_limit.remaining != 0 {
                    return true;
                }
                return false;
            }
            "messages" => {
                let absolute_limit = self.get_limit("absoluteRate.sendMessages").unwrap();
                let request_method = request
                    .try_clone()
                    .unwrap()
                    .build()
                    .unwrap()
                    .method()
                    .as_str()
                    .to_owned();
                if absolute_limit.remaining != 0
                    || request_method != "POST"
                    || request_method != "PUT"
                    || request_method != "PATCH"
                {
                    return true;
                }
                return false;
            }
            "webhooks" => {
                let auth_limit = self.get_limit("webhooks").unwrap();
                if auth_limit.remaining != 0 {
                    return true;
                }
                return false;
            }
            "channels" => {
                let auth_limit = self.get_limit("channels").unwrap();
                if auth_limit.remaining != 0 {
                    return true;
                }
                return false;
            }
            "guilds" => {
                let auth_limit = self.get_limit("guilds").unwrap();
                if auth_limit.remaining != 0 {
                    return true;
                }
                return false;
            }
            &_ => {
                panic!();
            }
        }
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
    }

    /// `send_request` sends a request to the server, if `can_send_request()` is true.
    /// It will then update the `Limit`s by calling `update_limits()`.
    /// # Example
    pub async fn send_request(&mut self, request: RequestBuilder) -> reqwest::Response {
        if !self.can_send_request(request.try_clone().unwrap()) {
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
            // The different rate limit buckets, except for the absoluteRate ones. These will be
            // handled seperately.
            let types = [
                "ip",
                "auth.login",
                "auth.register",
                "global",
                "error",
                "guild",
                "webhook",
                "channel",
            ];
            for type_ in types.iter() {
                limit_vector.push(Limit {
                    limit: u64::MAX,
                    remaining: u64::MAX,
                    reset: 1,
                    bucket: type_.to_string(),
                });
            }
        } else {
            limit_vector.push(Limit {
                limit: config.rate.ip.count,
                remaining: config.rate.ip.count,
                reset: config.rate.ip.window,
                bucket: String::from("ip"),
            });
            limit_vector.push(Limit {
                limit: config.rate.global.count,
                remaining: config.rate.global.count,
                reset: config.rate.global.window,
                bucket: String::from("global"),
            });
            limit_vector.push(Limit {
                limit: config.rate.error.count,
                remaining: config.rate.error.count,
                reset: config.rate.error.window,
                bucket: String::from("error"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.guild.count,
                remaining: config.rate.routes.guild.count,
                reset: config.rate.routes.guild.window,
                bucket: String::from("guild"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.webhook.count,
                remaining: config.rate.routes.webhook.count,
                reset: config.rate.routes.webhook.window,
                bucket: String::from("webhook"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.channel.count,
                remaining: config.rate.routes.channel.count,
                reset: config.rate.routes.channel.window,
                bucket: String::from("channel"),
            });
            limit_vector.push(Limit {
                limit: config.rate.routes.auth.login.count,
                remaining: config.rate.routes.auth.login.count,
                reset: config.rate.routes.auth.login.window,
                bucket: String::from("auth.login"),
            });

            limit_vector.push(Limit {
                limit: config.rate.routes.auth.register.count,
                remaining: config.rate.routes.auth.register.count,
                reset: config.rate.routes.auth.register.window,
                bucket: String::from("auth.register"),
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
                bucket: String::from("absoluteRate.messages"),
            });
        }

        limit_vector
    }
}
