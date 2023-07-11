use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::LimitType,
    errors::{ChorusError, ChorusResult},
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{Channel, ChannelModifySchema, GetChannelMessagesSchema, Message, Snowflake},
};

impl Channel {
    /// Retrieves a channel from the server.
    pub async fn get(user: &mut UserMeta, channel_id: Snowflake) -> ChorusResult<Channel> {
        let url = user.belongs_to.borrow().urls.api.clone();
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!("{}/channels/{}/", url, channel_id))
                .bearer_auth(user.token()),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.deserialize_response::<Channel>(user).await
    }

    /// Deletes self.
    pub async fn delete(self, user: &mut UserMeta) -> ChorusResult<()> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .delete(format!(
                    "{}/channels/{}/",
                    user.belongs_to.borrow().urls.api,
                    self.id
                ))
                .bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Modifies a channel with the provided data.
    /// Replaces self with the new channel object.
    pub async fn modify(
        &mut self,
        modify_data: ChannelModifySchema,
        channel_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<()> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/channels/{}/",
                    user.belongs_to.borrow().urls.api,
                    channel_id
                ))
                .bearer_auth(user.token())
                .body(to_string(&modify_data).unwrap()),
            limit_type: LimitType::Channel(channel_id),
        };
        let new_channel = chorus_request.deserialize_response::<Channel>(user).await?;
        let _ = std::mem::replace(self, new_channel);
        Ok(())
    }

    /// Fetches recent messages from a channel.
    pub async fn messages(
        range: GetChannelMessagesSchema,
        channel_id: Snowflake,
        user: &mut UserMeta,
    ) -> Result<Vec<Message>, ChorusError> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/channels/{}/messages",
                    user.belongs_to.borrow().urls.api,
                    channel_id
                ))
                .bearer_auth(user.token())
                .query(&range),
            limit_type: Default::default(),
        };

        chorus_request
            .deserialize_response::<Vec<Message>>(user)
            .await
    }
}
