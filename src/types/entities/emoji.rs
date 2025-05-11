// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::types::entities::User;
use crate::types::Snowflake;
use crate::types::{PartialEmoji, Shared};

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use crate::types::Composite;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use chorus_macros::{Composite, Updateable};

use super::option_arc_rwlock_ptr_eq;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "client", derive(Updateable, Composite))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://docs.discord.food/resources/emoji#emoji-object>
pub struct Emoji {
    pub id: Snowflake,
    pub name: Option<String>,
    pub roles: Option<Vec<Snowflake>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[cfg(not(tarpaulin_include))]
#[allow(clippy::nonminimal_bool)]
impl PartialEq for Emoji {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.roles == other.roles
            && self.roles == other.roles
            && option_arc_rwlock_ptr_eq(&self.user, &other.user)
            && self.require_colons == other.require_colons
            && self.managed == other.managed
            && self.animated == other.animated
            && self.available == other.available
    }
}

impl From<PartialEmoji> for Emoji {
    fn from(value: PartialEmoji) -> Self {
        Self {
            id: value.id.unwrap_or_default(), // TODO: Make this method an impl to TryFrom<> instead
            name: Some(value.name),
            roles: None,
            user: None,
            require_colons: Some(value.animated),
            managed: None,
            animated: Some(value.animated),
            available: None,
        }
    }
}
