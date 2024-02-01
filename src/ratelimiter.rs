// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Ratelimiter and request handling functionality.

use std::collections::HashMap;

use log::{self, debug};
use reqwest::{Client, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::from_str;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    types::{types::subconfigs::limits::rates::RateLimits, Limit, LimitType, LimitsConfiguration},
};

/// Chorus' request struct. This struct is used to send rate-limited requests to the Spacebar server.
/// See <https://discord.com/developers/docs/topics/rate-limits#rate-limits> for more information.
#[derive(Debug)]
pub struct ChorusRequest {
    pub request: RequestBuilder,
    pub limit_type: LimitType,
}

impl ChorusRequest {
    /// Makes a new [`ChorusRequest`].
    /// # Arguments
    /// * `method` - The HTTP method to use. Must be one of the following:
    ///     * [`http::Method::GET`]
    ///     * [`http::Method::POST`]
    ///     * [`http::Method::PUT`]
    ///     * [`http::Method::DELETE`]
    ///     * [`http::Method::PATCH`]
    ///     * [`http::Method::HEAD`]
    #[allow(unused_variables)] // TODO: Add mfa_token to request, once we figure out *how* to do so correctly
    pub fn new(
        method: http::Method,
        url: &str,
        body: Option<String>,
        audit_log_reason: Option<&str>,
        mfa_token: Option<&str>,
        chorus_user: Option<&mut ChorusUser>,
        limit_type: LimitType,
    ) -> ChorusRequest {
        let request = Client::new();
        let mut request = match method {
            http::Method::GET => request.get(url),
            http::Method::POST => request.post(url),
            http::Method::PUT => request.put(url),
            http::Method::DELETE => request.delete(url),
            http::Method::PATCH => request.patch(url),
            http::Method::HEAD => request.head(url),
            _ => panic!("Illegal state: Method not supported."),
        };
        if let Some(user) = chorus_user {
            request = request.header("Authorization", user.token());
        }
        if let Some(body) = body {
            // ONCE TOLD ME THE WORLD WAS GONNA ROLL ME
            request = request
                .body(body)
                .header("Content-Type", "application/json");
        }
        if let Some(reason) = audit_log_reason {
            request = request.header("X-Audit-Log-Reason", reason);
        }

        ChorusRequest {
            request,
            limit_type,
        }
    }

    /// Sends a [`ChorusRequest`]. Checks if the user is rate limited, and if not, sends the request.
    /// If the user is not rate limited and the instance has rate limits enabled, it will update the
    /// rate limits.
    #[allow(clippy::await_holding_refcell_ref)]
    pub(crate) async fn send_request(self, user: &mut ChorusUser) -> ChorusResult<Response> {
        if !ChorusRequest::can_send_request(user, &self.limit_type) {
            log::info!("Rate limit hit. Bucket: {:?}", self.limit_type);
            return Err(ChorusError::RateLimited {
                bucket: format!("{:?}", self.limit_type),
            });
        }
        let client = user.belongs_to.read().unwrap().client.clone();
        let result = match client.execute(self.request.build().unwrap()).await {
            Ok(result) => {
                debug!("Request successful: {:?}", result);
                result
            }
            Err(error) => {
                log::warn!("Request failed: {:?}", error);
                return Err(ChorusError::RequestFailed {
                    url: error.url().unwrap().to_string(),
                    error: error.to_string(),
                });
            }
        };
        drop(client);
        if !result.status().is_success() {
            if result.status().as_u16() == 429 {
                log::warn!("Rate limit hit unexpectedly. Bucket: {:?}. Setting the instances' remaining global limit to 0 to have cooldown.", self.limit_type);
                user.belongs_to
                    .write()
                    .unwrap()
                    .limits_information
                    .as_mut()
                    .unwrap()
                    .ratelimits
                    .get_mut(&LimitType::Global)
                    .unwrap()
                    .remaining = 0;
                return Err(ChorusError::RateLimited {
                    bucket: format!("{:?}", self.limit_type),
                });
            }
            log::warn!("Request failed: {:?}", result);
            return Err(ChorusRequest::interpret_error(result).await);
        }
        ChorusRequest::update_rate_limits(user, &self.limit_type, !result.status().is_success());
        Ok(result)
    }

    fn can_send_request(user: &mut ChorusUser, limit_type: &LimitType) -> bool {
        log::trace!("Checking if user or instance is rate-limited...");
        let mut belongs_to = user.belongs_to.write().unwrap();
        if belongs_to.limits_information.is_none() {
            log::trace!("Instance indicates no rate limits are configured. Continuing.");
            return true;
        }
        let instance_dictated_limits = [
            &LimitType::AuthLogin,
            &LimitType::AuthRegister,
            &LimitType::Global,
            &LimitType::Ip,
        ];
        let limits = match instance_dictated_limits.contains(&limit_type) {
            true => {
                log::trace!(
                    "Limit type {:?} is dictated by the instance. Continuing.",
                    limit_type
                );
                belongs_to
                    .limits_information
                    .as_mut()
                    .unwrap()
                    .ratelimits
                    .clone()
            }
            false => {
                log::trace!(
                    "Limit type {:?} is dictated by the user. Continuing.",
                    limit_type
                );
                ChorusRequest::ensure_limit_in_map(
                    &belongs_to
                        .limits_information
                        .as_ref()
                        .unwrap()
                        .configuration,
                    user.limits.as_mut().unwrap(),
                    limit_type,
                );
                user.limits.as_mut().unwrap().clone()
            }
        };
        let global = belongs_to
            .limits_information
            .as_ref()
            .unwrap()
            .ratelimits
            .get(&LimitType::Global)
            .unwrap();
        let ip = belongs_to
            .limits_information
            .as_ref()
            .unwrap()
            .ratelimits
            .get(&LimitType::Ip)
            .unwrap();
        let limit_type_limit = limits.get(limit_type).unwrap();
        global.remaining > 0 && ip.remaining > 0 && limit_type_limit.remaining > 0
    }

    fn ensure_limit_in_map(
        rate_limits_config: &RateLimits,
        map: &mut HashMap<LimitType, Limit>,
        limit_type: &LimitType,
    ) {
        log::trace!("Ensuring limit type {:?} is in the map.", limit_type);
        let time: u64 = chrono::Utc::now().timestamp() as u64;
        match limit_type {
            LimitType::Channel(snowflake) => {
                if map.get(&LimitType::Channel(*snowflake)).is_some() {
                    log::trace!(
                        "Limit type {:?} is already in the map. Returning.",
                        limit_type
                    );
                    return;
                }
                log::trace!("Limit type {:?} is not in the map. Adding it.", limit_type);
                let channel_limit = &rate_limits_config.routes.channel;
                map.insert(
                    LimitType::Channel(*snowflake),
                    Limit {
                        bucket: LimitType::Channel(*snowflake),
                        limit: channel_limit.count,
                        remaining: channel_limit.count,
                        reset: channel_limit.window + time,
                        window: channel_limit.window,
                    },
                );
            }
            LimitType::Guild(snowflake) => {
                if map.get(&LimitType::Guild(*snowflake)).is_some() {
                    return;
                }
                let guild_limit = &rate_limits_config.routes.guild;
                map.insert(
                    LimitType::Guild(*snowflake),
                    Limit {
                        bucket: LimitType::Guild(*snowflake),
                        limit: guild_limit.count,
                        remaining: guild_limit.count,
                        reset: guild_limit.window + time,
                        window: guild_limit.window,
                    },
                );
            }
            LimitType::Webhook(snowflake) => {
                if map.get(&LimitType::Webhook(*snowflake)).is_some() {
                    return;
                }
                let webhook_limit = &rate_limits_config.routes.webhook;
                map.insert(
                    LimitType::Webhook(*snowflake),
                    Limit {
                        bucket: LimitType::Webhook(*snowflake),
                        limit: webhook_limit.count,
                        remaining: webhook_limit.count,
                        reset: webhook_limit.window + time,
                        window: webhook_limit.window,
                    },
                );
            }
            other_limit => {
                if map.get(other_limit).is_some() {
                    return;
                }
                let limits_map = ChorusRequest::limits_config_to_hashmap(rate_limits_config);
                map.insert(
                    *other_limit,
                    Limit {
                        bucket: *other_limit,
                        limit: limits_map.get(other_limit).as_ref().unwrap().limit,
                        remaining: limits_map.get(other_limit).as_ref().unwrap().remaining,
                        reset: limits_map.get(other_limit).as_ref().unwrap().reset,
                        window: limits_map.get(other_limit).as_ref().unwrap().window,
                    },
                );
            }
        }
    }

    async fn interpret_error(response: reqwest::Response) -> ChorusError {
        match response.status().as_u16() {
            401..=403 | 407 => ChorusError::NoPermission,
            404 => ChorusError::NotFound {
                error: response.text().await.unwrap(),
            },
            405 | 408 | 409 => ChorusError::ReceivedErrorCode { error_code: response.status().as_u16(), error: response.text().await.unwrap() },
            411..=421 | 426 | 428 | 431 => ChorusError::InvalidArguments {
                error: response.text().await.unwrap(),
            },
            429 => panic!("Illegal state: Rate limit exception should have been caught before this function call."),
            451 => ChorusError::NoResponse,
            500..=599 => ChorusError::ReceivedErrorCode { error_code: response.status().as_u16(), error: response.text().await.unwrap() },
            _ => ChorusError::ReceivedErrorCode { error_code: response.status().as_u16(), error: response.text().await.unwrap()},
        }
    }

    /// Updates the rate limits of the user. The following steps are performed:
    /// 1.  If the current unix timestamp is greater than the reset timestamp, the reset timestamp is
    ///     set to the current unix timestamp + the rate limit window. The remaining rate limit is
    ///     reset to the rate limit limit.
    /// 2. The remaining rate limit is decreased by 1.
    fn update_rate_limits(user: &mut ChorusUser, limit_type: &LimitType, response_was_err: bool) {
        if user.belongs_to.read().unwrap().limits_information.is_none() {
            return;
        }
        let instance_dictated_limits = [
            &LimitType::AuthLogin,
            &LimitType::AuthRegister,
            &LimitType::Global,
            &LimitType::Ip,
        ];
        // modify this to store something to look up the value with later, instead of storing a reference to the actual data itself.
        let mut relevant_limits = Vec::new();
        if instance_dictated_limits.contains(&limit_type) {
            relevant_limits.push((LimitOrigin::Instance, *limit_type));
        } else {
            relevant_limits.push((LimitOrigin::User, *limit_type));
        }
        relevant_limits.push((LimitOrigin::Instance, LimitType::Global));
        relevant_limits.push((LimitOrigin::Instance, LimitType::Ip));
        if response_was_err {
            relevant_limits.push((LimitOrigin::User, LimitType::Error));
        }
        let time: u64 = chrono::Utc::now().timestamp() as u64;
        for relevant_limit in relevant_limits.iter() {
            let mut belongs_to = user.belongs_to.write().unwrap();
            let limit = match relevant_limit.0 {
                LimitOrigin::Instance => {
                    log::trace!(
                        "Updating instance rate limit. Bucket: {:?}",
                        relevant_limit.1
                    );
                    belongs_to
                        .limits_information
                        .as_mut()
                        .unwrap()
                        .ratelimits
                        .get_mut(&relevant_limit.1)
                        .unwrap()
                }
                LimitOrigin::User => {
                    log::trace!("Updating user rate limit. Bucket: {:?}", relevant_limit.1);
                    user.limits
                        .as_mut()
                        .unwrap()
                        .get_mut(&relevant_limit.1)
                        .unwrap()
                }
            };
            if time > limit.reset {
                // Spacebar does not yet return rate limit information in its response headers. We
                // therefore have to guess the next rate limit window. This is not ideal. Oh well!
                log::trace!("Rate limit replenished. Bucket: {:?}", limit.bucket);
                limit.reset += limit.window;
                limit.remaining = limit.limit;
            }
            limit.remaining -= 1;
        }
    }

    /// Gets the ratelimit configuration.
    ///
    /// # Notes
    /// This is a spacebar only endpoint.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#get-/policies/instance/limits/>
    pub async fn get_limits_config(url_api: &str) -> ChorusResult<LimitsConfiguration> {
        let request = Client::new()
            .get(format!("{}/policies/instance/limits/", url_api))
            .send()
            .await;
        let request = match request {
            Ok(request) => request,
            Err(e) => {
                return Err(ChorusError::RequestFailed {
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
            404 => return Err(ChorusError::NotFound { error: "Route \"/policies/instance/limits/\" not found. Are you perhaps trying to request the Limits configuration from an unsupported server?".to_string() }),
            400..=u16::MAX => {
                return Err(ChorusError::ReceivedErrorCode { error_code: request.status().as_u16(), error: request.text().await.unwrap() })
            }
            _ => {
                return Err(ChorusError::InvalidResponse {
                    error: request.text().await.unwrap(),
                })
            }
        };

        Ok(limits_configuration)
    }

    pub(crate) fn limits_config_to_hashmap(
        limits_configuration: &RateLimits,
    ) -> HashMap<LimitType, Limit> {
        let config = limits_configuration.clone();
        let routes = config.routes;
        let mut map: HashMap<LimitType, Limit> = HashMap::new();
        let time: u64 = chrono::Utc::now().timestamp() as u64;
        map.insert(
            LimitType::AuthLogin,
            Limit {
                bucket: LimitType::AuthLogin,
                limit: routes.auth.login.count,
                remaining: routes.auth.login.count,
                reset: routes.auth.login.window + time,
                window: routes.auth.login.window,
            },
        );
        map.insert(
            LimitType::AuthRegister,
            Limit {
                bucket: LimitType::AuthRegister,
                limit: routes.auth.register.count,
                remaining: routes.auth.register.count,
                reset: routes.auth.register.window + time,
                window: routes.auth.register.window,
            },
        );
        map.insert(
            LimitType::ChannelBaseline,
            Limit {
                bucket: LimitType::ChannelBaseline,
                limit: routes.channel.count,
                remaining: routes.channel.count,
                reset: routes.channel.window + time,
                window: routes.channel.window,
            },
        );
        map.insert(
            LimitType::Error,
            Limit {
                bucket: LimitType::Error,
                limit: config.error.count,
                remaining: config.error.count,
                reset: config.error.window + time,
                window: config.error.window,
            },
        );
        map.insert(
            LimitType::Global,
            Limit {
                bucket: LimitType::Global,
                limit: config.global.count,
                remaining: config.global.count,
                reset: config.global.window + time,
                window: config.global.window,
            },
        );
        map.insert(
            LimitType::Ip,
            Limit {
                bucket: LimitType::Ip,
                limit: config.ip.count,
                remaining: config.ip.count,
                reset: config.ip.window + time,
                window: config.ip.window,
            },
        );
        map.insert(
            LimitType::GuildBaseline,
            Limit {
                bucket: LimitType::GuildBaseline,
                limit: routes.guild.count,
                remaining: routes.guild.count,
                reset: routes.guild.window + time,
                window: routes.guild.window,
            },
        );
        map.insert(
            LimitType::WebhookBaseline,
            Limit {
                bucket: LimitType::WebhookBaseline,
                limit: routes.webhook.count,
                remaining: routes.webhook.count,
                reset: routes.webhook.window + time,
                window: routes.webhook.window,
            },
        );
        map
    }

    /// Sends a [`ChorusRequest`] and returns a [`ChorusResult`] that contains nothing if the request
    /// was successful, or a [`ChorusError`] if the request failed.
    pub(crate) async fn handle_request_as_result(self, user: &mut ChorusUser) -> ChorusResult<()> {
        match self.send_request(user).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sends a [`ChorusRequest`] and returns a [`ChorusResult`] that contains a [`T`] if the request
    /// was successful, or a [`ChorusError`] if the request failed.
    pub(crate) async fn deserialize_response<T: for<'a> Deserialize<'a>>(
        self,
        user: &mut ChorusUser,
    ) -> ChorusResult<T> {
        let response = self.send_request(user).await?;
        debug!("Got response: {:?}", response);
        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
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
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to deserialize the JSON response into requested type T: {}. JSON Response: {}",
                        e, response_text
                    ),
                })
            }
        };
        Ok(object)
    }
}

enum LimitOrigin {
    Instance,
    User,
}
