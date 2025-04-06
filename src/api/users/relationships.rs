// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::ChorusResult,
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        self, BulkRemoveRelationshipsQuery, CreateUserRelationshipSchema, FriendRequestSendSchema,
        LimitType, RelationshipType, Snowflake,
    },
};

impl ChorusUser {
    /// Retrieves a list of mutual friends between the authenticated user and a given user.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/docs/relationships.html#get-userspeer_idrelationships>
    pub async fn get_mutual_relationships(
        &mut self,
        user_id: Snowflake,
    ) -> ChorusResult<Vec<types::PublicUser>> {
        let url = format!(
            "{}/users/{}/relationships",
            self.belongs_to.read().unwrap().urls.api,
            user_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);
        chorus_request
            .deserialize_response::<Vec<types::PublicUser>>(self)
            .await
    }

    /// Retrieves the user's relationships.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/docs/relationships.html#get-usersmerelationships>
    pub async fn get_relationships(&mut self) -> ChorusResult<Vec<types::Relationship>> {
        let url = format!(
            "{}/users/@me/relationships",
            self.belongs_to.read().unwrap().urls.api
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);
        chorus_request
            .deserialize_response::<Vec<types::Relationship>>(self)
            .await
    }

    /// Sends a friend request to a user.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/docs/relationships.html#post-usersmerelationships>
    pub async fn send_friend_request(
        &mut self,
        schema: FriendRequestSendSchema,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships",
            self.belongs_to.read().unwrap().urls.api
        );
        let chorus_request = ChorusRequest {
            request: Client::new().post(url).json(&schema),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);
        chorus_request.handle_request_as_result(self).await
    }

    /// Modifies the relationship between the authenticated user and a given user.
    ///
    /// Can be used to unfriend users, accept or send friend requests and block or unblock users.
    pub async fn modify_user_relationship(
        &mut self,
        user_id: Snowflake,
        relationship_type: RelationshipType,
    ) -> ChorusResult<()> {
        let api_url = self.belongs_to.read().unwrap().urls.api.clone();
        match relationship_type {
            RelationshipType::None => {
                let chorus_request = ChorusRequest {
                    request: Client::new()
                        .delete(format!("{}/users/@me/relationships/{}", api_url, user_id)),
                    limit_type: LimitType::Global,
                }
                .with_headers_for(self);
                chorus_request.handle_request_as_result(self).await
            }
            RelationshipType::Friends | RelationshipType::Incoming | RelationshipType::Outgoing => {
                let schema = CreateUserRelationshipSchema {
                    relationship_type: None, // Selecting 'None' here will accept an incoming FR or send a new FR.
                    from_friend_suggestion: None,
                    friend_token: None,
                };
                let chorus_request = ChorusRequest {
                    request: Client::new()
                        .put(format!("{}/users/@me/relationships/{}", api_url, user_id))
                        .json(&schema),
                    limit_type: LimitType::Global,
                }
                .with_headers_for(self);
                chorus_request.handle_request_as_result(self).await
            }
            RelationshipType::Blocked => {
                let schema = CreateUserRelationshipSchema {
                    relationship_type: Some(RelationshipType::Blocked),
                    from_friend_suggestion: None,
                    friend_token: None,
                };
                let chorus_request = ChorusRequest {
                    request: Client::new()
                        .put(format!("{}/users/@me/relationships/{}", api_url, user_id))
                        .json(&schema),
                    limit_type: LimitType::Global,
                }
                .with_headers_for(self);
                chorus_request.handle_request_as_result(self).await
            }
            RelationshipType::Suggestion | RelationshipType::Implicit => Ok(()),
        }
    }

    /// Removes the relationship between the authenticated user and a given user.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/docs/relationships.html#delete-usersmerelationshipspeer_id>
    pub async fn remove_relationship(&mut self, user_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships/{}",
            self.belongs_to.read().unwrap().urls.api,
            user_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);
        chorus_request.handle_request_as_result(self).await
    }

    /// Removes multiple relationships.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/relationships#bulk-remove-relationships>
    pub async fn bulk_remove_relationships(
        &mut self,
        query: Option<BulkRemoveRelationshipsQuery>,
    ) -> ChorusResult<()> {
        let query_parameters = if let Some(passed) = query {
            passed.to_query()
        } else {
            Vec::new()
        };

        let url = format!(
            "{}/users/@me/relationships",
            self.belongs_to.read().unwrap().urls.api,
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).query(&query_parameters),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);
        chorus_request.handle_request_as_result(self).await
    }

    /// [Ignores](https://support.discord.com/hc/en-us/articles/28084948873623-How-to-Ignore-Users-on-Discord) a user.
    ///
    /// # Notes
    /// As of 2025/03/16, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/relationships#ignore-user>
    pub async fn ignore_user(&mut self, user_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships/{}/ignore",
            self.belongs_to.read().unwrap().urls.api,
            user_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().put(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);

        chorus_request.handle_request_as_result(self).await
    }

    /// [Unignores](https://support.discord.com/hc/en-us/articles/28084948873623-How-to-Ignore-Users-on-Discord) a user.
    ///
    /// # Notes
    /// As of 2025/03/16, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/relationships#unignore-user>
    pub async fn unignore_user(&mut self, user_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships/{}/ignore",
            self.belongs_to.read().unwrap().urls.api,
            user_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);

        chorus_request.handle_request_as_result(self).await
    }
}
