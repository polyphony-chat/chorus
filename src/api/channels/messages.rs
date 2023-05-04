pub mod messages {
    use reqwest::{Client, Response};
    use serde_json::to_string;
    use std::io::Read;

    use crate::api::limits::Limits;
    use crate::api::types::{DiscordFileAttachment, Message, User};
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
        pub async fn send(
            url_api: &String,
            token: &String,
            message: &Message,
            files: Option<Vec<DiscordFileAttachment>>,
            limits_user: &mut Limits,
            limits_instance: &mut Limits,
            requester: &mut LimitedRequester,
        ) -> Result<Response, InstanceServerError> {
            let mut request = Client::new()
                .post(format!(
                    "{}/channels/{}/messages",
                    url_api, message.channel_id
                ))
                .body(to_string(message).unwrap())
                .bearer_auth(token);
            if files.is_some() {}
            match requester
                .send_request(
                    request,
                    crate::api::limits::LimitType::Channel,
                    limits_instance,
                    limits_user,
                )
                .await
            {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        }
    }

    impl<'a> User<'a> {
        pub async fn send_message(
            &mut self,
            message: &Message,
            files: Option<Vec<DiscordFileAttachment>>,
        ) -> Result<Response, InstanceServerError> {
            Message::send(
                &self.belongs_to().urls.get_api().to_string(),
                &self.token(),
                message,
                files,
                self.rate_limits.get_as_mut(),
                &mut self.belongs_to.limits.get_as_mut(),
                &mut LimitedRequester::new().await,
            )
            .await
        }
    }
}
