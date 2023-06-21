use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::handle_request_as_result,
    errors::ChorusLibError,
    instance::UserMeta,
    types::{self, PermissionOverwrite, Snowflake},
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
    /// This function returns a result that is either [`Ok(())`] if the request is successful, or an [`Err(ChorusLibError)`].
    pub async fn edit_permissions(
        user: &mut UserMeta,
        channel_id: Snowflake,
        overwrite: PermissionOverwrite,
    ) -> Result<(), ChorusLibError> {
        let url = {
            format!(
                "{}/channels/{}/permissions/{}",
                user.belongs_to.borrow_mut().urls.api,
                channel_id,
                overwrite.id
            )
        };
        let body = match to_string(&overwrite) {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                });
            }
        };
        let request = Client::new().put(url).bearer_auth(user.token()).body(body);
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
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
    /// This function returns a Result that is either [`Ok(())`] if the request is successfulm or an [`Err(ChorusLibError)`].
    pub async fn delete_permission(
        user: &mut UserMeta,
        channel_id: Snowflake,
        overwrite_id: Snowflake,
    ) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/permissions/{}",
            user.belongs_to.borrow_mut().urls.api,
            channel_id,
            overwrite_id
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }
}
