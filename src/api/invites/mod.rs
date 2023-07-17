use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::{CreateChannelInviteSchema, GuildInvite, Invite, Snowflake};

impl UserMeta {
    /// # Arguments
    /// - invite_code: The invite code to accept the invite for.
    /// - session_id: The session ID that is accepting the invite, required for guest invites.
    ///
    /// # Reference:
    /// Read <https://discord-userdoccers.vercel.app/resources/invite#accept-invite>
    pub async fn accept_invite(
        &mut self,
        invite_code: &str,
        session_id: Option<&str>,
    ) -> ChorusResult<Invite> {
        let mut request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/invites/{}/",
                    self.belongs_to.borrow().urls.api,
                    invite_code
                ))
                .bearer_auth(self.token()),
            limit_type: super::LimitType::Global,
        };
        if session_id.is_some() {
            request.request = request
                .request
                .body(to_string(session_id.unwrap()).unwrap());
        }
        request.deserialize_response::<Invite>(self).await
    }
    /// Note: Spacebar does not yet implement this endpoint.
    pub async fn create_user_invite(&mut self, code: Option<&str>) -> ChorusResult<Invite> {
        ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/users/@me/invites/",
                    self.belongs_to.borrow().urls.api
                ))
                .body(to_string(&code).unwrap())
                .bearer_auth(self.token()),
            limit_type: super::LimitType::Global,
        }
        .deserialize_response::<Invite>(self)
        .await
    }

    pub async fn create_guild_invite(
        &mut self,
        create_channel_invite_schema: CreateChannelInviteSchema,
        channel_id: Snowflake,
    ) -> ChorusResult<GuildInvite> {
        ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/channels/{}/invites/",
                    self.belongs_to.borrow().urls.api,
                    channel_id
                ))
                .bearer_auth(self.token())
                .body(to_string(&create_channel_invite_schema).unwrap()),
            limit_type: super::LimitType::Channel(channel_id),
        }
        .deserialize_response::<GuildInvite>(self)
        .await
    }
}
