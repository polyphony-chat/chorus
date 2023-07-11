use std::{cell::RefCell, rc::Rc};

use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::policies::instance::LimitType,
    errors::ChorusResult,
    instance::{Instance, Token, UserMeta},
    ratelimiter::ChorusRequest,
    types::RegisterSchema,
};

impl Instance {
    /// Registers a new user on the server.
    pub async fn register_account(
        &mut self,
        register_schema: &RegisterSchema,
    ) -> ChorusResult<UserMeta> {
        let endpoint_url = self.urls.api.clone() + "/auth/register";
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .body(to_string(register_schema).unwrap()),
            limit_type: LimitType::AuthRegister,
        };
        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since register is an instance wide limit), which is why we are just cloning
        // the instances' limits to pass them on as user_rate_limits later.
        let mut shell = UserMeta::shell(Rc::new(RefCell::new(self.clone())), "None".to_string());
        let token = chorus_request
            .deserialize_response::<Token>(&mut shell)
            .await?
            .token;
        if self.limits_information.is_some() {
            self.limits_information.as_mut().unwrap().ratelimits = shell.limits.unwrap();
        }
        let user_object = self.get_user(token.clone(), None).await.unwrap();
        let settings = UserMeta::get_settings(&token, &self.urls.api.clone(), self).await?;
        let user = UserMeta::new(
            Rc::new(RefCell::new(self.clone())),
            token.clone(),
            self.clone_limits_if_some(),
            settings,
            user_object,
        );
        Ok(user)
    }
}
