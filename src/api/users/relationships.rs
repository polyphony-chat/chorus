use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::{deserialize_response, handle_request_as_result},
    errors::ChorusResult,
    instance::UserMeta,
    types::{self, CreateUserRelationshipSchema, RelationshipType, Snowflake},
};

impl UserMeta {
    /// Retrieves the mutual relationships between the authenticated user and the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user to retrieve the mutual relationships with.
    ///
    /// # Returns
    /// This function returns a [`ChorusResult<Vec<PublicUser>>`].
    pub async fn get_mutual_relationships(
        &mut self,
        user_id: Snowflake,
    ) -> ChorusResult<Vec<types::PublicUser>> {
        let url = format!(
            "{}/users/{}/relationships/",
            self.belongs_to.borrow().urls.api,
            user_id
        );
        let request = Client::new().get(url).bearer_auth(self.token());
        deserialize_response::<Vec<types::PublicUser>>(
            request,
            self,
            crate::api::limits::LimitType::Global,
        )
        .await
    }

    /// Retrieves the authenticated user's relationships.
    ///
    /// # Returns
    /// This function returns a [`ChorusResult<Vec<types::Relationship>>`].
    pub async fn get_relationships(&mut self) -> ChorusResult<Vec<types::Relationship>> {
        let url = format!(
            "{}/users/@me/relationships/",
            self.belongs_to.borrow().urls.api
        );
        let request = Client::new().get(url).bearer_auth(self.token());
        deserialize_response::<Vec<types::Relationship>>(
            request,
            self,
            crate::api::limits::LimitType::Global,
        )
        .await
    }

    /// Sends a friend request to a user.
    ///
    /// # Arguments
    ///
    /// * `schema` - A [`FriendRequestSendSchema`] struct that holds the information about the friend request to be sent.
    ///
    /// # Returns
    /// This function returns a [`Result`] that holds a [`ChorusLibError`] if the request fails.
    pub async fn send_friend_request(
        &mut self,
        schema: types::FriendRequestSendSchema,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships/",
            self.belongs_to.borrow().urls.api
        );
        let body = to_string(&schema).unwrap();
        let request = Client::new().post(url).bearer_auth(self.token()).body(body);
        handle_request_as_result(request, self, crate::api::limits::LimitType::Global).await
    }

    /// Modifies the relationship between the authenticated user and the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user to modify the relationship with.
    /// * `relationship_type` - A [`RelationshipType`] enum that specifies the type of relationship to modify.
    ///     * [`RelationshipType::None`]: Removes the relationship between the two users.
    ///     * [`RelationshipType::Friends`] | [`RelationshipType::Incoming`] | [`RelationshipType::Outgoing`]:
    ///     Either accepts an incoming friend request, or sends a new friend request, if there is no
    ///     incoming friend request from the specified `user_id`.
    ///     * [`RelationshipType::Blocked`]: Blocks the specified user_id.
    ///
    /// # Returns
    /// This function returns an [`Result`] that holds a [`ChorusLibError`] if the request fails.
    pub async fn modify_user_relationship(
        &mut self,
        user_id: Snowflake,
        relationship_type: RelationshipType,
    ) -> ChorusResult<()> {
        let api_url = self.belongs_to.borrow().urls.api.clone();
        match relationship_type {
            RelationshipType::None => {
                let request = Client::new()
                    .delete(format!("{}/users/@me/relationships/{}/", api_url, user_id))
                    .bearer_auth(self.token());
                handle_request_as_result(request, self, crate::api::limits::LimitType::Global).await
            }
            RelationshipType::Friends | RelationshipType::Incoming | RelationshipType::Outgoing => {
                let body = CreateUserRelationshipSchema {
                    relationship_type: None, // Selecting 'None' here will accept an incoming FR or send a new FR.
                    from_friend_suggestion: None,
                    friend_token: None,
                };
                let request = Client::new()
                    .put(format!("{}/users/@me/relationships/{}/", api_url, user_id))
                    .bearer_auth(self.token())
                    .body(to_string(&body).unwrap());
                handle_request_as_result(request, self, crate::api::limits::LimitType::Global).await
            }
            RelationshipType::Blocked => {
                let body = CreateUserRelationshipSchema {
                    relationship_type: Some(RelationshipType::Blocked),
                    from_friend_suggestion: None,
                    friend_token: None,
                };
                let request = Client::new()
                    .put(format!("{}/users/@me/relationships/{}/", api_url, user_id))
                    .bearer_auth(self.token())
                    .body(to_string(&body).unwrap());
                handle_request_as_result(request, self, crate::api::limits::LimitType::Global).await
            }
            RelationshipType::Suggestion | RelationshipType::Implicit => Ok(()),
        }
    }

    /// Removes the relationship between the authenticated user and the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - ID of the user to remove the relationship with.
    ///
    /// # Returns
    /// This function returns a [`Result`] that holds a [`ChorusLibError`] if the request fails.
    pub async fn remove_relationship(&mut self, user_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/users/@me/relationships/{}/",
            self.belongs_to.borrow().urls.api,
            user_id
        );
        let request = Client::new().delete(url).bearer_auth(self.token());
        handle_request_as_result(request, self, crate::api::limits::LimitType::Global).await
    }
}
