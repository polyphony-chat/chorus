use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_string_from_number, deserialize_option_number_from_string};

use crate::types::utils::Snowflake;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See https://discord.com/developers/docs/topics/permissions#role-object
pub struct RoleObject {
    pub id: Snowflake,
    pub name: String,
    pub color: f64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u16,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: Snowflake,
    pub tier_name: String,
    pub total_months_subscribed: u32,
    pub is_renewal: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
/// See https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
pub struct RoleTags {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub bot_id: Option<usize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub integration_id: Option<usize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub subscription_listing_id: Option<usize>,
    // These use the bad bool format, "Tags with type null represent booleans. They will be present and set to null if they are "true", and will be not present if they are "false"."
    // premium_subscriber: bool,
    // available_for_purchase: bool,
    // guild_connections: bool,
}
