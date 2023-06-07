use reqwest::Client;
use serde_json::from_str;

use crate::{
    instance::UserMeta,
    limit::LimitedRequester,
    types::{self, RoleObject},
};

impl types::RoleObject {
    pub async fn get_all(
        user: &mut UserMeta,
        guild_id: &str,
    ) -> Result<Option<Vec<RoleObject>>, crate::errors::InstanceServerError> {
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
}
