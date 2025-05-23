// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Ratelimiter and request handling functionality.

use std::collections::HashMap;

use log::{trace, warn};
use reqwest::{Client, RequestBuilder, Response};
use serde::Deserialize;
use serde_json::from_str;

use crate::{
    errors::{ApiError, ChorusError, ChorusResult, JsonError},
    instance::{ChorusUser, Instance},
    types::{
        types::subconfigs::limits::rates::RateLimits, Limit, LimitType, LimitsConfiguration,
        MfaRequiredSchema,
    },
};

/// Chorus' request struct. This struct is used to send rate-limited requests to the Spacebar server.
/// See <https://discord.com/developers/docs/topics/rate-limits#rate-limits> for more information.
#[derive(Debug)]
pub struct ChorusRequest {
    pub request: RequestBuilder,
    pub limit_type: LimitType,
}

impl ChorusRequest {
    /// Sends a [`ChorusRequest`]. Checks if the user / instance is rate limited, and if not, sends the request.
    ///
    /// If the user is not rate limited and the instance has rate limits enabled, it will update the
    /// rate limits.
    #[allow(clippy::await_holding_refcell_ref)]
    pub(crate) async fn send(self, user: &mut ChorusUser) -> ChorusResult<Response> {
        // Have one arc just for this method, so we don't need to borrow it from ChorusUser
        let instance_arc = user.belongs_to.clone();

        if !ChorusRequest::can_send_request(
            &mut Some(user),
            // Note: create a lock here, which will be released before we get to async code
            &mut instance_arc.write().unwrap(),
            &self.limit_type,
        ) {
            log::info!("Rate limit hit. Bucket: {:?}", self.limit_type);
            return Err(ChorusError::RateLimited {
                bucket: format!("{:?}", self.limit_type),
            });
        }

        let mut request = self.request;

        request = request.header("User-Agent", user.client_properties.user_agent.clone().0);

        let client = user.belongs_to.read().unwrap().client.clone();
        let result = match client.execute(request.build().unwrap()).await {
            Ok(result) => {
                log::trace!("Request successful: {:?}", result);
                result
            }
            Err(error) => {
                log::trace!("Request failed to send: {:?}", error);
                log::warn!("Request failed: {}", error);

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
                instance_arc
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

            let request_url = result.url().clone();

            log::trace!("Request failed, result: {:?}", result);

            let error = ChorusRequest::interpret_error(result).await;
            log::warn!("Request failed: {} | {}", request_url.path(), error);

            return Err(error);
        }

        ChorusRequest::update_rate_limits(
            Some(user),
            // Now we are past the async code and can safely create another lock
            &mut instance_arc.write().unwrap(),
            &self.limit_type,
            !result.status().is_success(),
        );
        Ok(result)
    }

    /// Sends a [`ChorusRequest`] without a [ChorusUser].
    ///
    /// Checks if the instance is rate limited, and if not, sends the request.
    ///
    /// If the instance has rate limits enabled, they will be updated.
    ///
    /// Note: anonymous requests shouldn't count towards user buckets!
    #[allow(clippy::await_holding_refcell_ref)]
    pub(crate) async fn send_anonymous(self, instance: &mut Instance) -> ChorusResult<Response> {
        if !ChorusRequest::can_send_request(&mut None, instance, &self.limit_type) {
            log::info!("Rate limit hit. Bucket: {:?}", self.limit_type);
            return Err(ChorusError::RateLimited {
                bucket: format!("{:?}", self.limit_type),
            });
        }

        let request = self.request;

        // TODO: maybe have a default Instance user agent?

        let client = instance.client.clone();
        let result = match client.execute(request.build().unwrap()).await {
            Ok(result) => {
                log::trace!("Request successful: {:?}", result);
                result
            }
            Err(error) => {
                log::trace!("Request failed to send: {:?}", error);
                log::warn!("Request failed: {}", error);

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
                instance
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

            let request_url = result.url().clone();

            log::trace!("Request failed, result: {:?}", result);

            let error = ChorusRequest::interpret_error(result).await;
            log::warn!("Request failed: {} | {}", request_url.path(), error);

            return Err(error);
        }
        ChorusRequest::update_rate_limits(
            None,
            instance,
            &self.limit_type,
            !result.status().is_success(),
        );
        Ok(result)
    }

    /// Checks if we've hit our ratelimit buckets before sending a request.
    ///
    /// Returns false if we're trying to send an anonymous request with a User bucket.
    fn can_send_request(
        user: &mut Option<&mut ChorusUser>,
        instance: &mut Instance,
        limit_type: &LimitType,
    ) -> bool {
        log::trace!("Checking if user or instance is rate-limited...");

        if instance.limits_information.is_none() {
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
                instance
                    .limits_information
                    .as_mut()
                    .unwrap()
                    .ratelimits
                    .clone()
            }
            false => {
                if user.is_none() {
                    log::error!(
                    "Limit type {:?} for anonymous request is dictated by the user. Cannot continue.",
                    limit_type
                );
                    return false;
                }

                let user = user.as_mut().unwrap();

                log::trace!(
                    "Limit type {:?} is dictated by the user. Continuing.",
                    limit_type
                );
                ChorusRequest::ensure_limit_in_map(
                    &instance.limits_information.as_ref().unwrap().configuration,
                    user.limits.as_mut().unwrap(),
                    limit_type,
                );
                user.limits.as_mut().unwrap().clone()
            }
        };
        let global = instance
            .limits_information
            .as_ref()
            .unwrap()
            .ratelimits
            .get(&LimitType::Global)
            .unwrap();
        let ip = instance
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

    /// Interprets an unsuccessful [reqwest::Response] type as a [ChorusError]
    async fn interpret_error(response: reqwest::Response) -> ChorusError {
        let http_status = response.status();

        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                    http_status,
                }
            }
        };

        let json_error = match serde_json::from_str::<JsonError>(&response_text) {
            Ok(json_error) => json_error,
            Err(e) => {
                return ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to deserialize the JSON response into a JsonError: {}. JSON Response: {}",
                        e, response_text
                    ),
                    http_status
                };
            }
        };

        match json_error.code {
            // MFA required code
            60003 => match serde_json::from_str::<MfaRequiredSchema>(&response_text) {
                Ok(response) => ChorusError::MfaRequired { error: response },
                Err(e) => {
                    warn!(
                        "Received MFA required error, but could not deserialize challenge: {}.",
                        e
                    );
                    trace!("JSON Response: {}", response_text);
                    ChorusError::ReceivedError {
                        error: ApiError {
                            json_error,
                            http_status,
                        },
                        response_text,
                    }
                }
            },
            // This means the code has no further information and we should look at the status
            0 => match http_status.as_u16() {
                401..=403 | 407 => ChorusError::NoPermission,
                404 => ChorusError::NotFound {
                    error: json_error
                        .message
                        .unwrap_or(String::from("Resource not found")),
                },
                429 => {
                    panic!("Illegal state: Rate limit exception should have been caught before this function call.")
                }
                451 => ChorusError::NoResponse,
                _ => ChorusError::ReceivedError {
                    error: ApiError {
                        json_error,
                        http_status,
                    },
                    response_text,
                },
            },
            _ => ChorusError::ReceivedError {
                error: ApiError {
                    json_error,
                    http_status,
                },
                response_text,
            },
        }
    }

    /// Updates the rate limits of the user / instance. The following steps are performed:
    ///
    /// 1. If the current unix timestamp is greater than the reset timestamp, the reset timestamp is
    ///    set to the current unix timestamp + the rate limit window. The remaining rate limit is
    ///    reset to the rate limit limit.
    ///
    /// 2. The remaining rate limit is decreased by 1.
    ///
    /// If user is [Some], the request will be treated as a user request.
    /// If user is [None], it will be treated as an anonymous request.
    fn update_rate_limits(
        mut user: Option<&mut ChorusUser>,
        instance: &mut Instance,
        limit_type: &LimitType,
        response_was_err: bool,
    ) {
        if instance.limits_information.is_none() {
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
        if response_was_err && user.is_some() {
            relevant_limits.push((LimitOrigin::User, LimitType::Error));
        }
        let time: u64 = chrono::Utc::now().timestamp() as u64;
        for relevant_limit in relevant_limits.iter() {
            let limit = match relevant_limit.0 {
                LimitOrigin::Instance => {
                    log::trace!(
                        "Updating instance rate limit. Bucket: {:?}",
                        relevant_limit.1
                    );
                    instance
                        .limits_information
                        .as_mut()
                        .unwrap()
                        .ratelimits
                        .get_mut(&relevant_limit.1)
                        .unwrap()
                }
                LimitOrigin::User => {
                    if user.is_none() {
                        warn!(
                            "Anonymous request was part of User bucket ({:?})!",
                            relevant_limit.1
                        );
                        warn!("This means our ratelimit count is now out of sync.");
                        return;
                    }

                    log::trace!("Updating user rate limit. Bucket: {:?}", relevant_limit.1);
                    user.as_mut()
                        .unwrap()
                        .limits
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
            _ => {
                return Err(ChorusRequest::interpret_error(request).await)
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
    pub(crate) async fn send_and_handle_as_result(self, user: &mut ChorusUser) -> ChorusResult<()> {
        match self.send(user).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[allow(unused)]
    /// Sends an anonymous [`ChorusRequest`] and returns a [`ChorusResult`] that contains nothing if the request
    /// was successful, or a [`ChorusError`] if the request failed.
    ///
    /// Note: these kinds of requests cannot properly count towards User ratelimit buckets!
    ///
    /// However, this method is still preferable to writing or copy pasting code, which does not
    /// count any ratelimit
    pub(crate) async fn send_anonymous_and_handle_as_result(
        self,
        instance: &mut Instance,
    ) -> ChorusResult<()> {
        match self.send_anonymous(instance).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Sends a [`ChorusRequest`] and calls [Self::deserialize_response].
    ///
    /// Returns a [`ChorusResult`] that contains a [`T`] if the request
    /// was successful, or a [`ChorusError`] if the request failed.
    pub(crate) async fn send_and_deserialize_response<T: for<'a> Deserialize<'a>>(
        self,
        user: &mut ChorusUser,
    ) -> ChorusResult<T> {
        let response = self.send(user).await?;
        log::trace!("Got response: {:?}", response);

        Self::deserialize_response(response).await
    }

    /// Sends an anonymous [`ChorusRequest`] and calls [Self::deserialize_response].
    ///
    /// Note: these kinds of requests cannot properly count towards User ratelimit buckets!
    ///
    /// However, this method is still preferable to writing or copy pasting code, which does not
    /// count any ratelimit
    ///
    /// Returns a [`ChorusResult`] that contains a [`T`] if the request
    /// was successful, or a [`ChorusError`] if the request failed.
    pub(crate) async fn send_anonymous_and_deserialize_response<T: for<'a> Deserialize<'a>>(
        self,
        instance: &mut Instance,
    ) -> ChorusResult<T> {
        let response = self.send_anonymous(instance).await?;
        log::trace!("Got response: {:?}", response);

        Self::deserialize_response(response).await
    }

    /// Processes a [reqwest::Result] (acquired by [Self::send_request] or [Self::send_anonymous_request]), expecting a [`T`] in JSON
    ///
    /// Returns a [`ChorusResult`] that contains a [`T`] if the request
    /// was successful, or a [`ChorusError`] if the request failed.
    pub(crate) async fn deserialize_response<T: for<'a> Deserialize<'a>>(
        response: reqwest::Response,
    ) -> ChorusResult<T> {
        let http_status = response.status();

        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                    http_status,
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
						  http_status
                })
            }
        };
        Ok(object)
    }

    /// Adds an audit log reason to the request.
    ///
    /// Sets the X-Audit-Log-Reason header
    pub(crate) fn with_audit_log_reason(self, reason: String) -> ChorusRequest {
        let mut request = self;

        request.request = request.request.header("X-Audit-Log-Reason", reason);
        request
    }

    /// Adds an audit log reason to the request, if it is [Some]
    ///
    /// Sets the X-Audit-Log-Reason header
    pub(crate) fn with_maybe_audit_log_reason(self, reason: Option<String>) -> ChorusRequest {
        if let Some(reason_some) = reason {
            return self.with_audit_log_reason(reason_some);
        }

        self
    }

    /// Adds an authorization token to the request.
    ///
    /// Sets the Authorization header
    pub(crate) fn with_authorization(self, token: &String) -> ChorusRequest {
        let mut request = self;

        request.request = request.request.header("Authorization", token);
        request
    }

    /// Adds authorization for a [ChorusUser] to the request.
    ///
    /// Sets the Authorization header
    pub(crate) fn with_authorization_for(self, user: &ChorusUser) -> ChorusRequest {
        self.with_authorization(&user.token)
    }

    /// Adds user-specific headers for a [ChorusUser] to the request.
    ///
    /// Adds authorization and telemetry; for specific details see
    /// [Self::with_authorization_for] and [Self::with_client_properties_for]
    ///
    /// If a route you're adding involves authorization as the user, you
    /// should likely use this method.
    pub(crate) fn with_headers_for(self, user: &ChorusUser) -> ChorusRequest {
        self.with_authorization_for(user)
            .with_client_properties_for(user)
    }
}

enum LimitOrigin {
    Instance,
    User,
}
