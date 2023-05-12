pub mod messages {
    use http::header::CONTENT_DISPOSITION;
    use http::HeaderMap;
    use reqwest::{multipart, Client};
    use serde_json::to_string;

    use crate::api::types::{Message, PartialDiscordFileAttachment, User};
    use crate::errors::InstanceServerError;
    use crate::limit::LimitedRequester;

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
        * [`InstanceServerError`] - If the message cannot be sent.
         */
        pub async fn send<'a>(
            url_api: &String,
            channel_id: &String,
            message: &mut crate::api::schemas::MessageSendSchema,
            files: Option<Vec<PartialDiscordFileAttachment>>,
            token: &String,
            user: &mut User<'a>,
        ) -> Result<reqwest::Response, crate::errors::InstanceServerError> {
            let mut requester = LimitedRequester::new().await;
            let user_rate_limits = &mut user.limits;
            let instance_rate_limits = &mut user.belongs_to.limits;

            if files.is_none() {
                let message_request = Client::new()
                    .post(format!("{}/channels/{}/messages/", url_api, channel_id))
                    .bearer_auth(token)
                    .body(to_string(message).unwrap());
                requester
                    .send_request(
                        message_request,
                        crate::api::limits::LimitType::Channel,
                        instance_rate_limits,
                        user_rate_limits,
                    )
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

                    let mut part = multipart::Part::bytes(attachment_content)
                        .file_name(attachment_filename)
                        .headers(header_map);

                    form = form.part(part_name, part);
                }

                let message_request = Client::new()
                    .post(format!("{}/channels/{}/messages/", url_api, channel_id))
                    .bearer_auth(token)
                    .multipart(form);

                println!("[Request Headers: ] {:?}", message_request);

                requester
                    .send_request(
                        message_request,
                        crate::api::limits::LimitType::Channel,
                        instance_rate_limits,
                        user_rate_limits,
                    )
                    .await
            }
        }
    }

    impl<'a> User<'a> {
        pub async fn send_message(
            &mut self,
            message: &mut crate::api::schemas::MessageSendSchema,
            channel_id: &String,
            files: Option<Vec<PartialDiscordFileAttachment>>,
        ) -> Result<reqwest::Response, crate::errors::InstanceServerError> {
            let token = self.token().clone();
            Message::send(
                &self.belongs_to.urls.get_api().to_string(),
                channel_id,
                message,
                files,
                &token,
                self,
            )
            .await
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        api::{AuthUsername, LoginSchema},
        instance::Instance,
        limit::LimitedRequester,
    };

    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    #[tokio::test]
    async fn send_message() {
        let channel_id = "1104413094102290492".to_string();
        let mut message = crate::api::schemas::MessageSendSchema::new(
            None,
            Some("ashjkdhjksdfgjsdfzjkhsdvhjksdf".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let mut instance = Instance::new(
            crate::URLBundle {
                api: "http://localhost:3001/api".to_string(),
                wss: "ws://localhost:3001/".to_string(),
                cdn: "http://localhost:3001".to_string(),
            },
            LimitedRequester::new().await,
        )
        .await
        .unwrap();
        let login_schema: LoginSchema = LoginSchema::new(
            AuthUsername::new("user1@gmail.com".to_string()).unwrap(),
            "user".to_string(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let login_result = instance.login_account(&login_schema).await.unwrap();
        let token = login_result.token;
        println!("TOKEN: {}", token);
        let settings = login_result.settings;
        let limits = instance.limits.clone();
        let mut user = crate::api::types::User::new(&mut instance, token, limits, settings, None);
        let _ = user
            .send_message(&mut message, &channel_id, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn send_message_attachment() {
        let channel_id = "1104413094102290492".to_string();

        let f = File::open("/drive/hdd1/Pictures/polyphony-chorus8bit.png").unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).unwrap();

        let attachment = crate::api::types::PartialDiscordFileAttachment {
            id: None,
            filename: "test.png".to_string(),
            description: None,
            content_type: None,
            size: None,
            url: None,
            proxy_url: None,
            width: None,
            height: None,
            ephemeral: None,
            duration_secs: None,
            waveform: None,
            content: buffer,
        };

        let mut message = crate::api::schemas::MessageSendSchema::new(
            None,
            Some("trans rights now".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![attachment.clone()]),
        );
        let mut instance = Instance::new(
            crate::URLBundle {
                api: "http://localhost:3001/api".to_string(),
                wss: "ws://localhost:3001/".to_string(),
                cdn: "http://localhost:3001".to_string(),
            },
            LimitedRequester::new().await,
        )
        .await
        .unwrap();
        let login_schema: LoginSchema = LoginSchema::new(
            AuthUsername::new("user1@gmail.com".to_string()).unwrap(),
            "user".to_string(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let login_result = instance.login_account(&login_schema).await.unwrap();
        let token = login_result.token;
        let settings = login_result.settings;
        let limits = instance.limits.clone();
        let mut user = crate::api::types::User::new(&mut instance, token, limits, settings, None);
        let vec_attach = vec![attachment.clone()];
        let _arg = Some(&vec_attach);
        let response = user
            .send_message(&mut message, &channel_id, Some(vec![attachment.clone()]))
            .await
            .unwrap();
        println!("[Response:] {}", response.text().await.unwrap());
    }
}
