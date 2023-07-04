use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response};
use serde_json::from_str;
use strum::IntoEnumIterator;

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
        if belongs_to.limits.is_some() && !ChorusRequest::can_send_request(&user, &self.limit_type)
        {
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
            Err(e) => {
                return Err(ChorusError::ReceivedErrorCodeError {
                    error_code: e.to_string(),
                })
            }
        };
        drop(belongs_to);
        ChorusRequest::update_rate_limits(user, &self.limit_type);
        if !result.status().is_success() {
            if result.status().as_u16() == 429 {
                user.limits
                    .as_mut()
                    .unwrap()
                    .get_mut(&self.limit_type)
                    .unwrap()
                    .remaining = 0;
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
        let limit_type = belongs_to.limits.as_ref().unwrap().get(limit_type).unwrap();
        if global.remaining == 0 || ip.remaining == 0 || limit_type.remaining == 0 {
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
            405 | 408 | 409 => ChorusError::RequestErrorError {
                url: response.url().to_string(),
                error: response.text().await.unwrap(),
            },
            411..=421 | 426 | 428 | 431 => ChorusError::InvalidArgumentsError {
                error: response.text().await.unwrap(),
            },
            429 => panic!("Illegal state: Rate limit exception should have been caught before this function call."),
            451 => ChorusError::NoResponse,
            500..=599 => ChorusError::ReceivedErrorCodeError { error_code: format!("{}", response.status().as_u16()) },
            _ => ChorusError::RequestErrorError { url: response.url().to_string(), error: response.text().await.unwrap() }
        }
    }

    fn update_rate_limits(user: &mut UserMeta, limit_type: &LimitType) {
        let mut belongs_to = user.belongs_to.borrow_mut();
        if belongs_to.limits.is_none() {
            return;
        }
        let instance_dictated_limits = [&LimitType::AuthLogin, &LimitType::AuthRegister];
        let user_dictated_limits = [
            &LimitType::Channel,
            &LimitType::Error,
            &LimitType::Guild,
            &LimitType::Webhook,
        ];

        if instance_dictated_limits.contains(&limit_type) {
            belongs_to
                .limits
                .as_mut()
                .unwrap()
                .get_mut(limit_type)
                .unwrap()
                .remaining -= 1;
        } else {
            user.limits
                .as_mut()
                .unwrap()
                .get_mut(limit_type)
                .unwrap()
                .remaining -= 1;
        }
        belongs_to
            .limits
            .as_mut()
            .unwrap()
            .get_mut(&LimitType::Global)
            .unwrap()
            .remaining -= 1;
        belongs_to
            .limits
            .as_mut()
            .unwrap()
            .get_mut(&LimitType::Ip)
            .unwrap()
            .remaining -= 1;
    }

    pub async fn get_rate_limits(url_api: &str) -> ChorusResult<Option<HashMap<LimitType, Limit>>> {
        let request = Client::new()
            .get(format!("{}/policies/instance/limits/", url_api))
            .send()
            .await;
        let request = match request {
            Ok(request) => request,
            Err(e) => {
                return Err(ChorusError::RequestErrorError {
                    url: url_api.to_string(),
                    error: e.to_string(),
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
            404 => return Err(ChorusError::RequestErrorError { url: url_api.to_string(), error: format!("Route \"/policies/instance/limits/\" not found. Are you perhaps trying to request the Limits configuration from an unsupported server?") }),
            400..=u16::MAX => {
                return Err(ChorusError::RequestErrorError {
                    url: url_api.to_string(),
                    error: request.text().await.unwrap(),
                })
            }
            _ => {
                return Err(ChorusError::InvalidResponseError {
                    error: request.text().await.unwrap(),
                })
            }
        };

        Ok(ChorusRequest::limits_config_to_hashmap(
            limits_configuration,
        ))
    }

    fn limits_config_to_hashmap(
        limits_configuration: LimitsConfiguration,
    ) -> Option<HashMap<LimitType, Limit>> {
        if !limits_configuration.rate.enabled {
            return None;
        }
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
        Some(map)
    }
}
