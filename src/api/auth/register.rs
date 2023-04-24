pub mod register {
    use reqwest::Client;
    use serde_json::json;

    use crate::{
        api::{
            limits::LimitType,
            schemas::schemas::{ErrorResponse, RegisterSchema},
        },
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
        pub async fn register_account(
            &mut self,
            register_schema: &RegisterSchema,
        ) -> Result<Token, InstanceServerError> {
            let json_schema = json!(register_schema);
            let limited_requester = &mut self.requester;
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/auth/register";
            let request_builder = client.post(endpoint_url).body(json_schema.to_string());
            let response = limited_requester
                .send_request(request_builder, LimitType::AuthRegister, &mut self.limits)
                .await;
            if !response.is_ok() {
                return Err(InstanceServerError::NoResponse);
            }

            let response_unwrap = response.unwrap();
            let status = response_unwrap.status();
            let response_text_string = response_unwrap.text().await.unwrap();
            if status.is_client_error() {
                let json: ErrorResponse = serde_json::from_str(&response_text_string).unwrap();
                let error_type = json.errors.errors.iter().next().unwrap().0.to_owned();
                let mut error = "".to_string();
                for (_, value) in json.errors.errors.iter() {
                    for error_item in value._errors.iter() {
                        error += &(error_item.message.to_string() + " (" + &error_item.code + ")");
                    }
                }
                return Err(InstanceServerError::InvalidFormBodyError { error_type, error });
            }
            return Ok(Token {
                token: response_text_string,
            });
        }
    }
}

#[cfg(test)]
mod test {
    use crate::api::schemas::schemas::{AuthEmail, AuthPassword, AuthUsername, RegisterSchema};
    use crate::errors::InstanceServerError;
    use crate::instance::Instance;
    use crate::limit::LimitedRequester;
    use crate::URLBundle;
    #[tokio::test]
    async fn test_incomplete_registration() {
        let urls = URLBundle::new(
            "http://localhost:3001/api".to_string(),
            "http://localhost:3001".to_string(),
            "http://localhost:3001".to_string(),
        );
        let limited_requester = LimitedRequester::new(urls.get_api().to_string()).await;
        let mut test_instance = Instance::new(urls.clone(), limited_requester)
            .await
            .unwrap();
        let reg = RegisterSchema::new(
            AuthUsername::new("hiiii".to_string()).unwrap(),
            None,
            true,
            Some(AuthEmail::new("me@mail.xy".to_string()).unwrap()),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(
            InstanceServerError::InvalidFormBodyError {
                error_type: "date_of_birth".to_string(),
                error: "This field is required (BASE_TYPE_REQUIRED)".to_string()
            },
            test_instance.register_account(&reg).await.err().unwrap()
        );
    }

    #[tokio::test]
    async fn test_registration() {
        let urls = URLBundle::new(
            "http://localhost:3001/api".to_string(),
            "http://localhost:3001".to_string(),
            "http://localhost:3001".to_string(),
        );
        let limited_requester = LimitedRequester::new(urls.get_api().to_string()).await;
        let mut test_instance = Instance::new(urls.clone(), limited_requester)
            .await
            .unwrap();
        let reg = RegisterSchema::new(
            AuthUsername::new("Hiiii".to_string()).unwrap(),
            Some(AuthPassword::new("mysupersecurepass123!".to_string()).unwrap()),
            true,
            Some(AuthEmail::new("flori@aaaa.xyz".to_string()).unwrap()),
            None,
            None,
            Some("2000-01-01".to_string()),
            None,
            None,
            None,
        )
        .unwrap();
        let token = test_instance.register_account(&reg).await.unwrap().token;
        println!("{}", token);
    }
}
