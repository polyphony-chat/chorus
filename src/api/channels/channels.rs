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
    /// Retrieves a channel from the server.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#get-channel>
    pub async fn get(user: &mut UserMeta, channel_id: Snowflake) -> ChorusResult<Channel> {
        let url = user.belongs_to.borrow().urls.api.clone();
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!("{}/channels/{}", url, channel_id))
                .bearer_auth(user.token()),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.deserialize_response::<Channel>(user).await
    }

    /// Deletes self.
    ///
    /// Requires the [`MANAGE_CHANNELS`](crate::types::PermissionFlags::MANAGE_CHANNELS) permission in a guild, or
    /// the [`MANAGE_THREADS`](crate::types::PermissionFlags::MANAGE_THREADS) permission if the channel is a thread.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#delete-channel>
    pub async fn delete(self, user: &mut UserMeta) -> ChorusResult<()> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .delete(format!(
                    "{}/channels/{}",
                    user.belongs_to.borrow().urls.api,
                    self.id
                ))
                .bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Modifies a channel with the provided data.
    /// Returns the new Channel.
    ///
    /// Requires the [`MANAGE_CHANNELS`](crate::types::PermissionFlags::MANAGE_CHANNELS) permission in a guild.
    ///
    /// If modifying permission overwrites, the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission is required.
    /// Only permissions you have in the guild or parent channel (if applicable) can be allowed/denied
    /// (unless you have a [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) overwrite in the channel).
    ///
    /// If modifying a thread and setting `archived` to `false`, when `locked` is also `false`, only the [`SEND_MESSAGES`](crate::types::PermissionFlags::SEND_MESSAGES) permission is required.
    /// Otherwise, requires the [`MANAGE_THREADS`](crate::types::PermissionFlags::MANAGE_THREADS) permission. Requires the thread to have `archived` set to `false` or be set to `false` in the request.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#modify-channel>
    pub async fn modify(
        &self,
        modify_data: ChannelModifySchema,
        channel_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<Channel> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/channels/{}",
                    user.belongs_to.borrow().urls.api,
                    channel_id
                ))
                .bearer_auth(user.token())
                .body(to_string(&modify_data).unwrap()),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.deserialize_response::<Channel>(user).await
    }

    /// Fetches recent messages from a channel.
    ///
    /// If operating on a guild channel, this endpoint requires the [`VIEW_CHANNEL`](crate::types::PermissionFlags::VIEW_CHANNEL) permission.
    ///
    /// If the user is missing the [`READ_MESSAGE_HISTORY`](crate::types::PermissionFlags::READ_MESSAGE_HISTORY) permission,
    /// this method returns an empty list.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/message#get-messages>
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

    /// Adds a recipient to a group DM.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/channel#add-channel-recipient>
    pub async fn add_channel_recipient(
        &self,
        recipient_id: Snowflake,
        user: &mut UserMeta,
        add_channel_recipient_schema: Option<AddChannelRecipientSchema>,
    ) -> ChorusResult<()> {
        let mut request = Client::new()
            .put(format!(
                "{}/channels/{}/recipients/{}",
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

    /// Removes a recipient from a group DM.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/channel#remove-channel-recipient>
    pub async fn remove_channel_recipient(
        &self,
        recipient_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<()> {
        let request = Client::new()
            .delete(format!(
                "{}/channels/{}/recipients/{}",
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
