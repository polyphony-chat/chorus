pub mod messages {
    use reqwest::Client;
    use serde_json::to_string;

    
    use crate::api::types::{Message, PartialDiscordFileAttachment, User};
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
                return Err(crate::errors::InstanceServerError::InvalidFormBodyError {
                    error_type: "Not implemented".to_string(),
                    error: "Not implemented".to_string(),
                });
            }
        }
    }

    impl<'a> User<'a> {
        pub async fn send_message(
            &mut self,
            mut message: &mut crate::api::schemas::MessageSendSchema,
            channel_id: &String,
            files: Option<Vec<PartialDiscordFileAttachment>>,
        ) -> Result<reqwest::Response, crate::errors::InstanceServerError> {
            let token = self.token().clone();
            Message::send(
                &self.belongs_to.urls.get_api().to_string(),
                channel_id,
                &mut message,
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
        api::{AuthUsername, LoginSchema, MessageSendSchema, UserObject},
        instance::Instance,
        limit::LimitedRequester,
    };

    use super::*;

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
        let settings = login_result.settings;
        let limits = instance.limits.clone();
        let mut user =
            crate::api::types::User::new(true, &mut instance, token, limits, settings, None);
        let response = user
            .send_message(&mut message, &channel_id, None)
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
