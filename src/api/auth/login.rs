use std::cell::RefCell;
use std::rc::Rc;

use reqwest::Client;
use serde_json::{from_str, json};

use crate::api::limits::LimitType;
use crate::errors::ChorusLibError;
use crate::instance::{Instance, UserMeta};
use crate::limit::LimitedRequester;
use crate::types::{ErrorResponse, LoginResult, LoginSchema};

impl Instance {
    pub async fn login_account(
        &mut self,
        login_schema: &LoginSchema,
    ) -> Result<UserMeta, ChorusLibError> {
        let json_schema = json!(login_schema);
        let client = Client::new();
        let endpoint_url = self.urls.api.clone() + "/auth/login";
        let request_builder = client.post(endpoint_url).body(json_schema.to_string());
        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since login is an instance wide limit), which is why we are just cloning the
        // instances' limits to pass them on as user_rate_limits later.
        let mut cloned_limits = self.limits.clone();
        let response = LimitedRequester::send_request(
            request_builder,
            LimitType::AuthRegister,
            self,
            &mut cloned_limits,
        )
        .await;
        if response.is_err() {
            return Err(ChorusLibError::NoResponse);
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
            return Err(ChorusLibError::InvalidFormBodyError { error_type, error });
        }

        let cloned_limits = self.limits.clone();
        let login_result: LoginResult = from_str(&response_text_string).unwrap();
        let object = self
            .get_user(login_result.token.clone(), None)
            .await
            .unwrap();
        let user = UserMeta::new(
            Rc::new(RefCell::new(self.clone())),
            login_result.token,
            cloned_limits,
            login_result.settings,
            object,
        );

        Ok(user)
    }
}
