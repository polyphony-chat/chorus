use serde::{Deserialize, Serialize};

use crate::entities::User;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Emoji {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub roles: Option<Vec<u64>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
