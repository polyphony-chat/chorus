// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::RelationshipType;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/user#create-user-relationship>
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct CreateUserRelationshipSchema {
    #[serde(rename = "type")]
    pub relationship_type: Option<RelationshipType>,
    pub from_friend_suggestion: Option<bool>,
    pub friend_token: Option<String>,
}

/// Optional query parameters for the
/// [ChorusUser::bulk_remove_relationships](crate::instance::ChorusUser::bulk_remove_relationships)
/// route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/relationships#bulk-remove-relationships>
#[derive(Deserialize, Serialize, Debug, Copy, Default, Clone, PartialEq, Eq)]
pub struct BulkRemoveRelationshipsQuery {
    /// Remove relationships with this type (default [RelationshipType::Incoming], only
    /// [RelationshipType::Incoming] is allowed.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<RelationshipType>,

    /// Whether to only remove relationships that were flagged as spam (false by default)
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_spam: Option<bool>,
}

impl BulkRemoveRelationshipsQuery {
    /// Converts self to query string parameters
    pub fn to_query(self) -> Vec<(&'static str, String)> {
        let mut query = Vec::with_capacity(2);

        if let Some(relationship_type) = self.relationship_type {
            query.push((
                "relationship_type",
                serde_json::to_string(&relationship_type).unwrap(),
            ));
        }

        if let Some(only_spam) = self.only_spam {
            query.push(("only_spam", only_spam.to_string()));
        }

        query
    }
}
