// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::gateway::Gateway;
use crate::instance::{ChorusUser, Instance};
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    ClientProperties, GatewayIdentifyPayload, LimitType, LoginResult, LoginSchema,
    MfaAuthenticationType, SendMfaSmsResponse, SendMfaSmsSchema, Shared, User,
    VerifyMFALoginResponse, VerifyMFALoginSchema,
};

impl Instance {
    /// Logs into an existing account on the spacebar server.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#post-/auth/login/>
    pub async fn login_account(
        instance: Shared<Instance>,
        login_schema: LoginSchema,
    ) -> ChorusResult<ChorusUser> {
        let endpoint_url = instance.read().unwrap().urls.api.clone() + "/auth/login";
        let chorus_request = ChorusRequest {
            request: Client::new().post(endpoint_url).json(&login_schema),
            limit_type: LimitType::AuthLogin,
        }
        // Note: yes, this is still sent even for login and register
        .with_client_properties(&ClientProperties::default());

        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since login is an instance wide limit), which is why we are just cloning the
        // instances' limits to pass them on as user_rate_limits later.
        let mut user = ChorusUser::shell(instance, "None").await;

        let login_result = chorus_request
            .send_and_deserialize_response::<LoginResult>(&mut user)
            .await?;

        user.update_with_login_data(login_result.token, Some(login_result.settings))
            .await?;

        Ok(user)
    }

    /// Verifies a multi-factor authentication login
    ///
    /// # Reference
    /// See <https://docs.discord.food/authentication#verify-mfa-login>
    pub async fn verify_mfa_login(
        instance: Shared<Instance>,
        authenticator: MfaAuthenticationType,
        schema: VerifyMFALoginSchema,
    ) -> ChorusResult<ChorusUser> {
        let endpoint_url =
            instance.read().unwrap().urls.api.clone() + "/auth/mfa/" + &authenticator.to_string();

        let chorus_request = ChorusRequest {
            request: Client::new().post(endpoint_url).json(&schema),
            limit_type: LimitType::AuthLogin,
        }
        // Note: yes, this is still sent even for login and register
        .with_client_properties(&ClientProperties::default());

        let mut user = ChorusUser::shell(instance, "None").await;

        match chorus_request
            .send_and_deserialize_response::<VerifyMFALoginResponse>(&mut user)
            .await?
        {
            VerifyMFALoginResponse::Success {
                token,
                user_settings,
            } => {
                user.update_with_login_data(token, Some(user_settings))
                    .await?;
            }
            VerifyMFALoginResponse::UserSuspended {
                suspended_user_token,
            } => {
                return Err(crate::errors::ChorusError::SuspendUser {
                    token: suspended_user_token,
                })
            }
        }

        Ok(user)
    }

    /// Sends a multi-factor authentication code to the user's phone number
    ///
    /// # Reference
    /// See <https://docs.discord.food/authentication#send-mfa-sms>
    pub async fn send_mfa_sms(
        &mut self,
        schema: SendMfaSmsSchema,
    ) -> ChorusResult<SendMfaSmsResponse> {
        let endpoint_url = self.urls.api.clone() + "/auth/mfa/sms/send";
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .header("Content-Type", "application/json")
                .json(&schema),
            limit_type: LimitType::Ip,
        };

        let send_mfa_sms_response = chorus_request
            .send_anonymous_and_deserialize_response::<SendMfaSmsResponse>(self)
            .await?;

        Ok(send_mfa_sms_response)
    }
}
