use reqwest::Client;

use crate::{
    api::{deserialize_response, handle_request_as_result},
    errors::ChorusResult,
    instance::UserMeta,
    types,
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
    /// A [`Result`] containing a [`GuildMember`] if the request succeeds, or a [`ChorusLibError`] if the request fails.
    pub async fn get(
        user: &mut UserMeta,
        guild_id: &str,
        member_id: &str,
    ) -> ChorusResult<types::GuildMember> {
        let url = format!(
            "{}/guilds/{}/members/{}/",
            user.belongs_to.borrow().urls.api,
            guild_id,
            member_id
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        deserialize_response::<types::GuildMember>(
            request,
            user,
            crate::api::limits::LimitType::Guild,
        )
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
    /// An `Result` containing a `ChorusLibError` if the request fails, or `()` if the request succeeds.
    pub async fn add_role(
        user: &mut UserMeta,
        guild_id: &str,
        member_id: &str,
        role_id: &str,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}/",
            user.belongs_to.borrow().urls.api,
            guild_id,
            member_id,
            role_id
        );
        let request = Client::new().put(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Guild).await
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
    /// A `Result` containing a `ChorusLibError` if the request fails, or `()` if the request succeeds.
    pub async fn remove_role(
        user: &mut UserMeta,
        guild_id: &str,
        member_id: &str,
        role_id: &str,
    ) -> Result<(), crate::errors::ChorusLibError> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}/",
            user.belongs_to.borrow().urls.api,
            guild_id,
            member_id,
            role_id
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Guild).await
    }
}
