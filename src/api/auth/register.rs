pub mod register {
    use custom_error::custom_error;
    use reqwest::Client;
    use serde_json::json;

    use crate::{
        api::{limits::LimitType, schemas::schemas::RegisterSchema},
        errors::InstanceServerError,
        instance::{Instance, Token},
    };

    impl Instance {
        /**
        Registers a new user on the Spacebar server.
        # Arguments
        * `register_schema` - The [`RegisterSchema`] that contains all the information that is needed to register a new user.
        # Errors
        * [`InstanceServerError`] - If the server does not respond.
         */
        pub async fn register(
            &mut self,
            register_schema: &RegisterSchema,
        ) -> Result<Token, InstanceServerError> {
            let json_schema = json!(register_schema);
            let limited_requester = &mut self.requester;
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/auth/register";
            let request_builder = client.post(endpoint_url).body(json_schema.to_string());
            let response = limited_requester
                .send_request(request_builder, LimitType::AuthRegister)
                .await;
            if response.is_none() {
                return Err(InstanceServerError::NoResponse);
            }
            let token = match response.unwrap().text().await {
                Ok(token) => token,
                Err(_) => return Err(InstanceServerError::NoResponse),
            };
            return Ok(Token { token });
        }
    }
}
