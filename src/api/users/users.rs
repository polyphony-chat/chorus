use crate::{
    api::{
        limits::Limits,
        types::{User, UserObject},
    },
    errors::InstanceServerError,
};

impl<'a> User<'a> {
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
        match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Ip,
                instance_limits,
                &mut Limits::default(),
            )
            .await
        {
            Ok(result) => Ok(serde_json::from_str(&result.text().await.unwrap()).unwrap()),
            Err(e) => Err(e),
        }
    }
}
