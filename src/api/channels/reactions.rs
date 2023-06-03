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
    /**
     Deletes all reactions for a message.
    # Arguments
     * `user` - A mutable reference to a `UserMeta` instance.

     # Returns
     A `Result` containing a `reqwest::Response` or a `crate::errors::InstanceServerError`.
     */
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

    /**
    Gets a list of users that reacted with a specific emoji to a message.

    # Arguments
    * `emoji` - A string slice containing the emoji to search for.
    * `user` - A mutable reference to a `UserMeta` instance.

    # Returns
    A `Result` containing a `reqwest::Response` or a `crate::errors::InstanceServerError`.
    */
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
