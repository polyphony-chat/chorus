use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::gateway::Gateway;
use crate::types::GatewayIdentifyPayload;
use crate::{
    api::policies::instance::LimitType,
    errors::ChorusResult,
    instance::{ChorusUser, Instance, Token},
    ratelimiter::ChorusRequest,
    types::RegisterSchema,
};

impl Instance {
    /// Registers a new user on the server.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#post-/auth/register/>
    pub async fn register_account(
        &mut self,
        register_schema: &RegisterSchema,
    ) -> ChorusResult<ChorusUser> {
        let endpoint_url = self.urls.api.clone() + "/auth/register";
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .body(to_string(register_schema).unwrap())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::AuthRegister,
        };
        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since register is an instance wide limit), which is why we are just cloning
        // the instances' limits to pass them on as user_rate_limits later.
        let mut shell =
            ChorusUser::shell(Arc::new(RwLock::new(self.clone())), "None".to_string()).await;
        let token = chorus_request
            .deserialize_response::<Token>(&mut shell)
            .await?
            .token;
        if self.limits_information.is_some() {
            self.limits_information.as_mut().unwrap().ratelimits = shell.limits.unwrap();
        }
        let user_object = self.get_user(token.clone(), None).await.unwrap();
        let settings = ChorusUser::get_settings(&token, &self.urls.api.clone(), self).await?;
        let mut identify = GatewayIdentifyPayload::common();
        let gateway = Gateway::new(self.urls.wss.clone()).await.unwrap();
        identify.token = token.clone();
        gateway.send_identify(identify).await;
        let user = ChorusUser::new(
            Arc::new(RwLock::new(self.clone())),
            token.clone(),
            self.clone_limits_if_some(),
            Arc::new(RwLock::new(settings)),
            Arc::new(RwLock::new(user_object)),
            Arc::new(gateway),
        );
        Ok(user)
    }
}
