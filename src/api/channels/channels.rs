use reqwest::Client;
use serde_json::to_string;

use crate::types::AddChannelRecipientSchema;
use crate::{
    api::LimitType,
    errors::{ChorusError, ChorusResult},
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{Channel, ChannelModifySchema, GetChannelMessagesSchema, Message, Snowflake},
};

impl Channel {
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
        &self,
        modify_data: ChannelModifySchema,
        channel_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<Channel> {
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
        chorus_request.deserialize_response::<Channel>(user).await
    }

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

    /// # Reference:
    /// Read: <https://discord-userdoccers.vercel.app/resources/channel#add-channel-recipient>
    pub async fn add_channel_recipient(
        &self,
        recipient_id: Snowflake,
        user: &mut UserMeta,
        add_channel_recipient_schema: Option<AddChannelRecipientSchema>,
    ) -> ChorusResult<()> {
        let mut request = Client::new()
            .put(format!(
                "{}/channels/{}/recipients/{}/",
                user.belongs_to.borrow().urls.api,
                self.id,
                recipient_id
            ))
            .bearer_auth(user.token());
        if let Some(schema) = add_channel_recipient_schema {
            request = request.body(to_string(&schema).unwrap());
        }
        ChorusRequest {
            request,
            limit_type: LimitType::Channel(self.id),
        }
        .handle_request_as_result(user)
        .await
    }

    /// # Reference:
    /// Read: <https://discord-userdoccers.vercel.app/resources/channel#remove-channel-recipient>
    pub async fn remove_channel_recipient(
        &self,
        recipient_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<()> {
        let request = Client::new()
            .delete(format!(
                "{}/channels/{}/recipients/{}/",
                user.belongs_to.borrow().urls.api,
                self.id,
                recipient_id
            ))
            .bearer_auth(user.token());
        ChorusRequest {
            request,
            limit_type: LimitType::Channel(self.id),
        }
        .handle_request_as_result(user)
        .await
    }
}
