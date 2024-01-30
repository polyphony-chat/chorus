// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::types::{AddChannelRecipientSchema, ModifyChannelPositionsSchema};
use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        Channel, ChannelModifySchema, GetChannelMessagesSchema, LimitType, Message, Snowflake,
    },
};

impl Channel {
    /// Retrieves a channel from the server.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#get-channel>
    pub async fn get(user: &mut ChorusUser, channel_id: Snowflake) -> ChorusResult<Channel> {
        let chorus_request = ChorusRequest::new(
            http::Method::GET,
            &format!(
                "{}/channels/{}",
                user.belongs_to.read().unwrap().urls.api.clone(),
                channel_id
            ),
            None,
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );

        chorus_request.deserialize_response::<Channel>(user).await
    }

    /// Deletes self.
    ///
    /// Requires the [`MANAGE_CHANNELS`](crate::types::PermissionFlags::MANAGE_CHANNELS) permission in a guild, or
    /// the [`MANAGE_THREADS`](crate::types::PermissionFlags::MANAGE_THREADS) permission if the channel is a thread.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#delete-channel>
    pub async fn delete(
        self,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}",
            user.belongs_to.read().unwrap().urls.api,
            self.id,
        );

        let request = ChorusRequest::new(
            http::Method::DELETE,
            &url,
            None,
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Channel(self.id),
        );

        request.handle_request_as_result(user).await
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
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<Channel> {
        let channel_id = self.id;
        let url = format!(
            "{}/channels/{}",
            user.belongs_to.read().unwrap().urls.api,
            channel_id
        );

        let request = ChorusRequest::new(
            http::Method::PATCH,
            &url,
            Some(to_string(&modify_data).unwrap()),
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );

        request.deserialize_response::<Channel>(user).await
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
        user: &mut ChorusUser,
    ) -> Result<Vec<Message>, ChorusError> {
        let url = format!(
            "{}/channels/{}/messages",
            user.belongs_to.read().unwrap().urls.api,
            channel_id
        );

        let mut chorus_request = ChorusRequest::new(
            http::Method::GET,
            &url,
            None,
            None,
            None,
            Some(user),
            Default::default(),
        );
        chorus_request.request = chorus_request.request.query(&range);

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
        user: &mut ChorusUser,
        add_channel_recipient_schema: Option<AddChannelRecipientSchema>,
    ) -> ChorusResult<()> {
        let mut request = Client::new()
            .put(format!(
                "{}/channels/{}/recipients/{}",
                user.belongs_to.read().unwrap().urls.api,
                self.id,
                recipient_id
            ))
            .header("Authorization", user.token())
            .header("Content-Type", "application/json");
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
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/recipients/{}",
            user.belongs_to.read().unwrap().urls.api,
            self.id,
            recipient_id
        );

        let request = ChorusRequest::new(
            http::Method::DELETE,
            &url,
            None,
            None,
            None,
            Some(user),
            LimitType::Channel(self.id),
        );

        request.handle_request_as_result(user).await
    }

    /// Modifies the positions of a set of channel objects for the guild. Requires the `MANAGE_CHANNELS` permission.
    /// Only channels to be modified are required.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/channel#modify-guild-channel-positions>
    pub async fn modify_positions(
        schema: Vec<ModifyChannelPositionsSchema>,
        guild_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/channels",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest::new(
            http::Method::PATCH,
            &url,
            Some(to_string(&schema).unwrap()),
            None,
            None,
            Some(user),
            LimitType::Guild(guild_id),
        );

        request.handle_request_as_result(user).await
    }
}
