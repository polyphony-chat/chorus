use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::handle_request,
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
        let belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/channels/{}/permissions/{}",
            belongs_to.urls.get_api(),
            channel_id,
            overwrite.id
        );
        drop(belongs_to);
        let body = match to_string(&overwrite) {
            Ok(string) => string,
            Err(e) => {
                return Some(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                });
            }
        };
        let request = Client::new().put(url).bearer_auth(user.token()).body(body);
        match handle_request(request, user, crate::api::limits::LimitType::Channel).await {
            Ok(_) => None,
            Err(e) => Some(ChorusLibError::InvalidResponseError {
                error: e.to_string(),
            }),
        }
    }

    /// Deletes a permission overwrite for a channel.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `channel_id` - A string slice representing the ID of the channel.
    /// * `overwrite_id` - A string slice representing the ID of the permission overwrite to delete.
    ///
    /// # Returns
    ///
    /// This function returns [`None`] if the request is successful, otherwise it returns a [`ChorusLibError`] instance.
    pub async fn delete_permission(
        user: &mut UserMeta,
        channel_id: &str,
        overwrite_id: &str,
    ) -> Option<ChorusLibError> {
        let belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/channels/{}/permissions/{}",
            belongs_to.urls.get_api(),
            channel_id,
            overwrite_id
        );
        drop(belongs_to);
        let request = Client::new().delete(url).bearer_auth(user.token());
        match handle_request(request, user, crate::api::limits::LimitType::Channel).await {
            Ok(_) => None,
            Err(e) => Some(ChorusLibError::InvalidResponseError {
                error: e.to_string(),
            }),
        }
    }
}
