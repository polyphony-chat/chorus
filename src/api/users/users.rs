use reqwest::Client;

use crate::{
    api::{
        limits::Limits,
        types::{User, UserObject},
        UserSettings,
    },
    errors::InstanceServerError,
    instance::Instance,
};

impl<'a> User<'a> {
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
