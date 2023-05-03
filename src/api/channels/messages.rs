pub mod messages {
    use reqwest::Client;
    use serde_json::to_string;

    use crate::api::limits::Limits;
    use crate::api::types::Message;
    use crate::instance::Instance;
    use crate::limit::LimitedRequester;

    impl Message {
        pub async fn send(
            url_api: &String,
            message: &Message,
            limits: &Limits,
            requester: &mut LimitedRequester,
        ) {
            let client = Client::new()
                .post(url_api)
                .body(to_string(message).unwrap())
                .send()
                .await;
        }
    }
}
