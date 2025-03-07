// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;

use crate::{
    errors::ChorusResult,
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{self, GuildMember, LimitType, Snowflake},
};

impl types::GuildMember {
    /// Retrieves a guild member.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-member>
    pub async fn get(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        member_id: Snowflake,
    ) -> ChorusResult<GuildMember> {
        let url = format!(
            "{}/guilds/{}/members/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            member_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<GuildMember>(user)
            .await
    }

    /// Adds a role to a guild member.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#add-guild-member-role>
    pub async fn add_role(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            member_id,
            role_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().put(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request.handle_request_as_result(user).await
    }

    /// Removes a role from a guild member.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#remove-guild-member-role>
    pub async fn remove_role(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> Result<(), crate::errors::ChorusError> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            member_id,
            role_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request.handle_request_as_result(user).await
    }
}
