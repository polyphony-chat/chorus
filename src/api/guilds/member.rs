use reqwest::Client;

use crate::{
    api::LimitType,
    errors::ChorusResult,
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{self, GuildMember, Snowflake},
};

impl types::GuildMember {
    /// Retrieves a guild member by their ID.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `guild_id` - The ID of the guild.
    /// * `member_id` - The ID of the member.
    ///
    /// # Returns
    ///
    /// A [`ChorusResult`] containing a [`GuildMember`] if the request succeeds.
    pub async fn get(
        user: &mut UserMeta,
        guild_id: Snowflake,
        member_id: Snowflake,
    ) -> ChorusResult<GuildMember> {
        let url = format!(
            "{}/guilds/{}/members/{}/",
            user.belongs_to.borrow().urls.api,
            guild_id,
            member_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).bearer_auth(user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request
            .deserialize_response::<GuildMember>(user)
            .await
    }

    /// Adds a role to a guild member.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a `UserMeta` instance.
    /// * `guild_id` - The ID of the guild.
    /// * `member_id` - The ID of the member.
    /// * `role_id` - The ID of the role to add.
    ///
    /// # Returns
    ///
    /// A [`ChorusResult`] containing a [`crate::errors::ChorusError`] if the request fails, or `()` if the request succeeds.
    pub async fn add_role(
        user: &mut UserMeta,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}/",
            user.belongs_to.borrow().urls.api,
            guild_id,
            member_id,
            role_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().put(url).bearer_auth(user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Removes a role from a guild member.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a `UserMeta` instance.
    /// * `guild_id` - The ID of the guild.
    /// * `member_id` - The ID of the member.
    /// * `role_id` - The ID of the role to remove.
    ///
    /// # Returns
    ///
    /// A [`ChorusResult`] containing a [`crate::errors::ChorusError`] if the request fails, or `()` if the request succeeds.
    pub async fn remove_role(
        user: &mut UserMeta,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> Result<(), crate::errors::ChorusError> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}/",
            user.belongs_to.borrow().urls.api,
            guild_id,
            member_id,
            role_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).bearer_auth(user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request.handle_request_as_result(user).await
    }
}
