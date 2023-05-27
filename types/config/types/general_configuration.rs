use serde::{Deserialize, Serialize};

use crate::utils::Snowflake;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralConfiguration {
    pub instance_name: String,
    pub instance_description: Option<String>,
    pub front_page: Option<String>,
    pub tos_page: Option<String>,
    pub correspondence_email: Option<String>,
    pub correspondence_user_id: Option<String>,
    pub image: Option<String>,
    pub instance_id: Option<Snowflake>,
    pub auto_create_bot_users: Option<bool>,
}

impl Default for GeneralConfiguration {
    fn default() -> Self {
        Self {
            instance_name: String::from("Spacebar Instance"),
            instance_description: Some(String::from(
                "This is a Spacebar instance made in the pre-release days",
            )),
            front_page: None,
            tos_page: None,
            correspondence_email: None,
            correspondence_user_id: None,
            image: None,
            instance_id: Some(Snowflake::generate()),
            auto_create_bot_users: Some(false),
        }
    }
}
