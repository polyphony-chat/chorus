use http::header::CONTENT_DISPOSITION;
use http::HeaderMap;
use reqwest::{multipart, Client};
use serde_json::{from_value, to_string, Value};

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::ChorusUser;
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    Channel, CreateGreetMessage, LimitType, Message, MessageAck, MessageModifySchema,
    MessageSearchEndpoint, MessageSearchQuery, MessageSendSchema, Snowflake,
};

impl Message {
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/message#create-message>
    pub async fn send(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        mut message: MessageSendSchema,
    ) -> ChorusResult<Message> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();

        if message.attachments.is_none() {
            let chorus_request = ChorusRequest {
                request: Client::new()
                    .post(format!("{}/channels/{}/messages", url_api, channel_id))
                    .header("Authorization", user.token())
                    .body(to_string(&message).unwrap())
                    .header("Content-Type", "application/json"),
                limit_type: LimitType::Channel(channel_id),
            };
            chorus_request.deserialize_response::<Message>(user).await
        } else {
            for (index, attachment) in message.attachments.iter_mut().enumerate() {
                attachment.get_mut(index).unwrap().set_id(index as i16);
            }
            let mut form = reqwest::multipart::Form::new();
            let payload_json = to_string(&message).unwrap();
            let payload_field = reqwest::multipart::Part::text(payload_json);

            form = form.part("payload_json", payload_field);

            for (index, attachment) in message.attachments.unwrap().into_iter().enumerate() {
                let (attachment_content, current_attachment) = attachment.move_content();
                let (attachment_filename, _) = current_attachment.move_filename();
                let part_name = format!("files[{}]", index);
                let content_disposition = format!(
                    "form-data; name=\"{}\"'; filename=\"{}\"",
                    part_name, &attachment_filename
                );
                let mut header_map = HeaderMap::new();
                header_map.insert(CONTENT_DISPOSITION, content_disposition.parse().unwrap());

                let part = multipart::Part::bytes(attachment_content)
                    .file_name(attachment_filename)
                    .headers(header_map);

                form = form.part(part_name, part);
            }

            let chorus_request = ChorusRequest {
                request: Client::new()
                    .post(format!("{}/channels/{}/messages", url_api, channel_id))
                    .header("Authorization", user.token())
                    .multipart(form),
                limit_type: LimitType::Channel(channel_id),
            };
            chorus_request.deserialize_response::<Message>(user).await
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
    /// See <https://discord-userdoccers.vercel.app/resources/message#search-messages>
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
                .header("Authorization", user.token())
                .header("Content-Type", "application/json")
                .body(to_string(&query).unwrap()),
        };
        let result = request.send_request(user).await?;
        let result_json = result.json::<Value>().await.unwrap();
        if !result_json.is_object() {
            return Err(search_error(result_json.to_string()));
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
            return Err(search_error(result_json.to_string()));
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
    /// See: <https://discord-userdoccers.vercel.app/resources/message#get-pinned-messages>
    pub async fn get_sticky(
        channel_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<Message>> {
        let chorus_request = ChorusRequest::new(
            http::Method::GET,
            format!(
                "{}/channels/{}/pins",
                user.belongs_to.read().unwrap().urls.api,
                channel_id
            )
            .as_str(),
            None,
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        chorus_request
            .deserialize_response::<Vec<Message>>(user)
            .await
    }

    /// Pins a message in a channel. Requires the `MANAGE_MESSAGES` permission. Returns a 204 empty response on success.
    /// The max pinned messages is 50.
    ///
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#pin-message>
    pub async fn sticky(
        channel_id: Snowflake,
        message_id: Snowflake,
        audit_log_reason: Option<&str>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest::new(
            http::Method::PUT,
            format!(
                "{}/channels/{}/pins/{}",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )
            .as_str(),
            None,
            audit_log_reason,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        request.handle_request_as_result(user).await
    }

    /// Unpins a message in a channel. Requires the `MANAGE_MESSAGES` permission. Returns a 204 empty response on success.
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#unpin-message>
    pub async fn unsticky(
        channel_id: Snowflake,
        message_id: Snowflake,
        audit_log_reason: Option<&str>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest::new(
            http::Method::DELETE,
            format!(
                "{}/channels/{}/pins/{}",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )
            .as_str(),
            None,
            audit_log_reason,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        request.handle_request_as_result(user).await
    }

    /// Returns a specific message object in the channel.
    /// If operating on a guild channel, this endpoint requires the `READ_MESSAGE_HISTORY` permission to be present on the current user.
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#get-message>
    pub async fn get(
        channel_id: Snowflake,
        message_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/channels/{}/messages/{}",
                    user.belongs_to.read().unwrap().urls.api,
                    channel_id,
                    message_id
                ))
                .header("Authorization", user.token())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.deserialize_response::<Message>(user).await
    }

    /// Posts a greet message to a channel. This endpoint requires the channel is a DM channel or you reply to a system message.
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#create-greet-message>
    pub async fn create_greet(
        channel_id: Snowflake,
        schema: CreateGreetMessage,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let request = ChorusRequest::new(
            http::Method::POST,
            format!(
                "{}/channels/{}/messages/greet",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
            )
            .as_str(),
            Some(to_string(&schema).unwrap()),
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        request.deserialize_response::<Message>(user).await
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
    /// See: <https://discord-userdoccers.vercel.app/resources/message#acknowledge-message>
    pub async fn acknowledge(
        channel_id: Snowflake,
        message_id: Snowflake,
        schema: MessageAck,
        user: &mut ChorusUser,
    ) -> ChorusResult<Option<String>> {
        let request = ChorusRequest::new(
            http::Method::POST,
            format!(
                "{}/channels/{}/messages/{}/ack",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )
            .as_str(),
            Some(to_string(&schema).unwrap()),
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        request.deserialize_response::<Option<String>>(user).await
    }

    /// Crossposts a message in a News Channel to following channels.
    /// This endpoint requires the `SEND_MESSAGES` permission, if the current user sent the message,
    /// or additionally the `MANAGE_MESSAGES` permission, for all other messages, to be present for the current user.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/message#crosspost-message>
    pub async fn crosspost(
        channel_id: Snowflake,
        message_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Message> {
        let request = ChorusRequest::new(
            http::Method::POST,
            format!(
                "{}/channels/{}/messages/{}/crosspost",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
                message_id
            )
            .as_str(),
            None,
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        request.deserialize_response::<Message>(user).await
    }

    /// Hides a message from the feed of the guild the channel belongs to. Returns a 204 empty response on success.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/message#hide-message-from-guild-feed>
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
        let chorus_request = ChorusRequest::new(
            http::Method::DELETE,
            &url,
            None,
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        chorus_request.handle_request_as_result(user).await
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
    /// See: <https://discord-userdoccers.vercel.app/resources/message#edit-message>
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
        let chorus_request = ChorusRequest::new(
            http::Method::PATCH,
            &url,
            Some(to_string(&schema).unwrap()),
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        chorus_request.deserialize_response::<Message>(user).await
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

        let chorus_request = ChorusRequest::new(
            http::Method::DELETE,
            &url,
            None,
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );

        chorus_request.handle_request_as_result(user).await
    }

    /// Deletes multiple messages in a single request. This endpoint can only be used on guild channels and requires the MANAGE_MESSAGES permission.
    /// Returns a 204 empty response on success.
    ///
    /// **This endpoint will not delete messages older than 2 weeks, and will fail if any message provided is older than that or if any duplicate message IDs are provided.**
    ///
    /// **This endpoint is not usable by user accounts.** (At least according to Discord.com. Spacebar behaviour may differ.)
    ///
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#bulk-delete-messages>
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
        let request = ChorusRequest::new(
            http::Method::POST,
            format!(
                "{}/channels/{}/messages/bulk-delete",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
            )
            .as_str(),
            Some(to_string(&messages).unwrap()),
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );
        request.handle_request_as_result(user).await
    }

    /// Acknowledges the currently pinned messages in a channel. Returns a 204 empty response on success.
    ///
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#acknowledge-pinned-messages>
    pub async fn acknowledge_pinned(
        channel_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let chorus_request = ChorusRequest::new(
            http::Method::POST,
            format!(
                "{}/channels/{}/pins/ack",
                user.belongs_to.read().unwrap().urls.api,
                channel_id,
            )
            .as_str(),
            None,
            None,
            None,
            Some(user),
            LimitType::Channel(channel_id),
        );

        chorus_request.handle_request_as_result(user).await
    }
}

fn search_error(result_text: String) -> ChorusError {
    ChorusError::InvalidResponse {
        error: format!(
            "Got unexpected Response, or Response which is not valid JSON. Response: \n{}",
            result_text
        ),
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
    /// See <https://discord-userdoccers.vercel.app/resources/message#create-message>
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
    /// See <https://discord-userdoccers.vercel.app/resources/message#search-messages>
    pub async fn search_messages(
        channel_id: Snowflake,
        query: MessageSearchQuery,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<Message>> {
        Message::search(MessageSearchEndpoint::Channel(channel_id), query, user).await
    }
}
