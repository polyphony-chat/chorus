// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::gateway::{Gateway, GatewayHandle};
use crate::types::{GatewayIdentifyPayload, User};
use crate::{
    errors::ChorusResult,
    instance::{ChorusUser, Instance, Token},
    ratelimiter::ChorusRequest,
    types::LimitType,
    types::RegisterSchema,
};

impl Instance {
    /// Registers a new user on the server.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#post-/auth/register/>
    pub async fn register_account(
        &mut self,
        register_schema: RegisterSchema,
    ) -> ChorusResult<ChorusUser> {
        let endpoint_url = self.urls.api.clone() + "/auth/register";
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .body(to_string(&register_schema).unwrap())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::AuthRegister,
        };
        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since register is an instance wide limit), which is why we are just cloning
        // the instances' limits to pass them on as user_rate_limits later.
        let mut user = ChorusUser::shell(Arc::new(RwLock::new(self.clone())), "None").await;

        let token = chorus_request
            .deserialize_response::<Token>(&mut user)
            .await?
            .token;

        user.update_with_login_data(token, None).await?;

        Ok(user)
    }
}
