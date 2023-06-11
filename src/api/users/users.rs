use reqwest::Client;
use serde_json::{from_str, to_string};

use crate::{
    api::{deserialize_response, handle_request_as_option, limits::Limits},
    errors::ChorusLibError,
    instance::{Instance, UserMeta},
    limit::LimitedRequester,
    types::{User, UserModifySchema, UserSettings},
};

impl UserMeta {
    /**
    Get a user object by id, or get the current user.
    # Arguments
    * `token` - A valid access token for the API.
    * `url_api` - The URL to the API.
    * `id` - The id of the user that will be retrieved. If this is None, the current user will be retrieved.
    * `instance_limits` - The [`Limits`] of the instance.
    # Errors
    * [`ChorusLibError`] - If the request fails.
     */
    pub async fn get(user: &mut UserMeta, id: Option<&String>) -> Result<User, ChorusLibError> {
        User::get(user, id).await
    }

    pub async fn get_settings(
        token: &String,
        url_api: &String,
        instance_limits: &mut Limits,
    ) -> Result<UserSettings, ChorusLibError> {
        User::get_settings(token, url_api, instance_limits).await
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
    pub async fn modify(
        &mut self,
        modify_schema: UserModifySchema,
    ) -> Result<User, ChorusLibError> {
        if modify_schema.new_password.is_some()
            || modify_schema.email.is_some()
            || modify_schema.code.is_some()
        {
            return Err(ChorusLibError::PasswordRequiredError);
        }
        let request = Client::new()
            .patch(format!(
                "{}/users/@me/",
                self.belongs_to.borrow_mut().urls.get_api()
            ))
            .body(to_string(&modify_schema).unwrap())
            .bearer_auth(self.token());
        let user_updated =
            deserialize_response::<User>(request, self, crate::api::limits::LimitType::Ip)
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
    /// Returns `None` if the user was successfully deleted, or an `ChorusLibError` if an error occurred.
    pub async fn delete(mut self) -> Option<ChorusLibError> {
        let belongs_to = self.belongs_to.borrow();
        let request = Client::new()
            .post(format!("{}/users/@me/delete/", belongs_to.urls.get_api()))
            .bearer_auth(self.token());
        drop(belongs_to);
        handle_request_as_option(request, &mut self, crate::api::limits::LimitType::Ip).await
    }
}

impl User {
    pub async fn get(user: &mut UserMeta, id: Option<&String>) -> Result<User, ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        User::_get(
            &user.token(),
            &format!("{}", belongs_to.urls.get_api()),
            &mut belongs_to.limits,
            id,
        )
        .await
    }

    async fn _get(
        token: &str,
        url_api: &str,
        limits_instance: &mut Limits,
        id: Option<&String>,
    ) -> Result<User, ChorusLibError> {
        let url: String;
        if id.is_none() {
            url = format!("{}/users/@me/", url_api);
        } else {
            url = format!("{}/users/{}", url_api, id.unwrap());
        }
        let request = reqwest::Client::new().get(url).bearer_auth(token);
        let mut requester = crate::limit::LimitedRequester::new().await;
        let mut cloned_limits = limits_instance.clone();
        match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Ip,
                limits_instance,
                &mut cloned_limits,
            )
            .await
        {
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
        instance_limits: &mut Limits,
    ) -> Result<UserSettings, ChorusLibError> {
        let request: reqwest::RequestBuilder = Client::new()
            .get(format!("{}/users/@me/settings/", url_api))
            .bearer_auth(token);
        let mut cloned_limits = instance_limits.clone();
        let mut requester = crate::limit::LimitedRequester::new().await;
        match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Ip,
                instance_limits,
                &mut cloned_limits,
            )
            .await
        {
            Ok(result) => Ok(serde_json::from_str(&result.text().await.unwrap()).unwrap()),
            Err(e) => Err(e),
        }
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
    pub async fn get_user(
        &mut self,
        token: String,
        id: Option<&String>,
    ) -> Result<User, ChorusLibError> {
        User::_get(
            &token,
            &self.urls.get_api().to_string(),
            &mut self.limits,
            id,
        )
        .await
    }
}
