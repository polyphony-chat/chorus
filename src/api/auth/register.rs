pub mod register {
    use reqwest::Client;
    use serde_json::{from_str, json};

    use crate::{
        api::{limits::LimitType, schemas::RegisterSchema, types::ErrorResponse, Token},
        errors::InstanceServerError,
        instance::Instance,
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
        ) -> Result<crate::api::types::User, InstanceServerError> {
            let json_schema = json!(register_schema);
            let limited_requester = &mut self.requester;
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/auth/register";
            let request_builder = client.post(endpoint_url).body(json_schema.to_string());
            // We do not have a user yet, and the UserRateLimits will not be affected by a login
            // request (since register is an instance wide limit), which is why we are just cloning
            // the instances' limits to pass them on as user_rate_limits later.
            let mut cloned_limits = self.limits.clone();
            let response = limited_requester
                .send_request(
                    request_builder,
                    LimitType::AuthRegister,
                    &mut self.limits,
                    &mut cloned_limits,
                )
                .await;
            if response.is_err() {
                return Err(InstanceServerError::NoResponse);
            }

            let response_unwrap = response.unwrap();
            let status = response_unwrap.status();
            let response_unwrap_text = response_unwrap.text().await.unwrap();
            println!("{}", response_unwrap_text);
            let token = from_str::<Token>(&response_unwrap_text).unwrap();
            let token = token.token;
            if status.is_client_error() {
                let json: ErrorResponse = serde_json::from_str(&token).unwrap();
                let error_type = json.errors.errors.iter().next().unwrap().0.to_owned();
                let mut error = "".to_string();
                for (_, value) in json.errors.errors.iter() {
                    for error_item in value._errors.iter() {
                        error += &(error_item.message.to_string() + " (" + &error_item.code + ")");
                    }
                }
                return Err(InstanceServerError::InvalidFormBodyError { error_type, error });
            }
            let user_object = self.get_user(token.clone(), None).await.unwrap();
            let settings = crate::api::types::User::get_settings(
                &token,
                &self.urls.get_api().to_string(),
                &mut self.limits,
            )
            .await
            .unwrap();
            let user: crate::api::types::User = crate::api::types::User::new(
                self,
                token.clone(),
                cloned_limits,
                settings,
                Some(user_object),
            );
            Ok(user)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::api::schemas::{AuthEmail, AuthPassword, AuthUsername, RegisterSchema};
    use crate::instance::Instance;
    use crate::limit::LimitedRequester;
    use crate::URLBundle;

    #[tokio::test]
    async fn test_registration() {
        let urls = URLBundle::new(
            "http://localhost:3001/api".to_string(),
            "http://localhost:3001".to_string(),
            "http://localhost:3001".to_string(),
        );
        let limited_requester = LimitedRequester::new().await;
        let mut test_instance = Instance::new(urls.clone(), limited_requester)
            .await
            .unwrap();
        let reg = RegisterSchema::new(
            AuthUsername::new("Hiiii".to_string()).unwrap(),
            Some(AuthPassword::new("mysupersecurepass123!".to_string()).unwrap()),
            true,
            Some(AuthEmail::new("four5@aaaa.xyz".to_string()).unwrap()),
            None,
            None,
            Some("2000-01-01".to_string()),
            None,
            None,
            None,
        )
        .unwrap();
        let token = test_instance.register_account(&reg).await.unwrap().token;
    }
}
