use reqwest::Client;

use crate::{instance::UserMeta, limit::LimitedRequester, types};

/**
Extends the [`types::Reaction`] struct with useful metadata.
 */
pub struct ReactionMeta {
    pub message_id: types::Snowflake,
    pub channel_id: types::Snowflake,
    pub reaction: types::Reaction,
}

impl ReactionMeta {
    pub async fn delete_all(
        &self,
        user: &mut UserMeta,
    ) -> Result<reqwest::Response, crate::errors::InstanceServerError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/",
            belongs_to.urls.get_api(),
            self.channel_id,
            self.message_id
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Channel,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
    }

    pub async fn get(
        &self,
        emoji: &str,
        user: &mut UserMeta,
    ) -> Result<reqwest::Response, crate::errors::InstanceServerError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/",
            belongs_to.urls.get_api(),
            self.channel_id,
            self.message_id,
            emoji
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Channel,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
    }
}
