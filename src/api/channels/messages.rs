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
            message: &mut Message,
            files: Option<Vec<PartialDiscordFileAttachment>>,
            user: &mut User<'a>,
            limits_instance: &mut Limits,
            requester: &mut LimitedRequester,
        ) {
            let token = user.token();
            let mut limits = &mut user.rate_limits;
        }
    }

    impl<'a> User<'a> {
        pub async fn send_message() {}
    }
}
