use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use chorus_macros::{Composite, Updateable};
use serde::{Deserialize, Serialize};

use crate::gateway::{GatewayHandle, Updateable};
use crate::types::entities::User;
use crate::types::{Composite, Snowflake};

#[derive(Debug, Clone, Deserialize, Serialize, Default, Updateable, Composite)]
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
    pub user: Option<Arc<RwLock<User>>>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

impl PartialEq for Emoji {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.roles == other.roles
            && self.roles == other.roles
            && self.require_colons == other.require_colons
            && self.managed == other.managed
            && self.animated == other.animated
            && self.available == other.available
    }
}

impl PartialOrd for Emoji {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.roles.partial_cmp(&other.roles) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.roles.partial_cmp(&other.roles) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.require_colons.partial_cmp(&other.require_colons) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.managed.partial_cmp(&other.managed) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.animated.partial_cmp(&other.animated) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.available.partial_cmp(&other.available)
    }
}
