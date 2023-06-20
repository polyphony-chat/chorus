use http::header::CONTENT_DISPOSITION;
use http::HeaderMap;
use reqwest::{multipart, Client};
use serde_json::to_string;

use crate::api::deserialize_response;
use crate::instance::UserMeta;
use crate::types::{Message, MessageSendSchema, PartialDiscordFileAttachment};

impl Message {
    /**
    Sends a message to the Spacebar server.
    # Arguments
    * `url_api` - The URL of the Spacebar server's API.
    * `message` - The [`Message`] that will be sent to the Spacebar server.
    * `limits_user` - The [`Limits`] of the user.
    * `limits_instance` - The [`Limits`] of the instance.
    * `requester` - The [`LimitedRequester`] that will be used to make requests to the Spacebar server.
    # Errors
    * [`ChorusLibError`] - If the message cannot be sent.
     */
    pub async fn send(
        user: &mut UserMeta,
        channel_id: String,
        message: &mut MessageSendSchema,
        files: Option<Vec<PartialDiscordFileAttachment>>,
    ) -> Result<Message, crate::errors::ChorusLibError> {
        let url_api = user.belongs_to.borrow().urls.api.clone();

        if files.is_none() {
            let request = Client::new()
                .post(format!("{}/channels/{}/messages/", url_api, channel_id))
                .bearer_auth(user.token())
                .body(to_string(message).unwrap());
            deserialize_response::<Message>(request, user, crate::api::limits::LimitType::Channel)
                .await
        } else {
            for (index, attachment) in message.attachments.iter_mut().enumerate() {
                attachment.get_mut(index).unwrap().set_id(index as i16);
            }
            let mut form = reqwest::multipart::Form::new();
            let payload_json = to_string(message).unwrap();
            let payload_field = reqwest::multipart::Part::text(payload_json);

            form = form.part("payload_json", payload_field);

            for (index, attachment) in files.unwrap().into_iter().enumerate() {
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

            let request = Client::new()
                .post(format!("{}/channels/{}/messages/", url_api, channel_id))
                .bearer_auth(user.token())
                .multipart(form);

            deserialize_response::<Message>(request, user, crate::api::limits::LimitType::Channel)
                .await
        }
    }
}

impl UserMeta {
    /// Shorthand call for Message::send()
    /**
    Sends a message to the Spacebar server.
    # Arguments
    * `url_api` - The URL of the Spacebar server's API.
    * `message` - The [`Message`] that will be sent to the Spacebar server.
    * `limits_user` - The [`Limits`] of the user.
    * `limits_instance` - The [`Limits`] of the instance.
    * `requester` - The [`LimitedRequester`] that will be used to make requests to the Spacebar server.
    # Errors
    * [`ChorusLibError`] - If the message cannot be sent.
     */
    pub async fn send_message(
        &mut self,
        message: &mut MessageSendSchema,
        channel_id: String,
        files: Option<Vec<PartialDiscordFileAttachment>>,
    ) -> Result<Message, crate::errors::ChorusLibError> {
        Message::send(self, channel_id, message, files).await
    }
}
