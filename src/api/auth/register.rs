pub mod register {
    use custom_error::custom_error;
    use reqwest::Client;
    use serde_json::json;

    use crate::{api::schemas::schemas::RegisterSchema, instance::Instance};

    impl Instance {
        pub fn register(&mut self, register_schema: &RegisterSchema) {
            let json_schema = json!(register_schema);
            let limited_requester = &self.requester;
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/auth/register";
            let request_builder = client.post(endpoint_url).body(json_schema.to_string());
            // TODO
        }
    }
}
