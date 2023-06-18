use serde::{Deserialize, Serialize};

use crate::types::RelationshipType;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FriendRequestSendSchema {
    pub username: String,
    pub discriminator: Option<String>,
}

/// Represents the schema for the Create User Relationship route.
/// # Arguments
///
/// * relationship_type: The [`RelationshipType`] to create (defaults to -1, which accepts an existing or creates a new friend request)
/// * from_friend_suggestion: Whether the relationship was created from a friend suggestion (default false)
/// * friend_token: The friend token of the user to add a direct friend relationship to
///
/// See: [https://discord-userdoccers.vercel.app/resources/user#create-user-relationship](https://discord-userdoccers.vercel.app/resources/user#create-user-relationship)
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateUserRelationshipSchema {
    #[serde(rename = "type")]
    pub relationship_type: Option<RelationshipType>,
    pub from_friend_suggestion: Option<bool>,
    pub friend_token: Option<String>,
}
