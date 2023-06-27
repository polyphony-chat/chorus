use std::fmt::format;

use reqwest::{Client, RequestBuilder, Response};
use serde_json::from_str;

use crate::{
    api::limits::{Limit, LimitType, Ratelimits},
    errors::{ChorusLibError, ChorusResult},
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
            return Err(ChorusLibError::RateLimited {
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
                return Err(ChorusLibError::ReceivedErrorCodeError {
                    error_code: e.to_string(),
                })
            }
        };
        drop(belongs_to);
        ChorusRequest::update_rate_limits(user, &self.limit_type);
        if !result.status().is_success() {
            if result.status().as_u16() == 429 {
                user.limits.get_mut(self.limit_type.clone()).remaining = 0;
            }
            return Err(ChorusRequest::interpret_error(result).await);
        }
        Ok(result)
    }

    fn can_send_request(user: &UserMeta, limit_type: &LimitType) -> bool {
        !user.limits.is_exhausted(limit_type)
            && !user
                .belongs_to
                .borrow()
                .limits
                .unwrap()
                .is_exhausted(limit_type)
    }

    async fn interpret_error(response: reqwest::Response) -> ChorusLibError {
        match response.status().as_u16() {
            200..=299 => ChorusLibError::InvalidArgumentsError {
                error: "You somehow passed a successful request into this function, which is not allowed."
                    .to_string(),
            },
            401..=403 | 407 => ChorusLibError::NoPermission,
            404 => ChorusLibError::NotFound {
                error: response.text().await.unwrap(),
            },
            405 | 408 | 409 => ChorusLibError::RequestErrorError {
                url: response.url().to_string(),
                error: response.text().await.unwrap(),
            },
            411..=421 | 426 | 428 | 431 => ChorusLibError::InvalidArgumentsError {
                error: response.text().await.unwrap(),
            },
            429 => panic!("Illegal state: Rate limit exception should have been caught before this function call."),
            451 => ChorusLibError::NoResponse,
            500..=599 => ChorusLibError::ReceivedErrorCodeError { error_code: format!("{}", response.status().as_u16()) },
            _ => ChorusLibError::RequestErrorError { url: response.url().to_string(), error: response.text().await.unwrap() }
        }
    }

    fn update_rate_limits(user: &mut UserMeta, limit_type: &LimitType) {}

    pub async fn check_rate_limits(url_api: &str) -> ChorusResult<Option<Ratelimits>> {
        let request = Client::new()
            .get(format!("{}/policies/instance/limits/", url_api))
            .send()
            .await;
        let request = match request {
            Ok(request) => request,
            Err(e) => {
                return Err(ChorusLibError::RequestErrorError {
                    url: url_api.to_string(),
                    error: e.to_string(),
                })
            }
        };
        let limits_configuration = match request.status().as_u16() {
            200 => from_str::<LimitsConfiguration>(&request.text().await.unwrap()).unwrap(),
            429 => {
                return Err(ChorusLibError::RateLimited {
                    bucket: format!("{:?}", LimitType::Ip),
                })
            }
            404 => return Err(ChorusLibError::RequestErrorError { url: url_api.to_string(), error: format!("Route \"/policies/instance/limits/\" not found. Are you perhaps trying to request the Limits configuration from an unsupported server?") }),
            400..=u16::MAX => {
                return Err(ChorusLibError::RequestErrorError {
                    url: url_api.to_string(),
                    error: request.text().await.unwrap(),
                })
            }
            _ => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: request.text().await.unwrap(),
                })
            }
        };

        Ok(ChorusRequest::limits_config_to_ratelimits(
            limits_configuration,
        ))
    }

    fn limits_config_to_ratelimits(
        limits_configuration: LimitsConfiguration,
    ) -> Option<Ratelimits> {
        if !limits_configuration.rate.enabled {
            return None;
        }
        let config = limits_configuration.rate;
        let routes = config.routes;
        Some(Ratelimits {
            auth_register: Limit {
                bucket: LimitType::AuthRegister,
                limit: routes.auth.register.count,
                remaining: routes.auth.register.count,
                reset: routes.auth.register.window,
            },
            auth_login: Limit {
                bucket: LimitType::AuthLogin,
                limit: routes.auth.login.count,
                remaining: routes.auth.login.count,
                reset: routes.auth.login.window,
            },
            global: Limit {
                bucket: LimitType::Global,
                limit: config.global.count,
                remaining: config.global.count - 1, // We have used 1 request to get this info
                reset: config.global.window,
            },
            ip: Limit {
                bucket: LimitType::Ip,
                limit: config.ip.count,
                remaining: config.ip.count - 1, // Same here
                reset: config.ip.window,
            },
            channel: Limit {
                bucket: LimitType::Channel,
                limit: routes.channel.count,
                remaining: routes.channel.count,
                reset: routes.channel.window,
            },
            error: Limit {
                bucket: LimitType::Error,
                limit: config.error.count,
                remaining: config.error.count,
                reset: config.error.window,
            },
            guild: Limit {
                bucket: LimitType::Guild,
                limit: routes.guild.count,
                remaining: routes.guild.count,
                reset: routes.guild.window,
            },
            webhook: Limit {
                bucket: LimitType::Webhook,
                limit: routes.webhook.count,
                remaining: routes.webhook.count,
                reset: routes.webhook.window,
            },
        })
    }
}
