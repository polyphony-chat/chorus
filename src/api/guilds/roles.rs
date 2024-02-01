// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        self, LimitType, RoleCreateModifySchema, RoleObject, RolePositionUpdateSchema, Snowflake,
    },
};

impl types::RoleObject {
    /// Retrieves a list of roles for a given guild.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-roles>
    pub async fn get_all(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<Vec<RoleObject>> {
        let url = format!(
            "{}/guilds/{}/roles",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).header("Authorization", user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        let roles = chorus_request
            .deserialize_response::<Vec<RoleObject>>(user)
            .await
            .unwrap();
        Ok(roles)
    }

    /// Retrieves a single role for a given guild.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#get-/guilds/-guild_id-/roles/-role_id-/>
    pub async fn get(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<RoleObject> {
        let url = format!(
            "{}/guilds/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            role_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).header("Authorization", user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request
            .deserialize_response::<RoleObject>(user)
            .await
    }

    /// Creates a new role for a given guild.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#create-guild-role>
    pub async fn create(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_create_schema: RoleCreateModifySchema,
    ) -> ChorusResult<RoleObject> {
        let url = format!(
            "{}/guilds/{}/roles",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );
        let body = to_string::<RoleCreateModifySchema>(&role_create_schema).map_err(|e| {
            ChorusError::FormCreation {
                error: e.to_string(),
            }
        })?;
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(url)
                .header("Authorization", user.token())
                .header("Content-Type", "application/json")
                .body(body),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request
            .deserialize_response::<RoleObject>(user)
            .await
    }

    /// Updates the position of a role in a given guild's hierarchy.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#modify-guild-role-positions>
    pub async fn position_update(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_position_update_schema: RolePositionUpdateSchema,
    ) -> ChorusResult<RoleObject> {
        let url = format!(
            "{}/guilds/{}/roles",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );
        let body =
            to_string(&role_position_update_schema).map_err(|e| ChorusError::FormCreation {
                error: e.to_string(),
            })?;
        let chorus_request = ChorusRequest {
            request: Client::new()
                .patch(url)
                .header("Authorization", user.token())
                .header("Content-Type", "application/json")
                .body(body),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request
            .deserialize_response::<RoleObject>(user)
            .await
    }

    /// Modifies a role in a guild.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#modify-guild-role>
    pub async fn modify(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_id: Snowflake,
        role_create_schema: RoleCreateModifySchema,
    ) -> ChorusResult<RoleObject> {
        let url = format!(
            "{}/guilds/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            role_id
        );
        let body = to_string::<RoleCreateModifySchema>(&role_create_schema).map_err(|e| {
            ChorusError::FormCreation {
                error: e.to_string(),
            }
        })?;
        let chorus_request = ChorusRequest {
            request: Client::new()
                .patch(url)
                .header("Authorization", user.token())
                .header("Content-Type", "application/json")
                .body(body),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request
            .deserialize_response::<RoleObject>(user)
            .await
    }

    /// Deletes a guild role. Requires the `MANAGE_ROLES` permission. Returns a 204 empty response on success.
    ///
    /// # Reference:
    /// See <https://discord.com/developers/docs/resources/guild#delete-guild-role>
    pub async fn delete_role(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_id: Snowflake,
        audit_log_reason: Option<String>,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            role_id
        );

        let request = ChorusRequest::new(
            http::Method::DELETE,
            &url,
            None,
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Guild(guild_id),
        );
        request.handle_request_as_result(user).await
    }
}
