// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bytes::Bytes;
use futures_util::FutureExt;
use reqwest::Client;
use serde_json::from_str;
use serde_json::to_string;

use crate::errors::ChorusError;
use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::instance::Instance;
use crate::ratelimiter::ChorusRequest;
use crate::types::ActionGuildJoinRequestSchema;
use crate::types::AdminCommunityEligibility;
use crate::types::BulkActionGuildJoinRequestsSchema;
use crate::types::BulkGuildBanReturn;
use crate::types::BulkGuildBanSchema;
use crate::types::CreateGuildJoinRequestSchema;
use crate::types::GetGuildJoinRequestsQuery;
use crate::types::GetGuildJoinRequestsReturn;
use crate::types::GetGuildMemberVerificationQuery;
use crate::types::GetGuildMembersSchema;
use crate::types::GetGuildMembersSupplementalSchema;
use crate::types::GetGuildPruneResult;
use crate::types::GuildJoinRequest;
use crate::types::GuildJoinRequestCooldown;
use crate::types::GuildMemberVerification;
use crate::types::GuildModifyMFALevelSchema;
use crate::types::GuildModifyVanityInviteSchema;
use crate::types::GuildOnboarding;
use crate::types::GuildPruneParameters;
use crate::types::GuildPruneResult;
use crate::types::GuildPruneSchema;
use crate::types::GuildVanityInviteInfo;
use crate::types::GuildWidget;
use crate::types::GuildWidgetImageStyle;
use crate::types::GuildWidgetSettings;
use crate::types::MFALevel;
use crate::types::ModifyGuildMemberVerificationSchema;
use crate::types::ModifyGuildOnboardingSchema;
use crate::types::ModifyGuildWelcomeScreenSchema;
use crate::types::ModifyGuildWidgetSchema;
use crate::types::PublicGuildWelcomeScreen;
use crate::types::SGMReturnNotIndexed;
use crate::types::SGMReturnOk;
use crate::types::SearchGuildBansQuery;
use crate::types::SearchGuildMembersReturn;
use crate::types::SearchGuildMembersSchema;
use crate::types::SupplementalGuildMember;
use crate::types::{
    Channel, ChannelCreateSchema, GetGuildBansQuery, Guild, GuildBanCreateSchema,
    GuildCreateSchema, GuildMember, GuildModifySchema, GuildPreview, LimitType,
    ModifyGuildMemberProfileSchema, ModifyGuildMemberSchema, QueryGuildMembersSchema,
    UserProfileMetadata,
};
use crate::types::{GuildBan, Snowflake};

impl Guild {
    /// Fetches a guild by its id.
    ///
    /// Setting `with_counts` to `true` will make the [Guild] object include approximate member and
    /// presence counts
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild>
    pub async fn get(
        guild_id: Snowflake,
        with_counts: Option<bool>,
        user: &mut ChorusUser,
    ) -> ChorusResult<Guild> {
        let mut chorus_request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/guilds/{}",
                user.belongs_to.read().unwrap().urls.api,
                guild_id
            )),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        if let Some(with_counts) = with_counts {
            chorus_request.request = chorus_request.request.query(&[(
                "with_counts",
                serde_json::to_string(&with_counts).unwrap().as_str(),
            )]);
        }

        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
    }

    /// Creates a new guild.
    ///
    /// Fires off a [crate::types::GuildCreate] gateway event
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#create-guild>
    pub async fn create(
        user: &mut ChorusUser,
        guild_create_schema: GuildCreateSchema,
    ) -> ChorusResult<Guild> {
        let url = format!("{}/guilds", user.belongs_to.read().unwrap().urls.api);
        let chorus_request = ChorusRequest {
            request: Client::new().post(url.clone()).json(&guild_create_schema),
            limit_type: LimitType::Global,
        }
        .with_headers_for(user);
        chorus_request.deserialize_response::<Guild>(user).await
    }

    /// Modify a guild's settings.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// Returns the updated guild.
    ///
    /// Fires a [GuildUpdate](crate::types::GuildUpdate) gateway event.
    ///
    /// # Notes
    /// This route requires MFA.
    ///
    /// # Reference
    /// <https://discord-userdoccers.vercel.app/resources/guild#modify-guild>
    pub async fn modify(
        guild_id: Snowflake,
        schema: GuildModifySchema,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<Guild> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/guilds/{}",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_mfa(&user.mfa_token)
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
    }

    /// Modifies the guild's mfa requirement for administrative actions.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// Fires a [GuildUpdate](crate::types::GuildUpdate) gateway event.
    ///
    /// # Notes
    /// This route requires MFA.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#modify-guild-mfa-level>
    pub async fn modify_mfa_level(
        guild_id: Snowflake,
        mfa_level: MFALevel,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/guilds/{}/mfa",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id
                ))
                .json(&GuildModifyMFALevelSchema { level: mfa_level }),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_mfa(&user.mfa_token)
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        chorus_request
            .deserialize_response::<GuildModifyMFALevelSchema>(user)
            .await
            .map(|_x| ())
    }

    /// Deletes a guild by its id.
    ///
    /// User must be the owner.
    ///
    /// # Notes
    /// This route requires MFA.
    ///
    /// # Example
    ///
    /// ```rust
    /// # mod tests;
    /// # tokio_test::block_on(async {
    /// # let mut bundle = tests::common::setup().await;
    /// # use chorus::{types::Guild, instance::ChorusUser, types::Snowflake};
    /// let mut user: ChorusUser;
    /// # user = bundle.user;
    /// let guild_id = Snowflake::from(1234567890);
    /// # let guild_id = bundle.guild.read().unwrap().id;
    ///
    /// match Guild::delete(&mut user, guild_id).await {
    ///     Err(e) => println!("Error deleting guild: {:?}", e),
    ///     Ok(_) => println!("Guild deleted successfully"),
    /// }
    /// # tests::common::teardown(bundle).await;
    /// # })
    /// ```
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#delete-guild>
    pub async fn delete(user: &mut ChorusUser, guild_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/delete",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let chorus_request = ChorusRequest {
            request: Client::new().post(url.clone()),
            limit_type: LimitType::Global,
        }
        .with_maybe_mfa(&user.mfa_token)
        .with_headers_for(user);

        chorus_request.handle_request_as_result(user).await
    }

    /// Creates a new channel in a guild.
    ///
    /// Requires the [MANAGE_CHANNELS](crate::types::PermissionFlags::MANAGE_CHANNELS) permission.
    ///
    /// # Notes
    /// This method is a wrapper for [Channel::create].
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#create-guild-channel>
    pub async fn create_channel(
        &self,
        user: &mut ChorusUser,
        audit_log_reason: Option<String>,
        schema: ChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        Channel::create(user, self.id, audit_log_reason, schema).await
    }

    /// Returns a list of the guild's channels.
    ///
    /// Doesn't include threads.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#get-guild-channels>
    pub async fn channels(&self, user: &mut ChorusUser) -> ChorusResult<Vec<Channel>> {
        let chorus_request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/guilds/{}/channels",
                user.belongs_to.read().unwrap().urls.api,
                self.id
            )),
            limit_type: LimitType::Channel(self.id),
        }
        .with_headers_for(user);

        let result = chorus_request.send_request(user).await?;
        let stringed_response = match result.text().await {
            Ok(value) => value,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: e.to_string(),
                });
            }
        };
        let _: Vec<Channel> = match from_str(&stringed_response) {
            Ok(result) => return Ok(result),
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: e.to_string(),
                });
            }
        };
    }

    /// Returns a guild preview object for the given guild ID.
    ///
    /// If the user is not in the guild, the guild must be discoverable.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-preview>
    pub async fn get_preview(
        guild_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildPreview> {
        let chorus_request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/guilds/{}/preview",
                user.belongs_to.read().unwrap().urls.api,
                guild_id,
            )),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        let response = chorus_request
            .deserialize_response::<GuildPreview>(user)
            .await?;
        Ok(response)
    }

    /// Returns a list of guild member objects that are members of the guild.
    ///
    /// # Notes
    /// This endpoint is not usable by user accounts and is restricted based on the
    /// GUILD_MEMBERS intent for applications
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-members>
    pub async fn get_members(
        guild_id: Snowflake,
        query: GetGuildMembersSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<GuildMember>> {
        let request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/guilds/{}/members",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .query(&query.to_query()),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response::<Vec<GuildMember>>(user).await
    }

    /// Returns a list of guild member objects whose username or nickname starts with a provided string.
    ///
    /// Functions identically to the [RequestGuildMembers](crate::types::GatewayRequestGuildMembers) gateway event
    ///
    /// # Notes
    /// This endpoint is not usable by user accounts
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#query-guild-members>
    pub async fn query_members(
        guild_id: Snowflake,
        query: QueryGuildMembersSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<GuildMember>> {
        let request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/guilds/{}/members/search",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .query(&query.to_query()),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response::<Vec<GuildMember>>(user).await
    }

    /// Returns [SupplementalGuildMember](crate::types::SupplementalGuildMember) objects that match a specified query.
    ///
    /// Requires the [PermissionFlags::MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Notes
    ///
    /// (On the Discord.com client, this
    /// endpoint is used for the User Management - Members tab in Server Settings)
    ///
    /// This endpoint utilizes Elasticsearch to power results.
    ///
    /// This means that while it is very powerful, it's also tricky to use and reliant on an
    /// index.
    ///
    /// As of 2025/01/15, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-members-supplemental>
    pub async fn search_members(
        guild_id: Snowflake,
        schema: SearchGuildMembersSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<SearchGuildMembersReturn> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/guilds/{}/members-search",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        let response = request.send_request(user).await?;
        log::trace!("Got response: {:?}", response);

        let status = response.status();

        match status {
            http::StatusCode::ACCEPTED | http::StatusCode::OK => {
                let response_text = match response.text().await {
                    Ok(string) => string,
                    Err(e) => {
                        return Err(ChorusError::InvalidResponse {
                            error: format!(
                                "Error while trying to process the HTTP response into a String: {}",
                                e
                            ),
                        });
                    }
                };

                match status {
                    http::StatusCode::ACCEPTED => {
                        match serde_json::from_str::<SGMReturnNotIndexed>(&response_text) {
                            Ok(object) => Ok(SearchGuildMembersReturn::NotIndexed(object)),
                            Err(e) => {
                                Err(ChorusError::InvalidResponse {
												error: format!(
												"Error while trying to deserialize the JSON response into requested type T: {}. JSON Response: {}",
												e, response_text),
											})
                            }
                        }
                    }
                    http::StatusCode::OK => {
                        match serde_json::from_str::<SGMReturnOk>(&response_text) {
                            Ok(object) => Ok(SearchGuildMembersReturn::Ok(object)),
                            Err(e) => {
                                Err(ChorusError::InvalidResponse {
												error: format!(
												"Error while trying to deserialize the JSON response into requested type T: {}. JSON Response: {}",
												e, response_text),
											})
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => Err(ChorusError::ReceivedErrorCode {
                error_code: response.status().as_u16(),
                error: response.status().to_string(),
            }),
        }
    }

    /// Fetches [SupplementalGuildMember] objects for the given user IDs.
    ///
    /// Requires the [PermissionFlags::MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Notes
    /// As of 2025/01/15, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-members-supplemental>
    pub async fn get_members_supplemental(
        guild_id: Snowflake,
        schema: GetGuildMembersSupplementalSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<SupplementalGuildMember>> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/guilds/{}/members/supplemental",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns a list of ban objects for the guild.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-bans>
    pub async fn get_bans(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: Option<GetGuildBansQuery>,
    ) -> ChorusResult<Vec<GuildBan>> {
        let url = format!(
            "{}/guilds/{}/bans",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let mut request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        if let Some(query) = query {
            request.request = request.request.query(&query.to_query());
        }

        request.deserialize_response::<Vec<GuildBan>>(user).await
    }

    /// Returns a list of ban objects whose usernames or display names contains a provided string.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#search-guild-bans>
    pub async fn search_bans(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: SearchGuildBansQuery,
    ) -> ChorusResult<Vec<GuildBan>> {
        let url = format!(
            "{}/guilds/{}/bans/search",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let mut request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.request = request.request.query(&query.to_query());

        request.deserialize_response::<Vec<GuildBan>>(user).await
    }

    /// Returns a ban object for the given user.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-ban>
    pub async fn get_ban(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        user_id: Snowflake,
    ) -> ChorusResult<GuildBan> {
        let url = format!(
            "{}/guilds/{}/bans/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            user_id
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response::<GuildBan>(user).await
    }

    /// Creates a ban for the guild - bans a user from the guild.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#create-guild-ban>
    pub async fn create_ban(
        guild_id: Snowflake,
        user_id: Snowflake,
        audit_log_reason: Option<String>,
        schema: GuildBanCreateSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        // FIXME: Return GuildBan instead of (). Requires <https://github.com/spacebarchat/server/issues/1096> to be resolved.
        let request = ChorusRequest {
            request: Client::new()
                .put(format!(
                    "{}/guilds/{}/bans/{}",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                    user_id
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.handle_request_as_result(user).await
    }

    /// Creates multiple bans for the guild.
    ///
    /// Requires both the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) and [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permissions.
    ///
    /// # Notes
    /// This route requires MFA.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#bulk-guild-ban>
    pub async fn bulk_create_ban(
        guild_id: Snowflake,
        audit_log_reason: Option<String>,
        schema: BulkGuildBanSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<BulkGuildBanReturn> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/guilds/{}/bulk-ban",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id,
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_maybe_mfa(&user.mfa_token)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Removes the ban for a user.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#delete-guild-ban>
    pub async fn delete_ban(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        user_id: Snowflake,
        audit_log_reason: Option<String>,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/bans/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            user_id
        );

        let request = ChorusRequest {
            request: Client::new().delete(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.handle_request_as_result(user).await
    }

    /// Returns the number of members that would be removed in a prune operation.
    ///
    /// Requires both the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) and [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permissions.
    ///
    /// By default, a prune will not remove users with roles.
    ///
    /// You can optionally include specific roles by providing the `include_roles` parameter.
    ///
    /// Any inactive user that has a subset of the provided roles will be counted in the prune and
    /// user with additional roles will not.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-prune>
    pub async fn get_prune(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: GuildPruneParameters,
    ) -> ChorusResult<GetGuildPruneResult> {
        let url = format!(
            "{}/guilds/{}/prune",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().get(url).query(&schema.to_query()),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Begins a prune operation.
    ///
    /// Requires both the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) and [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permissions.
    ///
    /// For large guilds, it's recommended to set the `compute_prune_count` field to `false`,
    /// allowing the request to reqturn before all members are pruned.
    ///
    /// By default, a prune will not remove users with roles.
    ///
    /// You can optionally include specific roles by providing the `include_roles` parameter.
    ///
    /// Any inactive user that has a subset of the provided roles will be counted in the prune and
    /// user with additional roles will not.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#prune-guild>
    pub async fn prune(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        audit_log_reason: Option<String>,
        schema: GuildPruneSchema,
    ) -> ChorusResult<GuildPruneResult> {
        let url = format!(
            "{}/guilds/{}/prune",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().post(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns the [GuildWidgetSettings] for a guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-widget-settings>
    pub async fn get_widget_settings(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildWidgetSettings> {
        let url = format!(
            "{}/guilds/{}/widget",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Modifies the [GuildWidgetSettings] for a guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// Returns the updated object on success.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-widget-settings>
    pub async fn modify_widget(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        audit_log_reason: Option<String>,
        schema: ModifyGuildWidgetSchema,
    ) -> ChorusResult<GuildWidgetSettings> {
        let url = format!(
            "{}/guilds/{}/widget",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns the [GuildWidget] for the given guild ID.
    ///
    /// This endpoint is unauthenticated.
    ///
    /// (The guild must have the widget enabled.)
    ///
    /// If a widget channel is set and a usable invite for it does not already exist,
    /// fetching the widget will create one. Subsequent calls will attempt to reuse the generated
    /// invite.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-widget>
    pub async fn get_widget(instance: &Instance, guild_id: Snowflake) -> ChorusResult<GuildWidget> {
        let url = format!("{}/guilds/{}/widget.json", instance.urls.api, guild_id,);

        let client = Client::new().get(url.clone());

        let response = match client.send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusError::RequestFailed {
                    url,
                    error: e.to_string(),
                });
            }
        };

        if !response.status().as_str().starts_with('2') {
            return Err(ChorusError::ReceivedErrorCode {
                error_code: response.status().as_u16(),
                error: response.text().await.unwrap(),
            });
        }

        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                });
            }
        };

        match from_str::<GuildWidget>(&response_text) {
			Ok(return_value) => Ok(return_value),
			Err(e) => Err(ChorusError::InvalidResponse { error: format!("Error while trying to deserialize the JSON response into response type T: {}. JSON Response: {}",
                        e, response_text) })
		  }
    }

    /// Returns a widget image for the given guild ID.
    ///
    /// This endpoint is unauthenticated.
    ///
    /// (The guild must have the widget enabled.)
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-widget-image>
    pub async fn get_widget_image(
        instance: &Instance,
        guild_id: Snowflake,
        style: Option<GuildWidgetImageStyle>,
    ) -> ChorusResult<Bytes> {
        let url = format!("{}/guilds/{}/widget.png", instance.urls.api, guild_id,);

        let mut client = Client::new().get(url.clone());

        if let Some(style_some) = style {
            match serde_json::to_string(&style_some) {
                Err(e) => {
                    return Err(ChorusError::FormCreation {
                        error: format!("Failed to serialize: {}", e),
                    })
                }
                Ok(string) => {
                    client = client.query(&("style".to_string(), string.replace('"', "")));
                }
            }
        }

        let response = match client.send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusError::RequestFailed {
                    url,
                    error: e.to_string(),
                });
            }
        };

        if !response.status().as_str().starts_with('2') {
            return Err(ChorusError::ReceivedErrorCode {
                error_code: response.status().as_u16(),
                error: response.text().await.unwrap(),
            });
        }

        let response_bytes = match response.bytes().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into Bytes: {}",
                        e
                    ),
                });
            }
        };

        Ok(response_bytes)
    }

    /// Fetches [GuildVanityInviteInfo] for a given guild.
    ///
    /// The guild must have the [VANITY_URL](crate::types::types::guild_configuration::GuildFeatures::VanityUrl) or [GUILD_WEB_PAGE_VANITY_URL](crate::types::types::guild_configuration::GuildFeatures::GuildWebPageVanityURL) feature.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-vanity-invite>
    pub async fn get_vanity_invite(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildVanityInviteInfo> {
        let url = format!(
            "{}/guilds/{}/vanity-url",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Modifies the guild vanity invite code for a given guild.
    ///
    /// The guild must have the [VANITY_URL](crate::types::types::guild_configuration::GuildFeatures::VanityUrl) or [GUILD_WEB_PAGE_VANITY_URL](crate::types::types::guild_configuration::GuildFeatures::GuildWebPageVanityURL) feature.
    ///
    /// Guild without the [VANITY_URL](crate::types::types::guild_configuration::GuildFeatures::VanityUrl) feature can only
    /// clear their vanity invite.
    ///
    /// Requires both the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) and [CREATE_INSTANT_INVITE](crate::types::PermissionFlags::CREATE_INSTANT_INVITE) permissions.
    ///
    /// # Notes
    /// This route requires MFA.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#modify-guild-vanity-invite>
    pub async fn modify_vanity_invite(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        code: Option<String>,
    ) -> ChorusResult<GuildVanityInviteInfo> {
        let url = format!(
            "{}/guilds/{}/vanity-url",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new()
                .patch(url)
                .json(&GuildModifyVanityInviteSchema { code }),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_mfa(&user.mfa_token)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Fetches the [GuildMemberVerification] object for a given guild if one is set.
    ///
    /// If the user is not in the guild, the guild must be discoverable or have guild previewing
    /// disabled.
    ///
    /// If `with_guild` is set to `true `(it is `false` by default), the object will include
    /// [GuildMemberVerificationGuild](crate::types::GuildMemberVerificationGuild).
    ///
    /// To set it to true, the user must not be a member of the guild and the guild must not be
    /// full.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-member-verification>
    pub async fn get_member_verification(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: GetGuildMemberVerificationQuery,
    ) -> ChorusResult<GuildMemberVerification> {
        let url = format!(
            "{}/guilds/{}/member-verification",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().get(url).query(&query.to_query()),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Modifies the [GuildMemberVerification] object for the guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// Returns the updated object.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#modify-guild-member-verification>
    pub async fn modify_member_verification(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: ModifyGuildMemberVerificationSchema,
        audit_log_reason: Option<String>,
    ) -> ChorusResult<GuildMemberVerification> {
        let url = format!(
            "{}/guilds/{}/member-verification",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns a list of [GuildJoinRequest]s for the guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-requests>
    pub async fn get_join_requests(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: GetGuildJoinRequestsQuery,
    ) -> ChorusResult<GetGuildJoinRequestsReturn> {
        let url = format!(
            "{}/guilds/{}/requests",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().get(url).query(&query.to_query()),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns a specific [GuildJoinRequest].
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission if the
    /// request is not for the current user.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-request>
    pub async fn get_join_request(
        user: &mut ChorusUser,
        request_id: Snowflake,
    ) -> ChorusResult<GuildJoinRequest> {
        let url = format!(
            "{}/join-requests/{}",
            user.belongs_to.read().unwrap().urls.api,
            request_id,
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns the remaining time until the current user can submit another join request for the
    /// guild.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-request-cooldown>
    pub async fn get_join_request_cooldown(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildJoinRequestCooldown> {
        let url = format!(
            "{}/guilds/{}/requests/@me/cooldown",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().put(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Submits a request to join a guild.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#create-guild-join-request>
    pub async fn create_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: CreateGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        let url = format!(
            "{}/guilds/{}/requests/@me",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().put(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Resets the current user's join request for a guild.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#reset-guild-join-request>
    pub async fn reset_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildJoinRequest> {
        let url = format!(
            "{}/guilds/{}/requests/@me",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().post(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Acknowledges an approved join request for the current user.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#reset-guild-join-request>
    pub async fn acknowledge_approved_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/requests/@me/ack",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().post(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.handle_request_as_result(user).await
    }

    /// If the guild has previewing disabled, deletes the current user's join request.
    ///
    /// Otherwise functions the same as [Guild::reset_join_request].
    ///
    /// Returns a partial [GuildJoinRequest] if the request was reset or [None] if the deletion was
    /// successful.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#delete-guild-join-request>
    pub async fn delete_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<Option<GuildJoinRequest>> {
        let url = format!(
            "{}/guilds/{}/requests/@me",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().delete(&url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        let response = match request.request.send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusError::RequestFailed {
                    url,
                    error: e.to_string(),
                });
            }
        };

        if !response.status().as_str().starts_with('2') {
            return Err(ChorusError::ReceivedErrorCode {
                error_code: response.status().as_u16(),
                error: response.text().await.unwrap(),
            });
        }

        // Note: empty response, the request was deleted
        if response.status().as_u16() == 204 {
            return Ok(None);
        }

        // Else the request was successful, and we likely received the join request json
        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                });
            }
        };

        match from_str::<GuildJoinRequest>(&response_text) {
			Ok(return_value) => Ok(Some(return_value)),
			Err(e) => Err(ChorusError::InvalidResponse { error: format!("Error while trying to deserialize the JSON response into response type T: {}. JSON Response: {}", e, response_text) })
		  }
    }

    /// Creates or returns an existing private interview channel for a join request.
    ///
    /// Returns a [GroupDm](crate::types::ChannelType::GroupDm) [Channel] object on success.
    ///
    /// The channel will have the same id as the join request and will be accessible by the join
    /// request user and user who used this endpoint.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#create-guild-join-request-interview>
    pub async fn create_join_request_interview(
        user: &mut ChorusUser,
        request_id: Snowflake,
    ) -> ChorusResult<Channel> {
        let url = format!(
            "{}/join-requests/{}/interview",
            user.belongs_to.read().unwrap().urls.api,
            request_id,
        );

        let request = ChorusRequest {
            request: Client::new().post(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Accepts or denies a join request.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#action-guild-join-request>
    pub async fn action_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        request_id: Snowflake,
        schema: ActionGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        let url = format!(
            "{}/guilds/{}/requests/id/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            request_id,
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Accepts or denies a join request for a given user.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#action-guild-join-request-by-user>
    pub async fn action_join_request_by_user(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        user_id: Snowflake,
        schema: ActionGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        let url = format!(
            "{}/guilds/{}/requests/{}",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
            user_id,
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Accepts or denies all pending join requests for a guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#bulk-action-guild-join-requests>
    pub async fn bulk_action_join_requests(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: BulkActionGuildJoinRequestsSchema,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/requests",
            user.belongs_to.read().unwrap().urls.api,
            guild_id,
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.handle_request_as_result(user).await
    }

    /// Returns the [welcome screen](crate::types::PublicGuildWelcomeScreen) object for the guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission if the
    /// welcome screen is not yet enabled.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-welcome-screen>
    pub async fn get_welcome_screen(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<PublicGuildWelcomeScreen> {
        let url = format!(
            "{}/guilds/{}/welcome-screen",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Modifies a guild's [welcome screen](crate::types::PublicGuildWelcomeScreen) object.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#modify-guild-welcome-screen>
    pub async fn modify_welcome_screen(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: ModifyGuildWelcomeScreenSchema,
        audit_log_reason: Option<String>,
    ) -> ChorusResult<PublicGuildWelcomeScreen> {
        let url = format!(
            "{}/guilds/{}/welcome-screen",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().patch(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Returns the [onboarding](crate::types::GuildOnboarding) object for the guild.
    ///
    /// User must be a member of the guild.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-onboarding>
    pub async fn get_onboarding(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildOnboarding> {
        let url = format!(
            "{}/guilds/{}/onboarding",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Modifies a guild's [onboarding](crate::types::PublicGuildWelcomeScreen) configuration.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Notes
    /// Onboarding enforces constraints when enabled:
    ///
    /// There must be at least 7 default channels and at least 5 of them must allow sending
    /// messages by the @everyone role.
    ///
    /// The mode field modifies what is considered when enforcing these constraints.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#modify-guild-onboarding>
    pub async fn modify_onboarding(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: ModifyGuildOnboardingSchema,
        audit_log_reason: Option<String>,
    ) -> ChorusResult<GuildOnboarding> {
        let url = format!(
            "{}/guilds/{}/onboarding",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().put(url).json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    // TODO: once we have documentation on how this works, add PUT /guilds/{guild_id}/onboarding-responses

    /// Checks if the user is eligible to join the Discord Admin Community through the guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-admin-community-eligibility>
    pub async fn get_admin_community_eligibility(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<AdminCommunityEligibility> {
        let url = format!(
            "{}/guilds/{}/admin-server-eligibility",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Joins the Discord Admin Community through the guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission.
    ///
    /// Returns the joined [Guild] on success.
    ///
    /// Also see [Guild::get_admin_community_eligibility].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#join-admin-community>
    pub async fn join_admin_community(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<Guild> {
        let url = format!(
            "{}/guilds/{}/join-admin-server",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().post(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response(user).await
    }

    /// Joins the Wumpus Feedback Squad through the guild.
    ///
    /// Requires the [MANAGE_GUILD](crate::types::PermissionFlags::MANAGE_GUILD) permission and the
    /// [CLAN](crate::types::types::guild_configuration::GuildFeatures::Clan) guild feature.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#join-wumpus-feedback-squad>
    pub async fn join_wumpus_feedback_squad(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/join-wfs-server",
            user.belongs_to.read().unwrap().urls.api,
            guild_id
        );

        let request = ChorusRequest {
            request: Client::new().post(url),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.handle_request_as_result(user).await
    }
}

impl Channel {
    /// Creates a new channel in a guild.
    ///
    /// Requires the [MANAGE_CHANNELS](crate::types::PermissionFlags::MANAGE_CHANNELS) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/channel#create-guild-channel>
    pub async fn create(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        audit_log_reason: Option<String>,
        schema: ChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        let request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/guilds/{}/channels",
                    user.belongs_to.read().unwrap().urls.api,
                    guild_id
                ))
                .json(&schema),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_maybe_audit_log_reason(audit_log_reason)
        .with_headers_for(user);

        request.deserialize_response::<Channel>(user).await
    }
}

impl GuildJoinRequest {
    /// Returns a list of [GuildJoinRequest]s for the guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::get_join_requests]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-requests>
    pub async fn get_all_for_guild(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: GetGuildJoinRequestsQuery,
    ) -> ChorusResult<GetGuildJoinRequestsReturn> {
        Guild::get_join_requests(user, guild_id, query).await
    }

    /// Returns a specific [GuildJoinRequest].
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission if the
    /// request is not for the current user.
    ///
    /// # Notes
    /// This method is an alias of [Guild::get_join_request]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-request>
    pub async fn get(user: &mut ChorusUser, id: Snowflake) -> ChorusResult<GuildJoinRequest> {
        Guild::get_join_request(user, id).await
    }

    /// Returns the remaining time until the current user can submit another join request for the
    /// guild.
    ///
    /// # Notes
    /// This method is an alias of [Guild::get_join_request_cooldown]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-request-cooldown>
    pub async fn get_cooldown(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildJoinRequestCooldown> {
        Guild::get_join_request_cooldown(user, guild_id).await
    }

    /// Submits a request to join a guild.
    ///
    /// # Notes
    /// This method is an alias of [Guild::create_join_request]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#create-guild-join-request>
    pub async fn create(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: CreateGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::create_join_request(user, guild_id, schema).await
    }

    /// Accepts or denies a join request.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::action_join_request]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#action-guild-join-request>
    pub async fn action(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        request_id: Snowflake,
        schema: ActionGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::action_join_request(user, guild_id, request_id, schema).await
    }

    /// Accepts or denies a join request for a given user.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::action_join_request_by_user]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#action-guild-join-request-by-user>
    pub async fn action_by_user(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        user_id: Snowflake,
        schema: ActionGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::action_join_request_by_user(user, guild_id, user_id, schema).await
    }

    /// Accepts or denies all pending join requests for a guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::bulk_action_join_requests]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#bulk-action-guild-join-requests>
    pub async fn bulk_action(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: BulkActionGuildJoinRequestsSchema,
    ) -> ChorusResult<()> {
        Guild::bulk_action_join_requests(user, guild_id, schema).await
    }
}

impl ChorusUser {
    /// Returns a list of [GuildJoinRequest]s for the guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::get_join_requests]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-requests>
    pub async fn get_guild_join_requests(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: GetGuildJoinRequestsQuery,
    ) -> ChorusResult<GetGuildJoinRequestsReturn> {
        Guild::get_join_requests(user, guild_id, query).await
    }

    /// Returns a specific [GuildJoinRequest].
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission if the
    /// request is not for the current user.
    ///
    /// # Notes
    /// This method is an alias of [Guild::get_join_request]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-request>
    pub async fn get_guild_join_request(
        user: &mut ChorusUser,
        id: Snowflake,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::get_join_request(user, id).await
    }

    /// Returns the remaining time until the current user can submit another join request for the
    /// guild.
    ///
    /// # Notes
    /// This method is an alias of [Guild::get_join_request_cooldown]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#get-guild-join-request-cooldown>
    pub async fn get_guild_join_request_cooldown(
        user: &mut ChorusUser,
        guild_id: Snowflake,
    ) -> ChorusResult<GuildJoinRequestCooldown> {
        Guild::get_join_request_cooldown(user, guild_id).await
    }

    /// Submits a request to join a guild.
    ///
    /// # Notes
    /// This method is an alias of [Guild::create_join_request]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#create-guild-join-request>
    pub async fn create_guild_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: CreateGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::create_join_request(user, guild_id, schema).await
    }

    /// Accepts or denies a join request.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::action_join_request]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#action-guild-join-request>
    pub async fn action_guild_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        request_id: Snowflake,
        schema: ActionGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::action_join_request(user, guild_id, request_id, schema).await
    }

    /// Accepts or denies a join request for a given user.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::action_join_request_by_user]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#action-guild-join-request-by-user>
    pub async fn action_guild_join_request_by_user(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        user_id: Snowflake,
        schema: ActionGuildJoinRequestSchema,
    ) -> ChorusResult<GuildJoinRequest> {
        Guild::action_join_request_by_user(user, guild_id, user_id, schema).await
    }

    /// Accepts or denies all pending join requests for a guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// Also requires that the guild have the [MemberVerificationManualApproval](crate::types::types::guild_configuration::GuildFeatures::MemberVerificationManualApproval) feature.
    ///
    /// # Notes
    /// This method is an alias of [Guild::bulk_action_join_requests]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/guild#bulk-action-guild-join-requests>
    pub async fn bulk_action_guild_join_request(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        schema: BulkActionGuildJoinRequestsSchema,
    ) -> ChorusResult<()> {
        Guild::bulk_action_join_requests(user, guild_id, schema).await
    }
}
