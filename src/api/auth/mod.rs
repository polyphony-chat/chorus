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
        let mut user =
            ChorusUser::shell(Arc::new(RwLock::new(self.clone())), token).await;

        let object = User::get_current(&mut user).await?;
        let settings = User::get_settings(&mut user).await?;

        *user.object.write().unwrap() = object;
        *user.settings.write().unwrap() = settings;

        let mut identify = GatewayIdentifyPayload::common();
        identify.token = user.token();
        user.gateway.send_identify(identify).await;

        Ok(user)
    }
}
