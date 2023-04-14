use crate::api::limits::{Limit, LimitType, Limits};

use reqwest::{Client, RequestBuilder, Response};
use std::collections::{HashMap, VecDeque};

// Note: There seem to be some overlapping request limiters. We need to make sure that sending a
// request checks for all the request limiters that apply, and blocks if any of the limiters are 0

#[allow(dead_code)]

pub struct TypedRequest {
    request: RequestBuilder,
    limit_type: LimitType,
}
pub struct LimitedRequester {
    http: Client,
    requests: VecDeque<TypedRequest>,
    limits_rate: HashMap<LimitType, Limit>,
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    #[allow(dead_code)]
    pub async fn new(api_url: String) -> Self {
        LimitedRequester {
            http: Client::new(),
            requests: VecDeque::new(),
            limits_rate: Limits::check_limits(api_url).await,
        }
    }

    pub async fn send_request(&mut self, request: RequestBuilder, limit_type: LimitType) {
        if self.can_send_request(limit_type) {
            let built_request = request
                .build()
                .unwrap_or_else(|e| panic!("Error while building the Request for sending: {}", e));
            let result = self.http.execute(built_request).await;
            let response = match result {
                Ok(is_response) => is_response,
                Err(e) => panic!("An error occured while processing the response: {}", e),
            };
            self.update_limits(response, limit_type);
        } else {
            self.requests.push_back(TypedRequest {
                request: request,
                limit_type: limit_type,
            });
        }
    }

    fn update_limit_entry(entry: &mut Limit, reset: u64, remaining: u64, limit: u64) {
        if reset != entry.reset {
            entry.reset = reset;
            entry.remaining = limit;
            entry.limit = limit;
        } else {
            entry.remaining = remaining;
            entry.limit = limit;
        }
    }

    pub fn can_send_request(&mut self, limit_type: LimitType) -> bool {
        let limits = self.limits_rate.get(&limit_type);

        match limits {
            Some(limit) => {
                if limit.remaining > 0 {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn update_limits(&mut self, response: Response, limit_type: LimitType) {
        // TODO: Make this work
        let remaining = match response.headers().get("X-RateLimit-Remaining") {
            Some(remaining) => remaining.to_str().unwrap().parse::<u64>().unwrap(),
            None => return, //false,
        };
        let limit = match response.headers().get("X-RateLimit-Limit") {
            Some(limit) => limit.to_str().unwrap().parse::<u64>().unwrap(),
            None => return, //false,
        };
        let reset = match response.headers().get("X-RateLimit-Reset") {
            Some(reset) => reset.to_str().unwrap().parse::<u64>().unwrap(),
            None => return, //false,
        };

        let mut limits_copy = self.limits_rate.clone();
        let status = response.status();
        let status_str = status.as_str();

        if status_str.chars().next().unwrap() == '4' {
            limits_copy.get_mut(&LimitType::Error).unwrap().remaining -= 1;
        }

        limits_copy.get_mut(&LimitType::Global).unwrap().remaining -= 1;
        limits_copy.get_mut(&LimitType::Ip).unwrap().remaining -= 1;

        match limit_type {
            // Error, Global and Ip get handled seperately.
            LimitType::Error => {}
            LimitType::Global => {}
            LimitType::Ip => {}
            LimitType::AuthLogin => {
                let entry = limits_copy.get_mut(&LimitType::AuthLogin).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
            }
            LimitType::AbsoluteRegister => {
                let entry = limits_copy.get_mut(&LimitType::AbsoluteRegister).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
                // AbsoluteRegister and AuthRegister both need to be updated, if a Register event
                // happens.
                limits_copy
                    .get_mut(&LimitType::AuthRegister)
                    .unwrap()
                    .remaining -= 1;
            }
            LimitType::AuthRegister => {
                let entry = limits_copy.get_mut(&LimitType::AuthRegister).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
                // AbsoluteRegister and AuthRegister both need to be updated, if a Register event
                // happens.
                limits_copy
                    .get_mut(&LimitType::AbsoluteRegister)
                    .unwrap()
                    .remaining -= 1;
            }
            LimitType::AbsoluteMessage => {
                let entry = limits_copy.get_mut(&LimitType::AbsoluteMessage).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
            }
            LimitType::Channel => {
                let entry = limits_copy.get_mut(&LimitType::Channel).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
            }
            LimitType::Guild => {
                let entry = limits_copy.get_mut(&LimitType::Guild).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
            }
            LimitType::Webhook => {
                let entry = limits_copy.get_mut(&LimitType::Webhook).unwrap();
                LimitedRequester::update_limit_entry(entry, reset, limit, limit);
            }
        }
    }
}
