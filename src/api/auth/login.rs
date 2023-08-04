use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::api::LimitType;
use crate::errors::ChorusResult;
use crate::gateway::Gateway;
use crate::instance::{Instance, UserMeta};
use crate::ratelimiter::ChorusRequest;
use crate::types::{GatewayIdentifyPayload, LoginResult, LoginSchema};

impl Instance {
    /// Logs into an existing account on the spacebar server.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#post-/auth/login/>
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
        let mut shell =
            UserMeta::shell(Rc::new(RefCell::new(self.clone())), "None".to_string()).await;
        let login_result = chorus_request
            .deserialize_response::<LoginResult>(&mut shell)
            .await?;
        let object = self.get_user(login_result.token.clone(), None).await?;
        if self.limits_information.is_some() {
            self.limits_information.as_mut().unwrap().ratelimits = shell.limits.clone().unwrap();
        }
        let mut identify = GatewayIdentifyPayload::common();
        let gateway = Gateway::new(self.urls.wss.clone()).await.unwrap();
        identify.token = login_result.token.clone();
        gateway.send_identify(identify).await;
        let user = UserMeta::new(
            Rc::new(RefCell::new(self.clone())),
            login_result.token,
            self.clone_limits_if_some(),
            login_result.settings,
            Arc::new(RwLock::new(object)),
            gateway,
        );
        Ok(user)
    }
}
