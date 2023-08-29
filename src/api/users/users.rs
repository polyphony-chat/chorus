use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::{ChorusUser, Instance},
    ratelimiter::ChorusRequest,
    types::{LimitType, User, UserModifySchema, UserSettings},
};

impl ChorusUser {
    /// Gets a user by id, or if the id is None, gets the current user.
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get`].
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#get-user> and
    /// <https://discord-userdoccers.vercel.app/resources/user#get-current-user>
    pub async fn get_user(&mut self, id: Option<&String>) -> ChorusResult<User> {
        User::get(self, id).await
    }

    /// Gets the user's settings.
    ///
    /// # Notes
    /// This functions is a wrapper around [`User::get_settings`].
    pub async fn get_settings(
        token: &String,
        url_api: &String,
        instance: &mut Instance,
    ) -> ChorusResult<UserSettings> {
        User::get_settings(token, url_api, instance).await
    }

    /// Modifies the current user's representation. (See [`User`])
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#modify-current-user>
    pub async fn modify(&mut self, modify_schema: UserModifySchema) -> ChorusResult<User> {
        if modify_schema.new_password.is_some()
            || modify_schema.email.is_some()
            || modify_schema.code.is_some()
        {
            return Err(ChorusError::PasswordRequired);
        }
        let request = Client::new()
            .patch(format!(
                "{}/users/@me",
                self.belongs_to.read().unwrap().urls.api
            ))
            .body(to_string(&modify_schema).unwrap())
            .header("Authorization", self.token())
            .header("Content-Type", "application/json");
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.deserialize_response::<User>(self).await
    }

    /// Deletes the user from the Instance.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#disable-user>
    pub async fn delete(mut self) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/delete",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            .header("Content-Type", "application/json");
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.handle_request_as_result(&mut self).await
    }
}

impl User {
    /// Gets a user by id, or if the id is None, gets the current user.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#get-user> and
    /// <https://discord-userdoccers.vercel.app/resources/user#get-current-user>
    pub async fn get(user: &mut ChorusUser, id: Option<&String>) -> ChorusResult<User> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let url = if id.is_none() {
            format!("{}/users/@me", url_api)
        } else {
            format!("{}/users/{}", url_api, id.unwrap())
        };
        let request = reqwest::Client::new()
            .get(url)
            .header("Authorization", user.token());
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

    /// Gets the user's settings.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/user_settings.html#get-users-me-settings>
    pub async fn get_settings(
        token: &String,
        url_api: &String,
        instance: &mut Instance,
    ) -> ChorusResult<UserSettings> {
        let request: reqwest::RequestBuilder = Client::new()
            .get(format!("{}/users/@me/settings", url_api))
            .header("Authorization", token);
        let mut user =
            ChorusUser::shell(Arc::new(RwLock::new(instance.clone())), token.clone()).await;
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        let result = match chorus_request.send_request(&mut user).await {
            Ok(result) => Ok(serde_json::from_str(&result.text().await.unwrap()).unwrap()),
            Err(e) => Err(e),
        };
        if instance.limits_information.is_some() {
            instance.limits_information.as_mut().unwrap().ratelimits = user
                .belongs_to
                .read()
                .unwrap()
                .clone_limits_if_some()
                .unwrap();
        }
        result
    }
}

impl Instance {
    /// Gets a user by id, or if the id is None, gets the current user.
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get`].
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#get-user> and
    /// <https://discord-userdoccers.vercel.app/resources/user#get-current-user>
    pub async fn get_user(&mut self, token: String, id: Option<&String>) -> ChorusResult<User> {
        let mut user = ChorusUser::shell(Arc::new(RwLock::new(self.clone())), token).await;
        let result = User::get(&mut user, id).await;
        if self.limits_information.is_some() {
            self.limits_information.as_mut().unwrap().ratelimits = user
                .belongs_to
                .read()
                .unwrap()
                .clone_limits_if_some()
                .unwrap();
        }
        result
    }
}
