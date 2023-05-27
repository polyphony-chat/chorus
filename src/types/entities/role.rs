use serde::{Deserialize, Serialize};

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
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    // to:do add role tags https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
    //pub tags: Option<RoleTags>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: Snowflake,
    pub tier_name: String,
    pub total_months_subscribed: u32,
    pub is_renewal: bool,
}
