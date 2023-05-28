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

    pub async fn delete(
        token: &str,
        url_api: &str,
        channel_id: &str,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Option<InstanceServerError> {
        let request = Client::new()
            .delete(format!("{}/channels/{}/", url_api, channel_id))
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
