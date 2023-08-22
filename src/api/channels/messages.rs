use http::header::CONTENT_DISPOSITION;
use http::HeaderMap;
use reqwest::{multipart, Client};
use serde_json::{from_str, to_string, to_value};

use crate::api::LimitType;
use crate::errors::{ChorusError, ChorusResult};
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    Message, MessageSearchEndpoint, MessageSearchQuery, MessageSendSchema, Snowflake,
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
                .body(to_string(&query).unwrap()),
        };
        let result = request.send_request(user).await?;
        let result_text = result.text().await.unwrap();
        if let Ok(response) = from_str::<Vec<Message>>(&result_text) {
            return Ok(response);
        }
        if to_value(result_text.clone()).is_err() {
            return Err(search_error(result_text));
        }
        let result_value = to_value(result_text.clone()).unwrap();
        if !result_value.is_object() {
            return Err(search_error(result_text));
        }
        let value_map = result_value.as_object().unwrap();
        if !value_map.contains_key("code") || !value_map.contains_key("retry_after") {
            return Err(search_error(result_text));
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
