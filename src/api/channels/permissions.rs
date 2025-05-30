// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{self, LimitType, PermissionOverwrite, Snowflake},
};

impl types::Channel {
    /// Edits the permission overwrites for a user or role in a channel.
    ///
    /// Only usable for guild channels.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    /// Only permissions you have in the guild or parent channel (if applicable) can be allowed/denied
    /// (unless you have a [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) overwrite in the channel).
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/channel#modify-channel-permissions>
    pub async fn modify_permissions(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        audit_log_reason: Option<String>,
        overwrite: PermissionOverwrite,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/permissions/{}",
            user.belongs_to.read().unwrap().urls.api,
            channel_id,
            overwrite.id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().put(url).json(&overwrite),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        chorus_request.send_and_handle_as_result(user).await
    }

    /// Deletes a permission overwrite for a user or role in a channel.
    ///
    /// Only usable for guild channels.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/channel#delete-channel-permission>
    pub async fn delete_permission(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        overwrite_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/permissions/{}",
            user.belongs_to.read().unwrap().urls.api,
            channel_id,
            overwrite_id
        );

        let request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Channel(channel_id),
        }
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }
}
