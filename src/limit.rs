use std::collections::VecDeque;

use reqwest::{Client, RequestBuilder, Response};

use crate::{
    api::limits::{Limit, LimitType, Limits, LimitsMutRef},
    errors::ChorusLibError,
};

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
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    #[allow(dead_code)]
    pub async fn new() -> Self {
        LimitedRequester {
            http: Client::new(),
            requests: VecDeque::new(),
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

    This method will error, if:
    - The request does not return a success status code (200-299)
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
        instance_rate_limits: &mut Limits,
        user_rate_limits: &mut Limits,
    ) -> Result<Response, ChorusLibError> {
        if self.can_send_request(limit_type, instance_rate_limits, user_rate_limits) {
            let built_request = match request.build() {
                Ok(request) => request,
                Err(e) => {
                    return Err(ChorusLibError::RequestErrorError {
                        url: "".to_string(),
                        error: e.to_string(),
                    });
                }
            };
            let result = self.http.execute(built_request).await;
            let response = match result {
                Ok(is_response) => is_response,
                Err(e) => {
                    return Err(ChorusLibError::ReceivedErrorCodeError {
                        error_code: e.to_string(),
                    });
                }
            };
            self.update_limits(
                &response,
                limit_type,
                instance_rate_limits,
                user_rate_limits,
            );
            if !response.status().is_success() {
                match response.status().as_u16() {
                    401 => return Err(ChorusLibError::TokenExpired),
                    403 => return Err(ChorusLibError::TokenExpired),
                    _ => {
                        return Err(ChorusLibError::ReceivedErrorCodeError {
                            error_code: response.status().as_str().to_string(),
                        });
                    }
                }
            } else {
                Ok(response)
            }
        } else {
            self.requests.push_back(TypedRequest {
                request,
                limit_type,
            });
            Err(ChorusLibError::RateLimited {
                bucket: limit_type.to_string(),
            })
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

    fn can_send_request(
        &mut self,
        limit_type: LimitType,
        instance_rate_limits: &Limits,
        user_rate_limits: &Limits,
    ) -> bool {
        // Check if all of the limits in this vec have at least one remaining request

        let rate_limits = Limits::combine(instance_rate_limits, user_rate_limits);

        let constant_limits: Vec<&LimitType> = [
            &LimitType::Error,
            &LimitType::Global,
            &LimitType::Ip,
            &limit_type,
        ]
        .to_vec();
        for limit in constant_limits.iter() {
            match rate_limits.to_hash_map().get(limit) {
                Some(limit) => {
                    if limit.remaining == 0 {
                        return false;
                    }
                    // AbsoluteRegister and AuthRegister can cancel each other out.
                    if limit.bucket == LimitType::AbsoluteRegister
                        && rate_limits
                            .to_hash_map()
                            .get(&LimitType::AuthRegister)
                            .unwrap()
                            .remaining
                            == 0
                    {
                        return false;
                    }
                    if limit.bucket == LimitType::AuthRegister
                        && rate_limits
                            .to_hash_map()
                            .get(&LimitType::AbsoluteRegister)
                            .unwrap()
                            .remaining
                            == 0
                    {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }

    fn update_limits(
        &mut self,
        response: &Response,
        limit_type: LimitType,
        instance_rate_limits: &mut Limits,
        user_rate_limits: &mut Limits,
    ) {
        let mut rate_limits = LimitsMutRef::combine_mut_ref(instance_rate_limits, user_rate_limits);

        let remaining = match response.headers().get("X-RateLimit-Remaining") {
            Some(remaining) => remaining.to_str().unwrap().parse::<u64>().unwrap(),
            None => rate_limits.get_limit_mut_ref(&limit_type).remaining - 1,
        };
        let limit = match response.headers().get("X-RateLimit-Limit") {
            Some(limit) => limit.to_str().unwrap().parse::<u64>().unwrap(),
            None => rate_limits.get_limit_mut_ref(&limit_type).limit,
        };
        let reset = match response.headers().get("X-RateLimit-Reset") {
            Some(reset) => reset.to_str().unwrap().parse::<u64>().unwrap(),
            None => rate_limits.get_limit_mut_ref(&limit_type).reset,
        };

        let status = response.status();
        let status_str = status.as_str();

        if status_str.starts_with('4') {
            rate_limits
                .get_limit_mut_ref(&LimitType::Error)
                .add_remaining(-1);
        }

        rate_limits
            .get_limit_mut_ref(&LimitType::Global)
            .add_remaining(-1);

        rate_limits
            .get_limit_mut_ref(&LimitType::Ip)
            .add_remaining(-1);

        match limit_type {
            LimitType::Error => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::Error);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::Global => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::Global);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::Ip => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::Ip);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::AuthLogin => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::AuthLogin);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::AbsoluteRegister => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::AbsoluteRegister);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
                // AbsoluteRegister and AuthRegister both need to be updated, if a Register event
                // happens.
                rate_limits
                    .get_limit_mut_ref(&LimitType::AuthRegister)
                    .remaining -= 1;
            }
            LimitType::AuthRegister => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::AuthRegister);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
                // AbsoluteRegister and AuthRegister both need to be updated, if a Register event
                // happens.
                rate_limits
                    .get_limit_mut_ref(&LimitType::AbsoluteRegister)
                    .remaining -= 1;
            }
            LimitType::AbsoluteMessage => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::AbsoluteMessage);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::Channel => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::Channel);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::Guild => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::Guild);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
            LimitType::Webhook => {
                let entry = rate_limits.get_limit_mut_ref(&LimitType::Webhook);
                LimitedRequester::update_limit_entry(entry, reset, remaining, limit);
            }
        }
    }
}

#[cfg(test)]
mod rate_limit {
    use serde_json::from_str;

    use crate::{api::limits::Config, URLBundle};

    use super::*;

    #[tokio::test]
    async fn create_limited_requester() {
        let _urls = URLBundle::new(
            String::from("http://localhost:3001/api/"),
            String::from("wss://localhost:3001/"),
            String::from("http://localhost:3001/cdn"),
        );
        let _requester = LimitedRequester::new().await;
    }

    #[tokio::test]
    async fn run_into_limit() {
        let urls = URLBundle::new(
            String::from("http://localhost:3001/api/"),
            String::from("wss://localhost:3001/"),
            String::from("http://localhost:3001/cdn"),
        );
        let mut requester = LimitedRequester::new().await;
        let mut request: Option<Result<Response, ChorusLibError>> = None;
        let mut instance_rate_limits = Limits::check_limits(urls.api.clone()).await;
        let mut user_rate_limits = Limits::check_limits(urls.api.clone()).await;

        for _ in 0..=50 {
            let request_path = urls.api.clone() + "/some/random/nonexisting/path";
            let request_builder = requester.http.get(request_path);
            request = Some(
                requester
                    .send_request(
                        request_builder,
                        LimitType::Channel,
                        &mut instance_rate_limits,
                        &mut user_rate_limits,
                    )
                    .await,
            );
        }
        if request.is_some() {
            match request.unwrap() {
                Ok(_) => assert!(false),
                Err(_) => assert!(true),
            }
        } else {
            assert!(false)
        }
    }

    #[tokio::test]
    async fn test_send_request() {
        let urls = URLBundle::new(
            String::from("http://localhost:3001/api/"),
            String::from("wss://localhost:3001/"),
            String::from("http://localhost:3001/cdn"),
        );
        let mut instance_rate_limits = Limits::check_limits(urls.api.clone()).await;
        let mut user_rate_limits = Limits::check_limits(urls.api.clone()).await;
        let mut requester = LimitedRequester::new().await;
        let request_path = urls.api.clone() + "/policies/instance/limits";
        let request_builder = requester.http.get(request_path);
        let request = requester
            .send_request(
                request_builder,
                LimitType::Channel,
                &mut instance_rate_limits,
                &mut user_rate_limits,
            )
            .await;
        let result = match request {
            Ok(result) => result,
            Err(_) => panic!("Request failed"),
        };
        let _config: Config = from_str(result.text().await.unwrap().as_str()).unwrap();
    }
}
