// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::gateway::Gateway;
use crate::instance::{ChorusUser, Instance};
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    AuthenticatorType, GatewayIdentifyPayload, LimitType, LoginResult, LoginSchema,
    SendMfaSmsResponse, SendMfaSmsSchema, User, VerifyMFALoginResponse, VerifyMFALoginSchema,
};

impl Instance {
    /// Logs into an existing account on the spacebar server.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#post-/auth/login/>
    pub async fn login_account(&mut self, login_schema: LoginSchema) -> ChorusResult<ChorusUser> {
        let endpoint_url = self.urls.api.clone() + "/auth/login";
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .body(to_string(&login_schema).unwrap())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::AuthLogin,
        };

        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since login is an instance wide limit), which is why we are just cloning the
        // instances' limits to pass them on as user_rate_limits later.
        let mut user = ChorusUser::shell(Arc::new(RwLock::new(self.clone())), "None").await;

        let login_result = chorus_request
            .deserialize_response::<LoginResult>(&mut user)
            .await?;
        user.set_token(&login_result.token);
        user.settings = login_result.settings;

        let object = User::get_current(&mut user).await?;
        user.object = Some(Arc::new(RwLock::new(object)));

        let mut identify = GatewayIdentifyPayload::common();
        identify.token = user.token();
        user.gateway.send_identify(identify).await;

        Ok(user)
    }

    /// Verifies a multi-factor authentication login
    ///
    /// # Reference
    /// See <https://docs.discord.sex/authentication#verify-mfa-login>
    pub async fn verify_mfa_login(
        &mut self,
        authenticator: AuthenticatorType,
        schema: VerifyMFALoginSchema,
    ) -> ChorusResult<ChorusUser> {
        let endpoint_url = self.urls.api.clone() + "/auth/mfa/" + &authenticator.to_string();

        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .header("Content-Type", "application/json")
                .json(&schema),
            limit_type: LimitType::AuthLogin,
        };

        let mut user = ChorusUser::shell(Arc::new(RwLock::new(self.clone())), "None").await;

        match chorus_request
            .deserialize_response::<VerifyMFALoginResponse>(&mut user)
            .await?
        {
            VerifyMFALoginResponse::Success {
                token,
                user_settings,
            } => {
                user.set_token(&token);
                user.settings = user_settings;
            }
            VerifyMFALoginResponse::UserSuspended {
                suspended_user_token,
            } => {
                return Err(crate::errors::ChorusError::SuspendUser {
                    token: suspended_user_token,
                })
            }
        }

        let object = User::get_current(&mut user).await?;
        user.object = Some(Arc::new(RwLock::new(object)));

        let mut identify = GatewayIdentifyPayload::common();
        identify.token = user.token();
        user.gateway.send_identify(identify).await;

        Ok(user)
    }

    /// Sends a multi-factor authentication code to the user's phone number
    ///
    /// # Reference
    /// See <https://docs.discord.sex/authentication#send-mfa-sms>
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

        let mut chorus_user =
            ChorusUser::shell(Arc::new(RwLock::new(self.clone())), "None").await;

        let send_mfa_sms_response = chorus_request
            .deserialize_response::<SendMfaSmsResponse>(&mut chorus_user)
            .await?;

        Ok(send_mfa_sms_response)
    }
}
