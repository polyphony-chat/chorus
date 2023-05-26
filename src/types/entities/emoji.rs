use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

use crate::types::entities::User;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Emoji {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u64>,
    pub name: Option<String>,
    pub roles: Option<Vec<u64>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
