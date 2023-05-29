use reqwest::Client;
use serde_json::{from_str, to_string};

use crate::{
    api::limits::Limits,
    errors::InstanceServerError,
    instance::UserMeta,
    limit::LimitedRequester,
    types::{Channel, ChannelModifySchema},
};

impl Channel {
    pub async fn get(
        user: &mut UserMeta,
        channel_id: &str,
    ) -> Result<Channel, InstanceServerError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let request = Client::new()
            .get(format!(
                "{}/channels/{}/",
                belongs_to.urls.get_api(),
                channel_id
            ))
            .bearer_auth(user.token());
        let mut requester = LimitedRequester::new().await;
        let result = match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        let result_text = result.text().await.unwrap();
        match from_str::<Channel>(&result_text) {
            Ok(object) => Ok(object),
            Err(e) => Err(InstanceServerError::RequestErrorError {
                url: format!("{}/channels/{}/", belongs_to.urls.get_api(), channel_id),
                error: e.to_string(),
            }),
        }
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
    /// An `Option` that contains an `InstanceServerError` if an error occurred during the request, or `None` if the request was successful.
    pub async fn delete(
        self,
        token: &str,
        url_api: &str,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Option<InstanceServerError> {
        let request = Client::new()
            .delete(format!("{}/channels/{}/", url_api, self.id.to_string()))
            .bearer_auth(token);
        match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Channel,
                limits_instance,
                limits_user,
            )
            .await
        {
            Ok(_) => None,
            Err(e) => return Some(e),
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
    /// A `Result` that contains a `Channel` object if the request was successful, or an `InstanceServerError` if an error occurred during the request.
    pub async fn modify(
        modify_data: ChannelModifySchema,
        token: &str,
        url_api: &str,
        channel_id: &str,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Result<Channel, InstanceServerError> {
        let request = Client::new()
            .patch(format!("{}/channels/{}/", url_api, channel_id))
            .bearer_auth(token)
            .body(to_string(&modify_data).unwrap());
        let channel = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Channel,
                limits_instance,
                limits_user,
            )
            .await
        {
            Ok(channel) => from_str::<Channel>(&channel.text().await.unwrap()).unwrap(),
            Err(e) => return Err(e),
        };
        Ok(channel)
    }
}
