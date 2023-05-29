use reqwest::Client;
use serde_json::from_str;

use crate::{
    api::limits::Limits, errors::InstanceServerError, limit::LimitedRequester, types::Channel,
};

impl Channel {
    pub async fn get(
        token: &str,
        url_api: &str,
        channel_id: &str,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Result<Channel, InstanceServerError> {
        let request = Client::new()
            .get(format!("{}/channels/{}/", url_api, channel_id))
            .bearer_auth(token);
        let mut requester = LimitedRequester::new().await;
        let result = match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
                limits_user,
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
                url: format!("{}/channels/{}/", url_api, channel_id),
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
}
