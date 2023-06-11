use reqwest::Client;
use serde_json::from_str;

use crate::{errors::ChorusLibError, instance::UserMeta, limit::LimitedRequester, types};

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
    ) -> Result<types::GuildMember, ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/guilds/{}/members/{}/",
            belongs_to.urls.get_api(),
            guild_id,
            member_id
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        let response = LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
            .unwrap();
        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                });
            }
        };
        let member = from_str::<types::GuildMember>(&response_text);
        if member.is_err() {
            return Err(ChorusLibError::InvalidResponseError {
                error: member.err().unwrap().to_string(),
            });
        }
        Ok(member.unwrap())
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
    /// An `Option` containing a `ChorusLibError` if the request fails, or `None` if the request succeeds.
    pub async fn add_role(
        user: &mut UserMeta,
        guild_id: &str,
        member_id: &str,
        role_id: &str,
    ) -> Option<crate::errors::ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}/",
            belongs_to.urls.get_api(),
            guild_id,
            member_id,
            role_id
        );
        let request = Client::new().put(url).bearer_auth(user.token());
        let response = LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await;
        if response.is_err() {
            return Some(response.err().unwrap());
        } else {
            return None;
        }
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
    /// An `Option` containing a `ChorusLibError` if the request fails, or `None` if the request succeeds.
    pub async fn remove_role(
        user: &mut UserMeta,
        guild_id: &str,
        member_id: &str,
        role_id: &str,
    ) -> Option<crate::errors::ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}/",
            belongs_to.urls.get_api(),
            guild_id,
            member_id,
            role_id
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        let response = LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await;
        if response.is_err() {
            return Some(response.err().unwrap());
        } else {
            return None;
        }
    }
}
