use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::from_str;

use crate::{
    api::limits::{Limit, LimitType},
    errors::{ChorusError, ChorusResult},
    instance::UserMeta,
    types::LimitsConfiguration,
};

pub struct ChorusRequest {
    pub request: RequestBuilder,
    pub limit_type: LimitType,
}

impl ChorusRequest {
    pub async fn send_request(self, user: &mut UserMeta) -> ChorusResult<Response> {
        let belongs_to = user.belongs_to.borrow();
        if !ChorusRequest::can_send_request(&user, &self.limit_type) {
            return Err(ChorusError::RateLimited {
                bucket: format!("{:?}", self.limit_type),
            });
        }
        let result = match belongs_to
            .client
            .execute(self.request.build().unwrap())
            .await
        {
            Ok(result) => result,
            Err(error) => {
                return Err(ChorusError::RequestErrorError {
                    url: error.url().unwrap().to_string(),
                    error,
                })
            }
        };
        drop(belongs_to);
        ChorusRequest::update_rate_limits(user, &self.limit_type, !result.status().is_success());
        if !result.status().is_success() {
            if result.status().as_u16() == 429 {
                user.limits
                    .as_mut()
                    .unwrap()
                    .get_mut(&self.limit_type)
                    .unwrap()
                    .remaining = 0;
                return Err(ChorusError::RateLimited {
                    bucket: format!("{:?}", self.limit_type),
                });
            }
            return Err(ChorusRequest::interpret_error(result).await);
        }
        Ok(result)
    }

    fn can_send_request(user: &UserMeta, limit_type: &LimitType) -> bool {
        let belongs_to = user.belongs_to.borrow();
        if belongs_to.limits.is_none() {
            return true;
        }
        let instance_dictated_limits = [
            &LimitType::AuthLogin,
            &LimitType::AuthRegister,
            &LimitType::Global,
            &LimitType::Ip,
        ];
        let limits: &mut HashMap<LimitType, Limit>;
        if instance_dictated_limits.contains(&limit_type) {
            limits = &mut belongs_to.limits.unwrap();
        } else {
            limits = &mut user.limits.unwrap();
        }
        let global = belongs_to
            .limits
            .as_ref()
            .unwrap()
            .get(&LimitType::Global)
            .unwrap();
        let ip = belongs_to
            .limits
            .as_ref()
            .unwrap()
            .get(&LimitType::Ip)
            .unwrap();
        let limit_type_limit = limits.get(limit_type).unwrap();
        if global.remaining == 0 || ip.remaining == 0 || limit_type_limit.remaining == 0 {
            return false;
        }
        true
    }

    async fn interpret_error(response: reqwest::Response) -> ChorusError {
        match response.status().as_u16() {
            200..=299 => ChorusError::InvalidArgumentsError {
                error: "You somehow passed a successful request into this function, which is not allowed."
                    .to_string(),
            },
            401..=403 | 407 => ChorusError::NoPermission,
            404 => ChorusError::NotFound {
                error: response.text().await.unwrap(),
            },
            405 | 408 | 409 => ChorusError::ReceivedErrorCodeError { error_code: response.status().as_u16(), error: response.text().await.unwrap() },
            411..=421 | 426 | 428 | 431 => ChorusError::InvalidArgumentsError {
                error: response.text().await.unwrap(),
            },
            429 => panic!("Illegal state: Rate limit exception should have been caught before this function call."),
            451 => ChorusError::NoResponse,
            500..=599 => ChorusError::ReceivedErrorCodeError { error_code: response.status().as_u16(), error: response.text().await.unwrap() },
            _ => ChorusError::ReceivedErrorCodeError { error_code: response.status().as_u16(), error: response.text().await.unwrap()},
        }
    }

    fn update_rate_limits(user: &mut UserMeta, limit_type: &LimitType, response_was_err: bool) {
        let mut belongs_to = user.belongs_to.borrow_mut();
        if belongs_to.limits.is_none() {
            return;
        }
        let instance_dictated_limits = [
            &LimitType::AuthLogin,
            &LimitType::AuthRegister,
            &LimitType::Global,
            &LimitType::Ip,
        ];
        let mut relevant_limits = Vec::new();
        if instance_dictated_limits.contains(&limit_type) {
            relevant_limits.push(
                belongs_to
                    .limits
                    .as_mut()
                    .unwrap()
                    .get_mut(limit_type)
                    .unwrap(),
            );
        } else {
            relevant_limits.push(user.limits.as_mut().unwrap().get_mut(limit_type).unwrap());
        }
        relevant_limits.push(
            belongs_to
                .limits
                .as_mut()
                .unwrap()
                .get_mut(&LimitType::Global)
                .unwrap(),
        );
        relevant_limits.push(
            belongs_to
                .limits
                .as_mut()
                .unwrap()
                .get_mut(&LimitType::Ip)
                .unwrap(),
        );
        if response_was_err {
            relevant_limits.push(
                user.limits
                    .as_mut()
                    .unwrap()
                    .get_mut(&LimitType::Error)
                    .unwrap(),
            );
        }
        let time: u64 = chrono::Utc::now().timestamp() as u64;
        for limit in relevant_limits.iter() {
            let limit = *limit; // deref here so we don't have to do it later
            if time > limit.reset {
                let limit_from_instance_config = belongs_to
                    .limits_configuration
                    .unwrap()
                    .rate
                    .to_hash_map()
                    .get(&limit.bucket)
                    .unwrap();
                // Spacebar does not yet return rate limit information in its response headers. We
                // therefore have to guess the next rate limit window. This is not ideal. Oh well!
                limit.reset = limit_from_instance_config.window + time;
                limit.remaining = limit.limit;
            }
            limit.remaining -= 1;
        }
    }

    pub async fn get_limits_config(url_api: &str) -> ChorusResult<LimitsConfiguration> {
        let request = Client::new()
            .get(format!("{}/policies/instance/limits/", url_api))
            .send()
            .await;
        let request = match request {
            Ok(request) => request,
            Err(e) => {
                return Err(ChorusError::RequestErrorError {
                    url: url_api.to_string(),
                    error: e,
                })
            }
        };
        let limits_configuration = match request.status().as_u16() {
            200 => from_str::<LimitsConfiguration>(&request.text().await.unwrap()).unwrap(),
            429 => {
                return Err(ChorusError::RateLimited {
                    bucket: format!("{:?}", LimitType::Ip),
                })
            }
            404 => return Err(ChorusError::NotFound { error: format!("Route \"/policies/instance/limits/\" not found. Are you perhaps trying to request the Limits configuration from an unsupported server?") }),
            400..=u16::MAX => {
                return Err(ChorusError::ReceivedErrorCodeError { error_code: request.status().as_u16(), error: request.text().await.unwrap() })
            }
            _ => {
                return Err(ChorusError::InvalidResponseError {
                    error: request.text().await.unwrap(),
                })
            }
        };

        Ok(limits_configuration)
    }

    pub fn limits_config_to_hashmap(
        limits_configuration: &LimitsConfiguration,
    ) -> HashMap<LimitType, Limit> {
        let config = limits_configuration.rate;
        let routes = config.routes;
        let mut map: HashMap<LimitType, Limit> = HashMap::new();
        map.insert(
            LimitType::AuthLogin,
            Limit {
                bucket: LimitType::AuthLogin,
                limit: routes.auth.login.count,
                remaining: routes.auth.login.count,
                reset: routes.auth.login.window,
            },
        );
        map.insert(
            LimitType::AuthRegister,
            Limit {
                bucket: LimitType::AuthRegister,
                limit: routes.auth.register.count,
                remaining: routes.auth.register.count,
                reset: routes.auth.register.window,
            },
        );
        map.insert(
            LimitType::Channel,
            Limit {
                bucket: LimitType::Channel,
                limit: routes.channel.count,
                remaining: routes.channel.count,
                reset: routes.channel.window,
            },
        );
        map.insert(
            LimitType::Error,
            Limit {
                bucket: LimitType::Error,
                limit: config.error.count,
                remaining: config.error.count,
                reset: config.error.window,
            },
        );
        map.insert(
            LimitType::Global,
            Limit {
                bucket: LimitType::Global,
                limit: config.global.count,
                remaining: config.global.count,
                reset: config.global.window,
            },
        );
        map.insert(
            LimitType::Ip,
            Limit {
                bucket: LimitType::Ip,
                limit: config.ip.count,
                remaining: config.ip.count,
                reset: config.ip.window,
            },
        );
        map
    }

    /// Sends a request to wherever it needs to go. Returns [`Ok(())`] on success and
    /// [`Err(ChorusLibError)`] on failure.
    pub async fn handle_request_as_result(self, user: &mut UserMeta) -> ChorusResult<()> {
        match self.send_request(user).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn deserialize_response<T: for<'a> Deserialize<'a>>(
        self,
        user: &mut UserMeta,
    ) -> ChorusResult<T> {
        let response = self.send_request(user).await?;
        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponseError {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                });
            }
        };
        let object = match from_str::<T>(&response_text) {
            Ok(object) => object,
            Err(e) => {
                return Err(ChorusError::InvalidResponseError {
                    error: format!(
                        "Error while trying to deserialize the JSON response into T: {}",
                        e
                    ),
                })
            }
        };
        Ok(object)
    }
}
