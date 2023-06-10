use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::ChorusLibError,
    instance::UserMeta,
    limit::LimitedRequester,
    types::{self, PermissionOverwrite},
};

impl types::Channel {
    /// Edits the permission overwrites for a channel.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `channel_id` - A string slice representing the ID of the channel.
    /// * `overwrite` - A [`PermissionOverwrite`] instance representing the new permission overwrites.
    ///
    /// # Returns
    ///
    /// This function returns [`None`] if the request is successful, otherwise it returns a [`ChorusLibError`] instance.
    pub async fn edit_permissions(
        user: &mut UserMeta,
        channel_id: &str,
        overwrite: PermissionOverwrite,
    ) -> Option<ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/channels/{}/permissions/{}",
            belongs_to.urls.get_api(),
            channel_id,
            overwrite.id
        );
        let body = match to_string(&overwrite) {
            Ok(string) => string,
            Err(e) => {
                return Some(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                });
            }
        };
        let request = Client::new().put(url).bearer_auth(user.token()).body(body);
        LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Channel,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
            .unwrap();
        None
    }
}
