use std::{cell::RefCell, rc::Rc};

use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::LimitType,
    errors::{ChorusError, ChorusResult},
    instance::{Instance, UserMeta},
    ratelimiter::ChorusRequest,
    types::{User, UserModifySchema, UserSettings},
};

impl UserMeta {
    /// Get a user object by id, or get the current user.
    ///
    /// # Arguments
    ///
    /// * `token` - A valid access token for the API.
    /// * `url_api` - The URL to the API.
    /// * `id` - The id of the user that will be retrieved. If this is None, the current user will be retrieved.
    /// * `instance_limits` - The [`Limits`] of the instance.
    ///
    /// # Errors
    ///
    /// * [`ChorusLibError`] - If the request fails.
    pub async fn get(user: &mut UserMeta, id: Option<&String>) -> ChorusResult<User> {
        User::get(user, id).await
    }

    pub async fn get_settings(
        token: &String,
        url_api: &String,
        instance: &mut Instance,
    ) -> ChorusResult<UserSettings> {
        User::get_settings(token, url_api, instance).await
    }

    /// Modify the current user's `UserObject`.
    ///
    /// # Arguments
    ///
    /// * `modify_schema` - A `UserModifySchema` object containing the fields to modify.
    ///
    /// # Errors
    ///
    /// Returns an `ChorusLibError` if the request fails or if a password is required but not provided.
    pub async fn modify(&mut self, modify_schema: UserModifySchema) -> ChorusResult<User> {
        if modify_schema.new_password.is_some()
            || modify_schema.email.is_some()
            || modify_schema.code.is_some()
        {
            return Err(ChorusError::PasswordRequired);
        }
        let request = Client::new()
            .patch(format!("{}/users/@me/", self.belongs_to.borrow().urls.api))
            .body(to_string(&modify_schema).unwrap())
            .bearer_auth(self.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        let user_updated = chorus_request
            .deserialize_response::<User>(self)
            .await
            .unwrap();
        let _ = std::mem::replace(&mut self.object, user_updated.clone());
        Ok(user_updated)
    }

    /// Sends a request to the server which deletes the user from the Instance.
    ///
    /// # Arguments
    ///
    /// * `self` - The `User` object to delete.
    ///
    /// # Returns
    ///
    /// Returns `()` if the user was successfully deleted, or a `ChorusLibError` if an error occurred.
    pub async fn delete(mut self) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/delete/",
                self.belongs_to.borrow().urls.api
            ))
            .bearer_auth(self.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.handle_request_as_result(&mut self).await
    }
}

impl User {
    pub async fn get(user: &mut UserMeta, id: Option<&String>) -> ChorusResult<User> {
        let url_api = user.belongs_to.borrow().urls.api.clone();
        let url = if id.is_none() {
            format!("{}/users/@me/", url_api)
        } else {
            format!("{}/users/{}", url_api, id.unwrap())
        };
        let request = reqwest::Client::new().get(url).bearer_auth(user.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        match chorus_request.send_request(user).await {
            Ok(result) => {
                let result_text = result.text().await.unwrap();
                Ok(serde_json::from_str::<User>(&result_text).unwrap())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_settings(
        token: &String,
        url_api: &String,
        instance: &mut Instance,
    ) -> ChorusResult<UserSettings> {
        let request: reqwest::RequestBuilder = Client::new()
            .get(format!("{}/users/@me/settings/", url_api))
            .bearer_auth(token);
        let mut user = UserMeta::shell(Rc::new(RefCell::new(instance.mock().await)), token.clone());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        let result = match chorus_request.send_request(&mut user).await {
            Ok(result) => Ok(serde_json::from_str(&result.text().await.unwrap()).unwrap()),
            Err(e) => Err(e),
        };
        if instance.limits_information.is_some() {
            instance.limits_information.as_mut().unwrap().ratelimits =
                user.belongs_to.borrow().clone_limits_if_some().unwrap();
        }
        result
    }
}

impl Instance {
    /**
    Get a user object by id, or get the current user.
    # Arguments
    * `token` - A valid access token for the API.
    * `id` - The id of the user that will be retrieved. If this is None, the current user will be retrieved.
    # Errors
    * [`ChorusLibError`] - If the request fails.
    # Notes
    This function is a wrapper around [`User::get`].
     */
    pub async fn get_user(&mut self, token: String, id: Option<&String>) -> ChorusResult<User> {
        let mut user = UserMeta::shell(Rc::new(RefCell::new(self.mock().await)), token);
        let result = User::get(&mut user, id).await;
        if self.limits_information.is_some() {
            self.limits_information.as_mut().unwrap().ratelimits =
                user.belongs_to.borrow().clone_limits_if_some().unwrap();
        }
        result
    }
}
