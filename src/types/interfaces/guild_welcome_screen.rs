use crate::types::utils::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct WelcomeScreenObject {
    pub enabled: bool,
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct WelcomeScreenChannel {
    pub channel_id: Snowflake,
    pub description: String,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}
