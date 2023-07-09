use std::cell::RefCell;
use std::rc::Rc;

use reqwest::Client;
use serde_json::to_string;

use crate::api::LimitType;
use crate::errors::ChorusResult;
use crate::instance::{Instance, UserMeta};
use crate::ratelimiter::ChorusRequest;
use crate::types::{LoginResult, LoginSchema};

impl Instance {
    pub async fn login_account(&mut self, login_schema: &LoginSchema) -> ChorusResult<UserMeta> {
        let endpoint_url = self.urls.api.clone() + "/auth/login";
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(endpoint_url)
                .body(to_string(login_schema).unwrap()),
            limit_type: LimitType::AuthLogin,
        };
        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since login is an instance wide limit), which is why we are just cloning the
        // instances' limits to pass them on as user_rate_limits later.
        let mut shell = UserMeta::shell(Rc::new(RefCell::new(self.clone())), "None".to_string());
        let login_result = chorus_request
            .deserialize_response::<LoginResult>(&mut shell)
            .await?;
        let object = self.get_user(login_result.token.clone(), None).await?;
        if self.limits_information.is_some() {
            self.limits_information.as_mut().unwrap().ratelimits = shell.limits.clone().unwrap();
        }
        let user = UserMeta::new(
            Rc::new(RefCell::new(self.clone())),
            login_result.token,
            self.clone_limits_if_some(),
            login_result.settings,
            object,
        );
        Ok(user)
    }
}
