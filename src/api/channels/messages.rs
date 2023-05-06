pub mod messages {
    use crate::api::limits::Limits;
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
            message: &mut crate::api::schemas::MessageSendSchema,
            files: Option<Vec<PartialDiscordFileAttachment>>,
            token: &String,
            user: &mut User<'a>,
            requester: &mut LimitedRequester,
        ) {
            let user_limits = &mut user.limits;
            let instance_limits = &mut user.belongs_to.limits;
        }
    }

    impl<'a> User<'a> {
        pub async fn send_message(
            &mut self,
            mut message: &mut crate::api::schemas::MessageSendSchema,
            files: Option<Vec<PartialDiscordFileAttachment>>,
        ) {
            let token = self.token().clone();
            Message::send(
                &self.belongs_to.urls.get_api().to_string(),
                &mut message,
                files,
                &token,
                self,
                &mut LimitedRequester::new().await,
            )
            .await;
        }
    }
}
