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
            request: Client::new().get(url).header("Authorization", user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
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
            request: Client::new()
                .put(url)
                .header("Authorization", user.token())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Guild(guild_id),
        };
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
            request: Client::new()
                .delete(url)
                .header("Authorization", user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request.handle_request_as_result(user).await
    }
}
