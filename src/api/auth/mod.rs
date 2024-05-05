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
<<<<<<< HEAD
    pub async fn login_with_token(&mut self, token: &str) -> ChorusResult<ChorusUser> {
        let mut user = ChorusUser::shell(Arc::new(RwLock::new(self.clone())), token).await;
=======
    pub async fn login_with_token(&mut self, token: String) -> ChorusResult<ChorusUser> {
        let mut user =
            ChorusUser::shell(Arc::new(RwLock::new(self.clone())), token).await;
>>>>>>> 03f1e7d (Refactor / fix login and register (#495))

        let object = User::get(&mut user, None).await?;
        let settings = User::get_settings(&mut user).await?;

        *user.object.write().unwrap() = object;
        *user.settings.write().unwrap() = settings;

        let mut identify = GatewayIdentifyPayload::common();
        identify.token = user.token();
        user.gateway.send_identify(identify).await;

        Ok(user)
    }
}
