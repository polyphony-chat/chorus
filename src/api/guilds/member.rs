use reqwest::Client;

use crate::{instance::UserMeta, limit::LimitedRequester, types};

impl types::GuildMember {
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
}
