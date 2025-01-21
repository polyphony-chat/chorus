// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    AcceptInviteSchema, CreateChannelInviteSchema, GuildInvite, Invite, LimitType, Snowflake,
};

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
        session_id: Option<String>,
    ) -> ChorusResult<Invite> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/invites/{}",
                    self.belongs_to.read().unwrap().urls.api,
                    invite_code
                ))
                .json(&AcceptInviteSchema { session_id }),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);

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
                .json(&code),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self)
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
                .json(&create_channel_invite_schema),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(self)
        .deserialize_response::<GuildInvite>(self)
        .await
    }
}
