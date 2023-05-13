use serde_json::to_string;

use crate::api::schemas;
use crate::api::types;

impl<'a> types::Guild {
    pub async fn create(
        user: &mut types::User<'a>,
        instance: &mut crate::instance::Instance,
        guild_create_schema: &schemas::GuildCreateSchema,
    ) -> Result<String, crate::errors::InstanceServerError> {
        let url = format!("{}/guilds/", instance.urls.get_api().to_string());
        let limits_user = user.limits.get_as_mut();
        let limits_instance = instance.limits.get_as_mut();
        let request = reqwest::Client::new()
            .post(url.clone())
            .bearer_auth(user.token.clone())
            .body(to_string(guild_create_schema).unwrap());
        let mut requester = crate::limit::LimitedRequester::new().await;
        let result = match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
                limits_user,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        return Ok(match result.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(crate::errors::InstanceServerError::RequestErrorError {
                    url: url.to_string(),
                    error: e.to_string(),
                })
            }
        });
    }
}
