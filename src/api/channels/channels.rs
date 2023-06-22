use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::common,
    errors::{ChorusLibError, ChorusResult},
    instance::UserMeta,
    types::{Channel, ChannelModifySchema, GetChannelMessagesSchema, Message, Snowflake},
};

impl Channel {
    pub async fn get(user: &mut UserMeta, channel_id: Snowflake) -> ChorusResult<Channel> {
        let url = user.belongs_to.borrow_mut().urls.api.clone();
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
    /// A `Result` that contains a `ChorusLibError` if an error occurred during the request, or `()` if the request was successful.
    pub async fn delete(self, user: &mut UserMeta) -> ChorusResult<()> {
        let request = Client::new()
            .delete(format!(
                "{}/channels/{}/",
                user.belongs_to.borrow_mut().urls.api,
                self.id
            ))
            .bearer_auth(user.token());
        common::handle_request_as_result(request, user, crate::api::limits::LimitType::Channel)
            .await
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
        channel_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<Channel> {
        let request = Client::new()
            .patch(format!(
                "{}/channels/{}/",
                user.belongs_to.borrow().urls.api,
                channel_id
            ))
            .bearer_auth(user.token())
            .body(to_string(&modify_data).unwrap());
        common::deserialize_response::<Channel>(
            request,
            user,
            crate::api::limits::LimitType::Channel,
        )
        .await
    }

    pub async fn messages(
        range: GetChannelMessagesSchema,
        channel_id: Snowflake,
        user: &mut UserMeta,
    ) -> Result<Vec<Message>, ChorusLibError> {
        let request = Client::new()
            .get(format!(
                "{}/channels/{}/messages",
                user.belongs_to.borrow().urls.api,
                channel_id
            ))
            .bearer_auth(user.token())
            .query(&range);

        common::deserialize_response::<Vec<Message>>(request, user, Default::default()).await
    }
}
