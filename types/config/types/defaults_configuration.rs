use serde::{Deserialize, Serialize};

use crate::config::types::subconfigs::defaults::{guild::GuildDefaults, user::UserDefaults};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultsConfiguration {
    pub guild: GuildDefaults,
    pub user: UserDefaults,
}
