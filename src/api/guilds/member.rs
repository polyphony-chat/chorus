// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use reqwest::Client;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        self, AddGuildMemberReturn, AddGuildMemberSchema, AddRoleMembersSchema, Guild, GuildMember,
        LimitType, ModifyCurrentGuildMemberSchema, ModifyGuildMemberProfileSchema,
        ModifyGuildMemberSchema, Snowflake, UserProfileMetadata,
    },
};

impl Guild {
    /// Fetch a [GuildMember] object for the specified user.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#get-guild-member>
    pub async fn get_member(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        member_id: Snowflake,
    ) -> ChorusResult<GuildMember> {
        let url = format!(
            "{}/guilds/{}/members/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            member_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        chorus_request
            .send_and_deserialize_response::<GuildMember>(user)
            .await
    }

    /// Adds a user to a guild, provided you have a valid OAuth2 access token for that user with
    /// the guilds.join scope.
    ///
    /// This endpoint is only usable by bots, they must be belonging
    /// to the application used for authorization and they must be a member
    /// of the guild with the [CREATE_INSTANT_INVITE](crate::types::PermissionFlags::CREATE_INSTANT_INVITE) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#add-guild-member>
    pub async fn add_member(
        guild_id: Snowflake,
        user_id: Snowflake,
        schema: AddGuildMemberSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<AddGuildMemberReturn> {
        let request = ChorusRequest {
            request: Client::new()
                .put(format!(
                    "{}/guilds/{}/members/{}",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                    user_id
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_authorization_for(user);

        let response = request.send(user).await?;
        log::trace!("Got response: {:?}", response);

        let http_status = response.status();

        match http_status {
            http::StatusCode::OK => {
                let response_text = match response.text().await {
                    Ok(string) => string,
                    Err(e) => {
                        return Err(ChorusError::InvalidResponse {
                            error: format!(
                                "Error while trying to process the HTTP response into a String: {}",
                                e
                            ),
                            http_status,
                        });
                    }
                };

                match serde_json::from_str::<GuildMember>(&response_text) {
                            Ok(object) => Ok(AddGuildMemberReturn::Joined(object)),
                            Err(e) => {
                                Err(ChorusError::InvalidResponse {
												error: format!(
												"Error while trying to deserialize the JSON response into requested type T: {}. JSON Response: {}",
												e, response_text),
												http_status
											})
                            }
                        }
            }
            http::StatusCode::NO_CONTENT => Ok(AddGuildMemberReturn::AlreadyAMember),
            _ => Err(ChorusError::InvalidResponse {
                error: format!("Received unexpected http status code: {}", http_status),
                http_status,
            }),
        }
    }

    /// Removes a [GuildMember] from a guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#remove-guild-member>
    pub async fn remove_member(
        guild_id: Snowflake,
        member_id: Snowflake,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest {
            request: Client::new().delete(format!(
                "{}/guilds/{}/members/{}",
                user.belongs_to.read().unwrap().urls.api,
                guild_id,
                member_id,
            )),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.send_and_handle_as_result(user).await
    }

    /// Modifies a [GuildMember] object.
    ///
    /// Returns the updated object on success.
    ///
    /// # Reference
    /// <https://docs.discord.food/resources/guild#modify-guild-member>
    pub async fn modify_member(
        guild_id: Snowflake,
        member_id: Snowflake,
        schema: ModifyGuildMemberSchema,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildMember> {
        let request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/guilds/{}/members/{}",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                    member_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request
            .send_and_deserialize_response::<GuildMember>(user)
            .await
    }

    /// Modifies the current user's member object in the guild.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#modify-current-guild-member>
    pub async fn modify_current_member(
        guild_id: Snowflake,
        schema: ModifyCurrentGuildMemberSchema,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildMember> {
        let request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/guilds/{}/members/@me",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request
            .send_and_deserialize_response::<GuildMember>(user)
            .await
    }

    /// Modifies the current user's profile in the guild.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#modify-guild-member-profile>
    pub async fn modify_current_member_profile(
        guild_id: Snowflake,
        schema: ModifyGuildMemberProfileSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<UserProfileMetadata> {
        let request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/guilds/{}/profile/@me",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request
            .send_and_deserialize_response::<UserProfileMetadata>(user)
            .await
    }

    /// Adds a role to a guild member.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#add-guild-member-role>
    pub async fn add_member_role(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            member_id,
            role_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().put(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        chorus_request.send_and_handle_as_result(user).await
    }

    /// Removes a role from a guild member.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#remove-guild-member-role>
    pub async fn remove_member_role(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/members/{}/roles/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            member_id,
            role_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        chorus_request.send_and_handle_as_result(user).await
    }

    /// Retrieves a mapping of role IDs to their respective member counts.
    ///
    /// # Notes
    /// This method is wrapper around
    /// [RoleObject::get_all_member_counts](crate::types::RoleObject::get_all_member_counts)
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#get-guild-role-member-counts>
    pub async fn get_role_member_counts(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<HashMap<Snowflake, usize>> {
        crate::types::RoleObject::get_all_member_counts(user, guild_id).await
    }

    /// Returns a list of member IDs that have the specified role, up to a maximum of 100.
    ///
    /// (This endpoint does not return results for the @everyone role)
    ///
    /// # Notes
    /// This method is wrapper around
    /// [RoleObject::get_all_member_counts](crate::types::RoleObject::get_all_member_counts)
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#get-guild-role-members>
    pub async fn get_role_members(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<Vec<Snowflake>> {
        crate::types::RoleObject::get_members(user, guild_id, role_id).await
    }

    /// Adds multiple guild members to a role.
    ///
    /// Requires the [MANAGE_ROLES](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// Returns a mapping of member IDs to guild member objects.
    ///
    /// # Notes
    /// This method is wrapper around
    /// [RoleObject::add_members](crate::types::RoleObject::add_members)
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#add-guild-role-members>
    pub async fn add_role_members(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        role_id: Snowflake,
        schema: AddRoleMembersSchema,
    ) -> ChorusResult<HashMap<Snowflake, GuildMember>> {
        crate::types::RoleObject::add_members(user, audit_log_reason, guild_id, role_id, schema)
            .await
    }
}

impl types::GuildMember {
    /// Fetch a [GuildMember] object for the specified user.
    ///
    /// # Notes
    /// This is an alias of [Guild::get_member]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#get-guild-member>
    pub async fn get(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        member_id: Snowflake,
    ) -> ChorusResult<GuildMember> {
        Guild::get_member(user, guild_id, member_id).await
    }

    /// Adds a user to a guild, provided you have a valid OAuth2 access token for that user with
    /// the guilds.join scope.
    ///
    /// This endpoint is only usable by bots, they must be belonging
    /// to the application used for authorization and they must be a member
    /// of the guild with the [CREATE_INSTANT_INVITE](crate::types::PermissionFlags::CREATE_INSTANT_INVITE) permission.
    ///
    /// # Notes
    /// This is an alias of [Guild::add_member]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#add-guild-member>
    pub async fn add(
        guild_id: Snowflake,
        user_id: Snowflake,
        schema: AddGuildMemberSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<AddGuildMemberReturn> {
        Guild::add_member(guild_id, user_id, schema, user).await
    }

    /// Removes a [GuildMember] from a guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// # Notes
    /// This is an alias of [Guild::remove_member]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#remove-guild-member>
    pub async fn remove(
        guild_id: Snowflake,
        member_id: Snowflake,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        Guild::remove_member(guild_id, member_id, audit_log_reason, user).await
    }

    /// Modifies a [GuildMember] object.
    ///
    /// Returns the updated object on success.
    ///
    /// # Notes
    /// This is an alias of [Guild::modify_member]
    ///
    /// # Reference
    /// <https://docs.discord.food/resources/guild#modify-guild-member>
    pub async fn modify(
        guild_id: Snowflake,
        member_id: Snowflake,
        schema: ModifyGuildMemberSchema,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildMember> {
        Guild::modify_member(guild_id, member_id, schema, audit_log_reason, user).await
    }

    /// Modifies the current user's member object in the guild.
    ///
    /// # Notes
    /// This is an alias of [Guild::modify_current_member]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#modify-current-guild-member>
    pub async fn modify_current(
        guild_id: Snowflake,
        schema: ModifyCurrentGuildMemberSchema,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildMember> {
        Guild::modify_current_member(guild_id, schema, audit_log_reason, user).await
    }

    /// Modifies the current user's profile in the guild.
    ///
    /// # Notes
    /// This is an alias of [Guild::modify_current_member_profile]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#modify-guild-member-profile>
    pub async fn modify_current_profile(
        guild_id: Snowflake,
        schema: ModifyGuildMemberProfileSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<UserProfileMetadata> {
        Guild::modify_current_member_profile(guild_id, schema, user).await
    }

    /// Adds a role to a guild member.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Notes
    /// This is an alias of [Guild::add_member_role]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#add-guild-member-role>
    pub async fn add_role(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> ChorusResult<()> {
        Guild::add_member_role(user, audit_log_reason, guild_id, member_id, role_id).await
    }

    /// Removes a role from a guild member.
    ///
    /// Requires the [`MANAGE_ROLES`](crate::types::PermissionFlags::MANAGE_ROLES) permission.
    ///
    /// # Notes
    /// This is an alias of [Guild::remove_member_role]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#remove-guild-member-role>
    pub async fn remove_role(
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        guild_id: Snowflake,
        member_id: Snowflake,
        role_id: Snowflake,
    ) -> Result<(), crate::errors::ChorusError> {
        Guild::remove_member_role(user, audit_log_reason, guild_id, member_id, role_id).await
    }
}
