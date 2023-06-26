use std::{cell::RefCell, rc::Rc};

use reqwest::Client;
use serde_json::{from_str, json};

use crate::{
    api::limits::LimitType,
    errors::{ChorusLibError, ChorusResult},
    instance::{Instance, Token, UserMeta},
    limit::LimitedRequester,
    types::{ErrorResponse, RegisterSchema},
};

impl Instance {
    /// Registers a new user on the Spacebar server.
    ///
    /// # Arguments
    ///
    /// * `register_schema` - The [`RegisterSchema`] that contains all the information that is needed to register a new user.
    ///
    /// # Errors
    ///
    /// * [`ChorusLibError`] - If the server does not respond.
    pub async fn register_account(
        &mut self,
        register_schema: &RegisterSchema,
    ) -> ChorusResult<UserMeta> {
        let json_schema = json!(register_schema);
        let client = Client::new();
        let endpoint_url = self.urls.api.clone() + "/auth/register";
        let request_builder = client.post(endpoint_url).body(json_schema.to_string());
        // We do not have a user yet, and the UserRateLimits will not be affected by a login
        // request (since register is an instance wide limit), which is why we are just cloning
        // the instances' limits to pass them on as user_rate_limits later.
        let mut cloned_limits = self.limits.clone();
        let response = LimitedRequester::send_request(
            request_builder,
            LimitType::AuthRegister,
            self,
            &mut cloned_limits,
        )
        .await?;

        let status = response.status();
        let response_text = response.text().await.unwrap();
        let token = from_str::<Token>(&response_text).unwrap();
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
            return Err(ChorusLibError::InvalidFormBodyError { error_type, error });
        }
        let user_object = self.get_user(token.clone(), None).await.unwrap();
        let settings = UserMeta::get_settings(&token, &self.urls.api.clone(), self).await?;
        let user = UserMeta::new(
            Rc::new(RefCell::new(self.clone())),
            token.clone(),
            cloned_limits,
            settings,
            user_object,
        );
        Ok(user)
    }
}
