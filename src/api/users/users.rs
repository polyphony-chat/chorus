use reqwest::Client;
use serde_json::{from_str, to_string};

use crate::{
    api::{
        limits::Limits,
        types::{User, UserObject},
        UserModifySchema, UserSettings,
    },
    errors::InstanceServerError,
    instance::Instance,
    limit::LimitedRequester,
};

impl User {
    /**
    Get a user object by id, or get the current user.
    # Arguments
    * `token` - A valid access token for the API.
    * `url_api` - The URL to the API.
    * `id` - The id of the user that will be retrieved. If this is None, the current user will be retrieved.
    * `instance_limits` - The [`Limits`] of the instance.
    # Errors
    * [`InstanceServerError`] - If the request fails.
     */
    pub async fn get(
        token: &String,
        url_api: &String,
        id: Option<&String>,
        instance_limits: &mut Limits,
    ) -> Result<UserObject, InstanceServerError> {
        let url: String;
        if id.is_none() {
            url = format!("{}/users/@me/", url_api);
        } else {
            url = format!("{}/users/{}", url_api, id.unwrap());
        }
        let request = reqwest::Client::new().get(url).bearer_auth(token);
        let mut requester = crate::limit::LimitedRequester::new().await;
        let mut cloned_limits = instance_limits.clone();
        match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Ip,
                instance_limits,
                &mut cloned_limits,
            )
            .await
        {
            Ok(result) => {
                let result_text = result.text().await.unwrap();
                Ok(serde_json::from_str::<UserObject>(&result_text).unwrap())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_settings(
        token: &String,
        url_api: &String,
        instance_limits: &mut Limits,
    ) -> Result<UserSettings, InstanceServerError> {
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

    /// Modify the current user's `UserObject`.
    ///
    /// # Arguments
    ///
    /// * `modify_schema` - A `UserModifySchema` object containing the fields to modify.
    ///
    /// # Errors
    ///
    /// Returns an `InstanceServerError` if the request fails or if a password is required but not provided.
    pub async fn modify(
        &mut self,
        modify_schema: UserModifySchema,
    ) -> Result<UserObject, InstanceServerError> {
        if modify_schema.new_password.is_some()
            || modify_schema.email.is_some()
            || modify_schema.code.is_some()
        {
            return Err(InstanceServerError::PasswordRequiredError);
        }
        let request = Client::new()
            .patch(format!(
                "{}/users/@me/",
                self.belongs_to.borrow_mut().urls.get_api()
            ))
            .body(to_string(&modify_schema).unwrap())
            .bearer_auth(self.token());
        let result = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Global,
                &mut self.belongs_to.borrow_mut().limits,
                &mut self.limits,
            )
            .await
        {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let user_updated: UserObject = from_str(&result.text().await.unwrap()).unwrap();
        let _ = std::mem::replace(
            &mut self.object.as_mut().unwrap(),
            &mut user_updated.clone(),
        );
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
    /// Returns `None` if the user was successfully deleted, or an `InstanceServerError` if an error occurred.
    pub async fn delete(mut self) -> Option<InstanceServerError> {
        let mut belongs_to = self.belongs_to.borrow_mut();
        let request = Client::new()
            .post(format!("{}/users/@me/delete/", belongs_to.urls.get_api()))
            .bearer_auth(self.token);
        match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Global,
                &mut belongs_to.limits,
                &mut self.limits,
            )
            .await
        {
            Ok(_) => None,
            Err(e) => Some(e),
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
    * [`InstanceServerError`] - If the request fails.
    # Notes
    This function is a wrapper around [`User::get`].
     */
    pub async fn get_user(
        &mut self,
        token: String,
        id: Option<&String>,
    ) -> Result<UserObject, InstanceServerError> {
        User::get(
            &token,
            &self.urls.get_api().to_string(),
            id,
            &mut self.limits,
        )
        .await
    }
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn get_user() {}
}
