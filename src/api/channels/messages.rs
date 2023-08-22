use http::header::CONTENT_DISPOSITION;
use http::HeaderMap;
use reqwest::{multipart, Client};
use serde_json::{from_value, to_string, Value};

use crate::api::LimitType;
use crate::errors::{ChorusError, ChorusResult};
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    Channel, Message, MessageSearchEndpoint, MessageSearchQuery, MessageSendSchema, Snowflake,
};

impl Message {
    /// Sends a message in the channel with the provided channel_id.
    /// Returns the sent message.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/message#create-message>
    pub async fn send(
        user: &mut UserMeta,
        channel_id: Snowflake,
        mut message: MessageSendSchema,
    ) -> ChorusResult<Message> {
        let url_api = user.belongs_to.borrow().urls.api.clone();

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
        user: &mut UserMeta,
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
                    &user.belongs_to.borrow().urls.api,
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
        user: &mut UserMeta,
    ) -> ChorusResult<Vec<Message>> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/channels/{}/pins",
                    user.belongs_to.borrow().urls.api,
                    channel_id
                ))
                .header("Authorization", user.token())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Channel(channel_id),
        };
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
        user: &mut UserMeta,
    ) -> ChorusResult<()> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .put(format!(
                    "{}/channels/{}/pins/{}",
                    user.belongs_to.borrow().urls.api,
                    channel_id,
                    message_id
                ))
                .header("Authorization", user.token())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Unpins a message in a channel. Requires the `MANAGE_MESSAGES` permission. Returns a 204 empty response on success.
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/message#unpin-message>
    pub async fn unsticky(
        channel_id: Snowflake,
        message_id: Snowflake,
        user: &mut UserMeta,
    ) -> ChorusResult<()> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .delete(format!(
                    "{}/channels/{}/pins/{}",
                    user.belongs_to.borrow().urls.api,
                    channel_id,
                    message_id
                ))
                .header("Authorization", user.token())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Channel(channel_id),
        };
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

impl UserMeta {
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
        user: &mut UserMeta,
    ) -> ChorusResult<Vec<Message>> {
        Message::search(MessageSearchEndpoint::Channel(channel_id), query, user).await
    }
}
