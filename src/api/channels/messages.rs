// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
use http::HeaderMap;
use reqwest::{multipart, Client};
use serde_json::{from_value, to_string, Value};

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::{ChorusUser, InstanceSoftware};
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    Channel, CreateGreetMessage, LimitType, Message, MessageAck, MessageModifySchema,
    MessageSearchEndpoint, MessageSearchQuery, MessageSendSchema, PartialDiscordFileAttachment,
    Snowflake,
};

use super::attachments;

impl MessageSendSchema {
    /// Performs a few needed operations on user-provided attachments before we're ready to
    /// upload them
    ///
    /// Makes attachment ids sequential and sets their size
    pub(crate) fn preprocess_attachments(&mut self) {
        if let Some(attachments) = self.attachments.as_mut() {
            for (index, attachment) in attachments.iter_mut().enumerate() {
                attachment.id = Some(index as u64);
                attachment.size = Some(attachment.content.len() as u64);
            }
        }
    }

    /// Converts the schema into a multipart attachment form (uploading via multipart/form-data=
    ///
    /// # Reference
    /// See <https://docs.discord.food/reference#example-request-bodies-(multipart/form-data)>
    pub(crate) fn into_multipart_attachment_form(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        let payload_json = to_string(&self).unwrap();

        let mut header_map = HeaderMap::new();
        header_map.insert(
            CONTENT_DISPOSITION,
            "form-data; name=\"payload_json\"".parse().unwrap(),
        );
        header_map.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let payload_field = reqwest::multipart::Part::text(payload_json).headers(header_map);
        form = form.part("payload_json", payload_field);

        for (index, attachment) in self.attachments.unwrap().into_iter().enumerate() {
            let attachment_content = attachment.content;
            let attachment_filename = attachment.filename;

            let part_name = format!("files[{}]", index);
            let content_disposition = format!(
                "form-data; name=\"{}\"'; filename=\"{}\"",
                part_name, &attachment_filename
            );

            let mut header_map = HeaderMap::new();
            header_map.insert(CONTENT_DISPOSITION, content_disposition.parse().unwrap());

            if let Some(content_type) = attachment.content_type {
                if let Ok(parsed_content_type) = content_type.parse() {
                    header_map.insert(CONTENT_TYPE, parsed_content_type);
                }
            }

            let part = multipart::Part::bytes(attachment_content)
                .file_name(attachment_filename)
                .headers(header_map);

            form = form.part(part_name, part);
        }

        form
    }
}

impl Message {
    /// Creates the [ChorusRequest] for the message send (create message) endpoint, without any
    /// body
    ///
    /// This is used so we don't duplicate this snippet of code for every different way to send
    /// messages
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/message#create-message>
    pub(crate) fn create_send_chorus_request(
        user: &mut ChorusUser,
        channel_id: Snowflake,
    ) -> ChorusRequest {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();

        ChorusRequest {
            request: Client::new().post(format!("{}/channels/{}/messages", url_api, channel_id)),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user)
    }

    #[allow(clippy::useless_conversion)]
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// **Does not handle attachments at all. Called by other send methods when the user has provided no
    /// attachments and assumes that has already been checked.**
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/message#create-message>
    pub(crate) async fn send_without_attachments(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        message: MessageSendSchema,
    ) -> ChorusResult<Message> {
        let mut chorus_request = Message::create_send_chorus_request(user, channel_id);
        chorus_request.request = chorus_request.request.json(&message);

        chorus_request
            .send_and_deserialize_response::<Message>(user)
            .await
    }

    #[allow(clippy::useless_conversion)]
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// Attachments are handled as a multipart form. Currently this is the only way on Spacebar (as
    /// of 2025/06/22) and the older (discouraged) way for later API versions (e.g. Discord.com)
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/message#create-message>
    pub async fn send_with_multipart_attachments(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        mut message: MessageSendSchema,
    ) -> ChorusResult<Message> {
        if message.attachments.is_none() {
            Message::send_without_attachments(user, channel_id, message).await
        } else {
            message.preprocess_attachments();

            let form = message.into_multipart_attachment_form();

            let mut chorus_request = Message::create_send_chorus_request(user, channel_id);
            chorus_request.request = chorus_request.request.multipart(form);

            chorus_request
                .send_and_deserialize_response::<Message>(user)
                .await
        }
    }

    #[allow(clippy::useless_conversion)]
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// Attachments are uploaded seperately to a cloud storage bucket and later referenced by ID.
    ///
    /// Currently this is the recommended way on later API versions (e.g. Discord.com), but is not
    /// yet available on Symfonia or Spacebar (as of 2025/06/22).
    ///
    /// For a lower level cloud attachment API, see
    /// [PartialDiscordFileAttachment::bulk_upload_to_storage_bucket].
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/message#create-message>
    pub async fn send_with_cloud_attachments(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        mut message: MessageSendSchema,
    ) -> ChorusResult<Message> {
        if message.attachments.is_none() {
            return Message::send_without_attachments(user, channel_id, message).await;
        };

        message.preprocess_attachments();

        let attachments = message.attachments.unwrap();

        let attachments_as_metadata = PartialDiscordFileAttachment::bulk_upload_to_storage_bucket(
            user,
            channel_id,
            attachments,
        )
        .await?;

        message.attachments = Some(attachments_as_metadata);

        let mut chorus_request = Message::create_send_chorus_request(user, channel_id);
        chorus_request.request = chorus_request.request.json(&message);

        chorus_request
            .send_and_deserialize_response::<Message>(user)
            .await
    }

    #[allow(clippy::useless_conversion)]
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// Attachments are handled based on the [Instance](crate::instance::Instance)'s detected
    /// [server software](crate::instance::InstanceSoftware).
    ///
    /// For Spacebar (and Symfonia for now), [Message::send_with_multipart_attachments] is used.
    /// For Other, [Message::send_with_cloud_attachments] is used.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/message#create-message>
    pub async fn send(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        message: MessageSendSchema,
    ) -> ChorusResult<Message> {
        if message.attachments.is_none() {
            return Message::send_without_attachments(user, channel_id, message).await;
        }

        let software = user.belongs_to.read().unwrap().software();

        match software {
            // FIXME: using this for Symfonia for now as well, not sure what they'll end up
            // implementing
            InstanceSoftware::SpacebarTypescript | InstanceSoftware::Symfonia => {
                Message::send_with_multipart_attachments(user, channel_id, message).await
            }
            InstanceSoftware::Other => {
                Message::send_with_cloud_attachments(user, channel_id, message).await
            }
        }
    }

    /// Returns messages without the reactions key that match a search query in the guild or channel.
    /// The messages that are direct results will have an extra hit key set to true.
    /// If operating on a guild channel, this endpoint requires the `READ_MESSAGE_HISTORY`
    /// permission to be present on the current user.
    ///
    /// If the guild/channel you are searching is not yet indexed, the endpoint will return a 202 accepted response.
    /// In this case, the method will return a [`ChorusError::InvalidResponse`] error.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/message#search-messages>
    pub(crate) async fn search(
        endpoint: MessageSearchEndpoint,
        query: MessageSearchQuery,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<Message>> {
        let limit_type = match &endpoint {
            MessageSearchEndpoint::Channel(id) => LimitType::Channel(*id),
            MessageSearchEndpoint::GuildChannel(id) => LimitType::Guild(*id),
        };
        let request = ChorusRequest {
            limit_type,
            request: Client::new()
                .get(format!(
                    "{}/{}/messages/search",
                    &user.belongs_to.read().unwrap().urls.api,
                    endpoint
                ))
                .json(&query),
        }
        .with_headers_for(user);

        let result = request.send(user).await?;

        let http_status = result.status();

        let result_json = result.json::<Value>().await.unwrap();
        if !result_json.is_object() {
            return Err(ChorusError::InvalidResponse {
                error: format!(
                    "Got unexpected Response, or Response which is not valid JSON. Response: \n{}",
                    result_json
                ),
                http_status,
            });
        }
        let value_map = result_json.as_object().unwrap();
        if let Some(messages) = value_map.get("messages") {
            if let Ok(response) = from_value::<Vec<Vec<Message>>>(messages.clone()) {
                let result_messages: Vec<Message> = response.into_iter().flatten().collect();
                return Ok(result_messages);
            }
        }
        // The code below might be incorrect. We'll cross that bridge when we come to it
        if !value_map.contains_key("code") || !value_map.contains_key("retry_after") {
            return Err(ChorusError::InvalidResponse {
                error: format!(
                    "Got unexpected Response, or Response which is not valid JSON. Response: \n{}",
                    result_json
                ),
                http_status,
            });
        }
        let code = value_map.get("code").unwrap().as_u64().unwrap();
        let retry_after = value_map.get("retry_after").unwrap().as_u64().unwrap();
        Err(ChorusError::NotFound {
            error: format!(
                "Index not yet available. Try again later. Code: {}. Retry after {}s",
                code, retry_after
            ),
        })
    }

    /// Returns all pinned messages in the channel as a Vector of message objects without the reactions key.
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#get-pinned-messages>
    pub async fn get_sticky(
        channel_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<Message>> {
        let request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/channels/{}/pins",
                user.belongs_to.read().unwrap().urls.api,
                channel_id
            )),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request
            .send_and_deserialize_response::<Vec<Message>>(user)
            .await
    }

    /// Pins a message in a channel. Requires the `MANAGE_MESSAGES` permission. Returns a 204 empty response on success.
    /// The max pinned messages is 50.
    ///
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#pin-message>
    pub async fn sticky(
        channel_id: Snowflake,
        message_id: Snowflake,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest {
            request: Client::new().put(format!(
                "{}/channels/{}/pins/{}",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }

    /// Unpins a message in a channel. Requires the `MANAGE_MESSAGES` permission. Returns a 204 empty response on success.
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#unpin-message>
    pub async fn unsticky(
        channel_id: Snowflake,
        message_id: Snowflake,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest {
            request: Client::new().delete(format!(
                "{}/channels/{}/pins/{}",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }

    /// Returns a specific message object in the channel.
    /// If operating on a guild channel, this endpoint requires the `READ_MESSAGE_HISTORY` permission to be present on the current user.
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#get-message>
    pub async fn get(
        channel_id: Snowflake,
        message_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let chorus_request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/channels/{}/messages/{}",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        chorus_request
            .send_and_deserialize_response::<Message>(user)
            .await
    }

    /// Posts a greet message to a channel. This endpoint requires the channel is a DM channel or you reply to a system message.
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#create-greet-message>
    pub async fn create_greet(
        channel_id: Snowflake,
        schema: CreateGreetMessage,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/channels/{}/messages/greet",
                    user.belongs_to.read().unwrap().urls.api,
                    channel_id,
                ))
                .json(&schema),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request.send_and_deserialize_response::<Message>(user).await
    }

    /// Sets the channel's latest acknowledged message (marks a message as read) for the current user.
    /// The message ID parameter does not need to be a valid message ID, but it must be a valid snowflake.
    /// If the message ID is being set to a message sent prior to the latest acknowledged one,
    /// manual should be true or the resulting read state update should be ignored by clients (but is still saved), resulting in undefined behavior.
    /// In this case, mention_count should also be set to the amount of mentions unacknowledged as it is not automatically calculated by Discord.
    ///
    /// Returns an optional token, which can be used as the new `ack` token for following `ack`s.
    ///
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#acknowledge-message>
    pub async fn acknowledge(
        channel_id: Snowflake,
        message_id: Snowflake,
        schema: MessageAck,
        user: &mut ChorusUser,
    ) -> ChorusResult<Option<String>> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/channels/{}/messages/{}/ack",
                    user.belongs_to.read().unwrap().urls.api,
                    channel_id,
                    message_id
                ))
                .json(&schema),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request
            .send_and_deserialize_response::<Option<String>>(user)
            .await
    }

    /// Crossposts a message in a News Channel to following channels.
    /// This endpoint requires the `SEND_MESSAGES` permission, if the current user sent the message,
    /// or additionally the `MANAGE_MESSAGES` permission, for all other messages, to be present for the current user.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/message#crosspost-message>
    pub async fn crosspost(
        channel_id: Snowflake,
        message_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let request = ChorusRequest {
            request: Client::new().post(format!(
                "{}/channels/{}/messages/{}/crosspost",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request.send_and_deserialize_response::<Message>(user).await
    }

    /// Hides a message from the feed of the guild the channel belongs to. Returns a 204 empty response on success.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/message#hide-message-from-guild-feed>
    pub async fn hide_from_guild_feed(
        channel_id: Snowflake,
        message_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/hide-guild-feed",
            user.belongs_to.read().unwrap().urls.api,
            channel_id,
            message_id
        );

        let request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }

    /// Edits a previously sent message. All fields can be edited by the original message author.
    /// Other users can only edit flags and only if they have the MANAGE_MESSAGES permission in the corresponding channel.
    /// When specifying flags, ensure to include all previously set flags/bits in addition to ones that you are modifying.
    /// When the content field is edited, the mentions array in the message object will be reconstructed from scratch based on the new content.
    /// The allowed_mentions field of the edit request controls how this happens.
    /// If there is no explicit allowed_mentions in the edit request, the content will be parsed with default allowances, that is,
    /// without regard to whether or not an allowed_mentions was present in the request that originally created the message.
    ///
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#edit-message>
    pub async fn modify(
        channel_id: Snowflake,
        message_id: Snowflake,
        schema: MessageModifySchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let url = format!(
            "{}/channels/{}/messages/{}",
            user.belongs_to.read().unwrap().urls.api,
            channel_id,
            message_id
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request.send_and_deserialize_response::<Message>(user).await
    }

    /// Deletes a message. If operating on a guild channel and trying to delete a message that was not sent by the current user,
    /// this endpoint requires the `MANAGE_MESSAGES` permission. Returns a 204 empty response on success.
    pub async fn delete(
        channel_id: Snowflake,
        message_id: Snowflake,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}",
            user.belongs_to.read().unwrap().urls.api,
            channel_id,
            message_id
        );

        let request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }

    /// Deletes multiple messages in a single request. This endpoint can only be used on guild channels and requires the MANAGE_MESSAGES permission.
    /// Returns a 204 empty response on success.
    ///
    /// **This endpoint will not delete messages older than 2 weeks, and will fail if any message provided is older than that or if any duplicate message IDs are provided.**
    ///
    /// **This endpoint is not usable by user accounts.** (At least according to Discord.com. Spacebar behaviour may differ.)
    ///
    /// # Reference:
    /// See: <https://docs.discord.food/resources/message#bulk-delete-messages>
    pub async fn bulk_delete(
        channel_id: Snowflake,
        messages: Vec<Snowflake>,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        if messages.len() < 2 {
            return Err(ChorusError::InvalidArguments {
                error: "`messages` must contain at least 2 entries.".to_string(),
            });
        }

        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/channels/{}/messages/bulk-delete",
                    user.belongs_to.read().unwrap().urls.api,
                    channel_id,
                ))
                .json(&messages),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }

    /// Acknowledges the currently pinned messages in a channel. Returns a 204 empty response on success.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/message#acknowledge-pinned-messages>
    pub async fn acknowledge_pinned(
        channel_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest {
            request: Client::new().post(format!(
                "{}/channels/{}/pins/ack",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
            )),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }
}

impl ChorusUser {
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// # Notes
    /// Shorthand call for [`Message::send`]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/message#create-message>
    pub async fn send_message(
        &mut self,
        message: MessageSendSchema,
        channel_id: Snowflake,
    ) -> ChorusResult<Message> {
        Message::send(self, channel_id, message).await
    }
}

impl Channel {
    /// Returns messages without the reactions key that match a search query in the channel.
    /// The messages that are direct results will have an extra hit key set to true.
    /// If operating on a guild channel, this endpoint requires the `READ_MESSAGE_HISTORY`
    /// permission to be present on the current user.
    ///
    /// If the guild/channel you are searching is not yet indexed, the endpoint will return a 202 accepted response.
    /// In this case, the method will return a [`ChorusError::InvalidResponse`] error.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/message#search-messages>
    pub async fn search_messages(
        channel_id: Snowflake,
        query: MessageSearchQuery,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<Message>> {
        Message::search(MessageSearchEndpoint::Channel(channel_id), query, user).await
    }
}
