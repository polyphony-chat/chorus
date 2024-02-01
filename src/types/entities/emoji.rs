// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::gateway::Shared;
use crate::types::entities::User;
use crate::types::Snowflake;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::{Composite, Updateable};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/emoji#emoji-object>
pub struct Emoji {
    pub id: Snowflake,
    pub name: Option<String>,
    #[cfg(feature = "sqlx")]
    pub roles: Option<sqlx::types::Json<Vec<Snowflake>>>,
    #[cfg(not(feature = "sqlx"))]
    pub roles: Option<Vec<Snowflake>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

impl std::hash::Hash for Emoji {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.roles.hash(state);
        self.roles.hash(state);
        self.require_colons.hash(state);
        self.managed.hash(state);
        self.animated.hash(state);
        self.available.hash(state);
    }
}

impl PartialEq for Emoji {
    fn eq(&self, other: &Self) -> bool {
        !(self.id != other.id
            || self.name != other.name
            || self.roles != other.roles
            || self.require_colons != other.require_colons
            || self.managed != other.managed
            || self.animated != other.animated
            || self.available != other.available)
    }
}
