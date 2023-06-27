use crate::{
    api::limits::{Limit, LimitType, Limits, LimitsMutRef},
    errors::{ChorusLibError, ChorusResult},
    instance::Instance,
};
use reqwest::{RequestBuilder, Response};

#[derive(Debug)]
pub struct LimitedRequester;

impl LimitedRequester {
    /// Checks if a request can be sent without hitting API rate limits and sends it, if true.
    /// Will automatically update the rate limits of the LimitedRequester the request has been
    /// sent with.
    ///
    /// # Arguments
    ///
    /// * `request`: A `RequestBuilder` that contains a request ready to be sent. Unfinished or
    /// invalid requests will result in the method panicing.
    /// * `limit_type`: Because this library does not yet implement a way to check for which rate
    /// limit will be used when the request gets send, you will have to specify this manually using
    /// a `LimitType` enum.
    ///
    /// # Returns
    ///
    /// * `Response`: The `Response` gotten from sending the request to the server. This will be
    /// returned if the Request was built and send successfully. Is wrapped in an `Option`.
    /// * `None`: `None` will be returned if the rate limit has been hit, and the request could
    /// therefore not have been sent.
    ///
    /// # Errors
    ///
    /// This method will error if:
    ///
    /// * The request does not return a success status code (200-299)
    /// * The supplied `RequestBuilder` contains invalid or incomplete information
    /// * There has been an error with processing (unwrapping) the `Response`
    /// * The call to `update_limits` yielded errors. Read the methods' Errors section for more
    /// information.
    pub async fn send_request(
        request: RequestBuilder,
        limit_type: LimitType,
        instance: &mut Instance,
        user_rate_limits: &mut Limits,
    ) -> ChorusResult<Response> {
        if LimitedRequester::can_send_request(limit_type, &instance.limits, user_rate_limits) {
            let built_request = match request.build() {
                Ok(request) => request,
                Err(e) => {
                    return Err(ChorusLibError::RequestErrorError {
                        url: "".to_string(),
                        error: e.to_string(),
                    });
                }
            };
            let result = instance.client.execute(built_request).await;
            let response = match result {
                Ok(is_response) => is_response,
                Err(e) => {
                    return Err(ChorusLibError::ReceivedErrorCodeError {
                        error_code: e.to_string(),
                    });
                }
            };
            LimitedRequester::update_limits(
                &response,
                limit_type,
                &mut instance.limits,
                user_rate_limits,
            );
            if !response.status().is_success() {
                match response.status().as_u16() {
                    401 => Err(ChorusLibError::TokenExpired),
                    403 => Err(ChorusLibError::TokenExpired),
                    _ => Err(ChorusLibError::ReceivedErrorCodeError {
                        error_code: response.status().as_str().to_string(),
                    }),
                }
            } else {
                Ok(response)
            }
        } else {
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

    use crate::{api::limits::Config, UrlBundle};

    use super::*;

    #[tokio::test]
    async fn run_into_limit() {
        let urls = UrlBundle::new(
            String::from("http://localhost:3001/api/"),
            String::from("wss://localhost:3001/"),
            String::from("http://localhost:3001/cdn"),
        );
        let mut instance = Instance::new(urls.clone()).await.unwrap();
        let mut user_rate_limits = Limits::check_limits(urls.api.clone()).await;

        let limit_type = LimitType::Channel;
        let mut ran_into_limits = false;
        for _ in 0..=50 {
            let request_path = urls.api.clone() + "/some/random/nonexisting/path";
            let request_builder = instance.client.get(request_path);
            let request = LimitedRequester::send_request(
                request_builder,
                limit_type,
                &mut instance,
                &mut user_rate_limits,
            )
            .await;
            ran_into_limits = matches!(request, Err(ChorusLibError::RateLimited { bucket}) if bucket == limit_type.to_string());
        }
        assert!(ran_into_limits);
    }

    #[tokio::test]
    async fn test_send_request() {
        let urls = UrlBundle::new(
            String::from("http://localhost:3001/api/"),
            String::from("wss://localhost:3001/"),
            String::from("http://localhost:3001/cdn"),
        );
        let mut instance = Instance::new(urls.clone()).await.unwrap();
        let mut user_rate_limits = Limits::check_limits(urls.api.clone()).await;
        let request_path = urls.api.clone() + "/policies/instance/limits";
        let request_builder = instance.client.get(request_path);
        let request = LimitedRequester::send_request(
            request_builder,
            LimitType::Channel,
            &mut instance,
            &mut user_rate_limits,
        )
        .await;
        from_str::<Config>(request.unwrap().text().await.unwrap().as_str()).unwrap();
    }
}
