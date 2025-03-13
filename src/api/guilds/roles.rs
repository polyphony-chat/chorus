// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        self, AddRoleMembersSchema, GuildMember, LimitType, RoleCreateModifySchema, RoleObject,
        RolePositionUpdateSchema, Snowflake,
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
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<Vec<RoleObject>>(user)
            .await
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
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<RoleObject>(user)
            .await
    }

    /// Retrieves a mapping of role IDs to their respective member counts.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-role-member-counts>
    pub async fn get_all_member_counts(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<HashMap<Snowflake, usize>> {
        let url = format!(
            "{}/guilds/{}/roles/member-counts",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<HashMap<Snowflake, usize>>(user)
            .await
    }

    /// Returns a list of member IDs that have the specified role, up to a maximum of 100.
    ///
    /// (This endpoint does not return results for the @everyone role)
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-role-members>
    pub async fn get_members(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<Vec<Snowflake>> {
        let url = format!(
            "{}/guilds/{}/roles/{}/member-ids",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            role_id,
        );

        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<Vec<Snowflake>>(user)
            .await
    }

    /// Adds a guild member to a role.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Notes
    /// This method is a wrapper around
    /// [Guild::add_member_role](crate::types::Guild::add_member_role)
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#add-guild-member-role>
    pub async fn add_member(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        crate::types::Guild::add_member_role(user, audit_log_reason, guild_id, member_id, role_id)
            .await
    }

    /// Removes a guild member from a role.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Notes
    /// This method is a wrapper around
    /// [Guild::remove_member_role](crate::types::Guild::remove_member_role)
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#remove-guild-member-role>
    pub async fn remove_member(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        crate::types::Guild::remove_member_role(
            user,
            audit_log_reason,
            guild_id,
            member_id,
            role_id,
        )
        .await
    }

    /// Adds multiple guild members to a role.
    ///
    /// Requires the [MANAGE_ROLES](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// Returns a mapping of member IDs to guild member objects.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#add-guild-role-members>
    pub async fn add_members(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        role_id: Snowflake,
        schema: AddRoleMembersSchema,
    ) -> ChorusResult<HashMap<Snowflake, GuildMember>> {
        let url = format!(
            "{}/guilds/{}/roles/{}/members",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            role_id,
        );

        let chorus_request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<HashMap<Snowflake, GuildMember>>(user)
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

        let chorus_request = ChorusRequest {
            request: Client::new().post(url).json(&role_create_schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

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

        let chorus_request = ChorusRequest {
            request: Client::new().patch(url).json(&role_position_update_schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

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

        let chorus_request = ChorusRequest {
            request: Client::new().patch(url).json(&role_create_schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

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

        let request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.handle_request_as_result(user).await
    }
}
