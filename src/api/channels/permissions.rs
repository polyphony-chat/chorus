use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::LimitType,
    errors::{ChorusError, ChorusResult},
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{self, PermissionOverwrite, Snowflake},
};

impl types::Channel {
    /// Edits the permission overwrites for a channel.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#modify-channel-permissions>
    pub async fn edit_permissions(
        user: &mut UserMeta,
        channel_id: Snowflake,
        overwrite: PermissionOverwrite,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/permissions/{}",
            user.belongs_to.borrow_mut().urls.api,
            channel_id,
            overwrite.id
        );
        let body = match to_string(&overwrite) {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::FormCreation {
                    error: e.to_string(),
                });
            }
        };
        let chorus_request = ChorusRequest {
            request: Client::new().put(url).bearer_auth(user.token()).body(body),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Deletes a permission overwrite for a channel.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#delete-channel-permission>
    pub async fn delete_permission(
        user: &mut UserMeta,
        channel_id: Snowflake,
        overwrite_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/permissions/{}",
            user.belongs_to.borrow_mut().urls.api,
            channel_id,
            overwrite_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }
}
