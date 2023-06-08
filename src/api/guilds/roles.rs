use reqwest::Client;
use serde_json::{from_str, to_string};

use crate::{
    errors::ChorusLibError,
    instance::UserMeta,
    limit::LimitedRequester,
    types::{self, RoleCreateModifySchema, RoleObject},
};

impl types::RoleObject {
    pub async fn get_all(
        user: &mut UserMeta,
        guild_id: &str,
    ) -> Result<Option<Vec<RoleObject>>, crate::errors::ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!("{}/guilds/{}/roles/", belongs_to.urls.get_api(), guild_id);
        let request = Client::new().get(url).bearer_auth(user.token());
        let requester = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(e),
        };
        let roles: Vec<RoleObject> = from_str(&requester.text().await.unwrap()).unwrap();

        if roles.is_empty() {
            return Ok(None);
        }

        Ok(Some(roles))
    }

    pub async fn create(
        user: &mut UserMeta,
        guild_id: &str,
        role_create_schema: RoleCreateModifySchema,
    ) -> Result<RoleObject, ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!("{}/guilds/{}/roles/", belongs_to.urls.get_api(), guild_id);
        let body = match to_string::<RoleCreateModifySchema>(&role_create_schema) {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                })
            }
        };
        let request = Client::new().post(url).bearer_auth(user.token()).body(body);
        let result = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(e),
        };
        let role: RoleObject = match from_str(&result.text().await.unwrap()) {
            Ok(role) => role,
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                })
            }
        };
        Ok(role)
    }
}
