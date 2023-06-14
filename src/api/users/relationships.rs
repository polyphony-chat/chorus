use reqwest::Client;

use crate::{api::deserialize_response, errors::ChorusLibError, instance::UserMeta, types};

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
}
