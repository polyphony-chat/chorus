use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::LimitType,
    errors::ChorusResult,
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{
        self, CreateUserRelationshipSchema, FriendRequestSendSchema, RelationshipType, Snowflake,
    },
};

impl UserMeta {
    /// Retrieves a list of mutual friends between the authenticated user and a given user.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/relationships.html#get-users-peer-id-relationships>
    pub async fn get_mutual_relationships(
        &mut self,
        user_id: Snowflake,
    ) -> ChorusResult<Vec<types::PublicUser>> {
        let url = format!(
            "{}/users/{}/relationships",
            self.belongs_to.borrow().urls.api,
            user_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).header("Authorization", self.token()),
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<Vec<types::PublicUser>>(self)
            .await
    }

    /// Retrieves the user's relationships.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/relationships.html#get-users-me-relationships>
    pub async fn get_relationships(&mut self) -> ChorusResult<Vec<types::Relationship>> {
        let url = format!(
            "{}/users/@me/relationships",
            self.belongs_to.borrow().urls.api
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).header("Authorization", self.token()),
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<Vec<types::Relationship>>(self)
            .await
    }

    /// Sends a friend request to a user.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/relationships.html#post-users-me-relationships>
    pub async fn send_friend_request(
        &mut self,
        schema: FriendRequestSendSchema,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships",
            self.belongs_to.borrow().urls.api
        );
        let body = to_string(&schema).unwrap();
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(url)
                .header("Authorization", self.token())
                .header("Content-Type", "application/json")
                .body(body),
            limit_type: LimitType::Global,
        };
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
        let api_url = self.belongs_to.borrow().urls.api.clone();
        match relationship_type {
            RelationshipType::None => {
                let chorus_request = ChorusRequest {
                    request: Client::new()
                        .delete(format!("{}/users/@me/relationships/{}", api_url, user_id))
                        .header("Authorization", self.token()),
                    limit_type: LimitType::Global,
                };
                chorus_request.handle_request_as_result(self).await
            }
            RelationshipType::Friends | RelationshipType::Incoming | RelationshipType::Outgoing => {
                let body = CreateUserRelationshipSchema {
                    relationship_type: None, // Selecting 'None' here will accept an incoming FR or send a new FR.
                    from_friend_suggestion: None,
                    friend_token: None,
                };
                let chorus_request = ChorusRequest {
                    request: Client::new()
                        .put(format!("{}/users/@me/relationships/{}", api_url, user_id))
                        .header("Authorization", self.token())
                        .body(to_string(&body).unwrap()),
                    limit_type: LimitType::Global,
                };
                chorus_request.handle_request_as_result(self).await
            }
            RelationshipType::Blocked => {
                let body = CreateUserRelationshipSchema {
                    relationship_type: Some(RelationshipType::Blocked),
                    from_friend_suggestion: None,
                    friend_token: None,
                };
                let chorus_request = ChorusRequest {
                    request: Client::new()
                        .put(format!("{}/users/@me/relationships/{}", api_url, user_id))
                        .header("Authorization", self.token())
                        .body(to_string(&body).unwrap()),
                    limit_type: LimitType::Global,
                };
                chorus_request.handle_request_as_result(self).await
            }
            RelationshipType::Suggestion | RelationshipType::Implicit => Ok(()),
        }
    }

    /// Removes the relationship between the authenticated user and a given user.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/relationships.html#delete-users-me-relationships-peer-id>
    pub async fn remove_relationship(&mut self, user_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships/{}",
            self.belongs_to.borrow().urls.api,
            user_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new()
                .delete(url)
                .header("Authorization", self.token()),
            limit_type: LimitType::Global,
        };
        chorus_request.handle_request_as_result(self).await
    }
}
