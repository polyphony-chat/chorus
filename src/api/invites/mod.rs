use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::ratelimiter::ChorusRequest;
use crate::types::{CreateChannelInviteSchema, GuildInvite, Invite, Snowflake};

impl ChorusUser {
    /// Accepts an invite to a guild, group DM, or DM.
    ///
    /// Note that the session ID is required for guest invites.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/invite#accept-invite>
    pub async fn accept_invite(
        &mut self,
        invite_code: &str,
        session_id: Option<&str>,
    ) -> ChorusResult<Invite> {
        let mut request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/invites/{}",
                    self.belongs_to.read().unwrap().urls.api,
                    invite_code
                ))
                .header("Authorization", self.token()),
            limit_type: super::LimitType::Global,
        };
        if session_id.is_some() {
            request.request = request
                .request
                .header("Content-Type", "application/json")
                .body(to_string(session_id.unwrap()).unwrap());
        }
        request.deserialize_response::<Invite>(self).await
    }

    /// Creates a new friend invite.
    ///
    /// Note: Spacebar does not yet implement this endpoint.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/invite#create-user-invite>
    pub async fn create_user_invite(&mut self, code: Option<&str>) -> ChorusResult<Invite> {
        ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/users/@me/invites",
                    self.belongs_to.read().unwrap().urls.api
                ))
                .body(to_string(&code).unwrap())
                .header("Authorization", self.token())
                .header("Content-Type", "application/json"),
            limit_type: super::LimitType::Global,
        }
        .deserialize_response::<Invite>(self)
        .await
    }

    /// Creates a new invite for a guild channel or group DM.
    ///
    /// # Guild Channels
    /// For guild channels, the endpoint requires the [`CREATE_INSTANT_INVITE`](crate::types::PermissionFlags::CREATE_INSTANT_INVITE) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/invite#create-channel-invite>
    pub async fn create_channel_invite(
        &mut self,
        create_channel_invite_schema: CreateChannelInviteSchema,
        channel_id: Snowflake,
    ) -> ChorusResult<GuildInvite> {
        ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/channels/{}/invites",
                    self.belongs_to.read().unwrap().urls.api,
                    channel_id
                ))
                .header("Authorization", self.token())
                .header("Content-Type", "application/json")
                .body(to_string(&create_channel_invite_schema).unwrap()),
            limit_type: super::LimitType::Channel(channel_id),
        }
        .deserialize_response::<GuildInvite>(self)
        .await
    }
}
