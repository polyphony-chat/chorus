use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::{deserialize_response, handle_request_as_option},
    errors::ChorusLibError,
    instance::UserMeta,
    types,
};

impl UserMeta {
    /// Retrieves the mutual relationships between the authenticated user and the specified user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - A string slice that holds the ID of the user to retrieve the mutual relationships with.
    ///
    /// # Returns
    /// This function returns a [`Option<Vec<Result<PublicUser, ChorusLibError>>>`].
    pub async fn get_mutual_relationships(
        &mut self,
        user_id: &str,
    ) -> Result<Option<Vec<types::PublicUser>>, ChorusLibError> {
        let belongs_to = self.belongs_to.borrow();
        let url = format!(
            "{}/users/{}/relationships/",
            belongs_to.urls.get_api(),
            user_id
        );
        drop(belongs_to);
        let request = Client::new().get(url).bearer_auth(self.token());
        deserialize_response::<Option<Vec<types::PublicUser>>>(
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
    /// This function returns an [`Option`] that holds a [`ChorusLibError`] if the request fails.
    pub async fn send_friend_request(
        &mut self,
        schema: types::FriendRequestSendSchema,
    ) -> Option<ChorusLibError> {
        let belongs_to = self.belongs_to.borrow();
        let url = format!("{}/users/@me/relationships/", belongs_to.urls.get_api());
        drop(belongs_to);
        let body = to_string(&schema).unwrap();
        let request = Client::new().post(url).bearer_auth(self.token()).body(body);
        handle_request_as_option(request, self, crate::api::limits::LimitType::Global).await
    }
}
