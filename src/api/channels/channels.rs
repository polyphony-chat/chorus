use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::common,
    errors::ChorusLibError,
    instance::UserMeta,
    types::{Channel, ChannelModifySchema},
};

impl Channel {
    pub async fn get(user: &mut UserMeta, channel_id: &str) -> Result<Channel, ChorusLibError> {
        let belongs_to = user.belongs_to.borrow_mut();
        let url = belongs_to.urls.get_api().to_string();
        drop(belongs_to);
        let request = Client::new()
            .get(format!("{}/channels/{}/", url, channel_id))
            .bearer_auth(user.token());

        let result = common::deserialize_response::<Channel>(
            request,
            user,
            crate::api::limits::LimitType::Channel,
        )
        .await;
        if result.is_err() {
            return Err(ChorusLibError::RequestErrorError {
                url: format!("{}/channels/{}/", url, channel_id),
                error: result.err().unwrap().to_string(),
            });
        }
        Ok(result.unwrap())
    }

    /// Deletes a channel.
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice that holds the authorization token.
    /// * `url_api` - A string slice that holds the URL of the API.
    /// * `channel` - A `Channel` object that represents the channel to be deleted.
    /// * `limits_user` - A mutable reference to a `Limits` object that represents the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` object that represents the instance's rate limits.
    ///
    /// # Returns
    ///
    /// An `Option` that contains an `ChorusLibError` if an error occurred during the request, or `None` if the request was successful.
    pub async fn delete(self, user: &mut UserMeta) -> Option<ChorusLibError> {
        let belongs_to = user.belongs_to.borrow_mut();
        let request = Client::new()
            .delete(format!(
                "{}/channels/{}/",
                belongs_to.urls.get_api(),
                self.id.to_string()
            ))
            .bearer_auth(user.token());
        drop(belongs_to);
        let response =
            common::handle_request(request, user, crate::api::limits::LimitType::Channel).await;
        if response.is_err() {
            return Some(response.err().unwrap());
        } else {
            return None;
        }
    }

    /// Modifies a channel.
    ///
    /// # Arguments
    ///
    /// * `modify_data` - A `ChannelModifySchema` object that represents the modifications to be made to the channel.
    /// * `token` - A string slice that holds the authorization token.
    /// * `url_api` - A string slice that holds the URL of the API.
    /// * `channel_id` - A string slice that holds the ID of the channel to be modified.
    /// * `limits_user` - A mutable reference to a `Limits` object that represents the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` object that represents the instance's rate limits.
    ///
    /// # Returns
    ///
    /// A `Result` that contains a `Channel` object if the request was successful, or an `ChorusLibError` if an error occurred during the request.
    pub async fn modify(
        modify_data: ChannelModifySchema,
        channel_id: &str,
        user: &mut UserMeta,
    ) -> Result<Channel, ChorusLibError> {
        let belongs_to = user.belongs_to.borrow();
        let request = Client::new()
            .patch(format!(
                "{}/channels/{}/",
                belongs_to.urls.get_api(),
                channel_id
            ))
            .bearer_auth(user.token())
            .body(to_string(&modify_data).unwrap());
        drop(belongs_to);
        let channel = common::deserialize_response::<Channel>(
            request,
            user,
            crate::api::limits::LimitType::Channel,
        )
        .await
        .unwrap();
        Ok(channel)
    }
}
