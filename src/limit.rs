use crate::api::limits::{Limit, LimitType, Limits};

use reqwest::{Client, RequestBuilder, Response};
use std::collections::{HashMap, VecDeque};

// Note: There seem to be some overlapping request limiters. We need to make sure that sending a
// request checks for all the request limiters that apply, and blocks if any of the limiters are 0

#[allow(dead_code)]
#[derive(Debug)]
pub struct TypedRequest {
    request: RequestBuilder,
    limit_type: LimitType,
}

#[derive(Debug)]
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

    /**
    # send_request
    Checks, if a request can be sent without hitting API rate limits and sends it, if true.
    Will automatically update the rate limits of the LimitedRequester the request has been
    sent with.

    ## Arguments
    - `request`: A [`RequestBuilder`](reqwest::RequestBuilder) that contains a request ready to be
    sent. Unfinished or invalid requests will result in the method panicing.
    - `limit_type`: Because this library does not yet implement a way to check for which rate limit
    will be used when the request gets send, you will have to specify this manually using a
    [`LimitType`](crate::api::limits::LimitType) enum.

    ## Returns
    - `Response`: The [`Response`](`reqwest::Response`) gotten from sending the request to the
    server. This will be returned if the Request was built and send successfully. Is wrapped in
    an [`Option`](`core::option::Option`)
    - `None`: [`None`](`core::option::Option`) will be returned if the rate limit has been hit, and
    the request could therefore not have been sent.

    ## Errors

    This method will panic, if:
    - The supplied [`RequestBuilder`](reqwest::RequestBuilder) contains invalid or incomplete
    information
    - There has been an error with processing (unwrapping) the [`Response`](`reqwest::Response`)
    - The call to [`update_limits`](`crate::limits::update_limits`) yielded errors. Read the
    methods' Errors section for more information.
    */
    pub async fn send_request(
        &mut self,
        request: RequestBuilder,
        limit_type: LimitType,
    ) -> Option<Response> {
        if self.can_send_request(limit_type) {
            let built_request = request
                .build()
                .unwrap_or_else(|e| panic!("Error while building the Request for sending: {}", e));
            let result = self.http.execute(built_request).await;
            let response = match result {
                Ok(is_response) => is_response,
                Err(e) => panic!("An error occured while processing the response: {}", e),
            };
            self.update_limits(&response, limit_type);
            return Some(response);
        } else {
            self.requests.push_back(TypedRequest {
                request: request,
                limit_type: limit_type,
            });
            return None;
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

    fn can_send_request(&mut self, limit_type: LimitType) -> bool {
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

    fn update_limits(&mut self, response: &Response, limit_type: LimitType) {
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

#[cfg(test)]
mod rate_limit {
    use super::*;
    use crate::URLBundle;
    #[tokio::test]
    async fn create_limited_requester() {
        let urls = URLBundle::new(
            String::from("http://localhost:3001/api/"),
            String::from("wss://localhost:3001/"),
            String::from("http://localhost:3001/cdn"),
        );
        let requester = LimitedRequester::new(urls.api).await;
        assert_eq!(
            requester.limits_rate.get(&LimitType::Ip).unwrap(),
            &Limit {
                bucket: LimitType::Ip,
                limit: 500,
                remaining: 500,
                reset: 5
            }
        );
    }
}
