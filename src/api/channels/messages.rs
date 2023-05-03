pub mod messages {
    use reqwest::{Client, Response};
    use serde_json::to_string;

    use crate::api::limits::Limits;
    use crate::api::types::Message;
    use crate::api::User;
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
            message: &Message,
            limits_user: &mut Limits,
            limits_instance: &mut Limits,
            requester: &mut LimitedRequester,
        ) -> Result<Response, InstanceServerError> {
            let request = Client::new()
                .post(format!(
                    "{}/channels/{}/messages",
                    url_api, message.channel_id
                ))
                .body(to_string(message).unwrap());
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

    impl<'a> User<'a> {}
}
