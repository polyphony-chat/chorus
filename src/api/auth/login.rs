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
use crate::types::{GatewayIdentifyPayload, LimitType, LoginResult, LoginSchema, User};

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
        let mut user =
            ChorusUser::shell(Arc::new(RwLock::new(self.clone())), "None".to_string()).await;
        
        let login_result = chorus_request
            .deserialize_response::<LoginResult>(&mut user)
            .await?;
        user.set_token(login_result.token);
        user.settings = login_result.settings;

        let object = User::get(&mut user, None).await?;
        *user.object.write().unwrap() = object;

        let mut identify = GatewayIdentifyPayload::common();
        identify.token = user.token();
        user.gateway.send_identify(identify).await;

        Ok(user)
    }
}
