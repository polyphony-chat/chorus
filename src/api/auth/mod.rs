// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::{Arc, RwLock};

#[allow(unused_imports)]
pub use login::*;

#[allow(unused_imports)]
pub use register::*;

use crate::gateway::Gateway;
use crate::{
    errors::ChorusResult,
    instance::{ChorusUser, Instance},
    types::{GatewayIdentifyPayload, User},
};

pub mod login;
pub mod register;

impl Instance {
    /// Logs into an existing account on the spacebar server, using only a token.
    pub async fn login_with_token(&mut self, token: String) -> ChorusResult<ChorusUser> {
        let object_result = self.get_user(token.clone(), None).await;
        if let Err(e) = object_result {
            return Result::Err(e);
        }

        let user_settings = User::get_settings(&token, &self.urls.api, &mut self.clone())
            .await
            .unwrap();
        let mut identify = GatewayIdentifyPayload::common();
        let gateway = Gateway::spawn(self.urls.wss.clone()).await.unwrap();
        identify.token = token.clone();
        gateway.send_identify(identify).await;
        let user = ChorusUser::new(
            Arc::new(RwLock::new(self.clone())),
            token.clone(),
            self.clone_limits_if_some(),
            Arc::new(RwLock::new(user_settings)),
            Arc::new(RwLock::new(object_result.unwrap())),
            gateway,
        );
        Ok(user)
    }
}
