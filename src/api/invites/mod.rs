use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::{Guild, Invite};

impl UserMeta {
    pub async fn accept_invite(
        &mut self,
        invite_code: &str,
        session_id: Option<&str>,
    ) -> ChorusResult<Guild> {
        ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/invites/{}",
                    self.belongs_to.borrow().urls.api,
                    invite_code
                ))
                .body(to_string(&session_id).unwrap())
                .bearer_auth(self.token()),
            limit_type: super::LimitType::Global,
        }
        .deserialize_response::<Guild>(self)
        .await
    }

    pub async fn create_user_invite(&mut self, code: Option<&str>) -> ChorusResult<Invite> {}
}
