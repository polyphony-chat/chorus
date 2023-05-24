pub mod login {
    use std::cell::RefCell;
    use std::rc::Rc;

    use reqwest::Client;
    use serde_json::{from_str, json};

    use crate::api::limits::LimitType;
    use crate::api::schemas::LoginSchema;
    use crate::api::types::{ErrorResponse, LoginResult};
    use crate::errors::InstanceServerError;
    use crate::instance::Instance;
    use crate::limit::LimitedRequester;

    impl Instance {
        pub async fn login_account(
            &mut self,
            login_schema: &LoginSchema,
        ) -> Result<crate::api::types::User, InstanceServerError> {
            let mut requester = LimitedRequester::new().await;
            let json_schema = json!(login_schema);
            let client = Client::new();
            let endpoint_url = self.urls.get_api().to_string() + "/auth/login";
            let request_builder = client.post(endpoint_url).body(json_schema.to_string());
            // We do not have a user yet, and the UserRateLimits will not be affected by a login
            // request (since login is an instance wide limit), which is why we are just cloning the
            // instances' limits to pass them on as user_rate_limits later.
            let mut cloned_limits = self.limits.clone();
            let response = requester
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

            let cloned_limits = self.limits.clone();
            let login_result: LoginResult = from_str(&response_text_string).unwrap();
            let object = self
                .get_user(login_result.token.clone(), None)
                .await
                .unwrap();
            let user = crate::api::types::User::new(
                Rc::new(RefCell::new(self.clone())),
                login_result.token,
                cloned_limits,
                login_result.settings,
                Some(object),
            );

            Ok(user)
        }
    }
}

/*#[cfg(test)]
mod test {
    use crate::api::schemas::schemas::{
        AuthEmail, AuthPassword, AuthUsername, LoginSchema, RegisterSchema,
    };
    use crate::instance::Instance;
    use crate::limit::LimitedRequester;
    use crate::URLBundle;

    #[tokio::test]
    async fn test_login() {
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
            AuthUsername::new("TestAccount".to_string()).unwrap(),
            Some(AuthPassword::new("transrights".to_string()).unwrap()),
            true,
            Some(AuthEmail::new("apiauthlogin1@testlogin.xyz".to_string()).unwrap()),
            None,
            None,
            Some("2000-01-01".to_string()),
            None,
            None,
            None,
        )
        .unwrap();
        test_instance.register_account(&reg).await.unwrap().token;

        let login_schema = LoginSchema::new(
            AuthUsername::new("apiauthlogin1@testlogin.xyz".to_string()).unwrap(),
            "transrights".to_string(),
            Some(false),
            None,
            None,
            None,
        );

        let login_result = test_instance
            .login_account(&login_schema.unwrap())
            .await
            .unwrap();
    }
}*/
