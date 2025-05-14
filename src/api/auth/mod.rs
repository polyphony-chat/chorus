// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::{Arc, RwLock};

#[allow(unused_imports)]
pub use login::*;

#[allow(unused_imports)]
pub use register::*;

use crate::gateway::Gateway;
use crate::types::Shared;
use crate::{
    errors::ChorusResult,
    instance::{ChorusUser, Instance},
    types::{GatewayIdentifyPayload, User},
};

pub mod login;
pub mod register;

impl Instance {
    /// Logs into an existing account on the spacebar server, using only a token.
    pub async fn login_with_token(
        instance: Shared<Instance>,
        token: &str,
    ) -> ChorusResult<ChorusUser> {
        let mut user = ChorusUser::shell(instance, token).await;

        user.update_with_login_data(token.to_string(), None).await?;

        Ok(user)
    }
}
