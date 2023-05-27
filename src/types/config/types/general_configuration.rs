use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

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
        }
    }
}

impl GeneralConfiguration {
    pub fn new(
        instance_name: String,
        instance_description: Option<String>,
        front_page: Option<String>,
        tos_page: Option<String>,
        correspondence_email: Option<String>,
        correspondence_user_id: Option<String>,
        image: Option<String>,
        instance_id: Option<Snowflake>,
    ) -> Self {
        Self {
            instance_name,
            instance_description,
            front_page,
            tos_page,
            correspondence_email,
            correspondence_user_id,
            image,
            instance_id,
        }
    }
}
