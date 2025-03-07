// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::from_str;
use serde_json::to_string;

use crate::errors::ChorusError;
use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    Channel, ChannelCreateSchema, Guild, GuildBanCreateSchema, GuildBansQuery, GuildCreateSchema,
    GuildMember, GuildMemberSearchSchema, GuildModifySchema, GuildPreview, LimitType,
    ModifyGuildMemberProfileSchema, ModifyGuildMemberSchema, UserProfileMetadata,
};
use crate::types::{GuildBan, Snowflake};

impl Guild {
    /// Fetches a guild by its id.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild>
    pub async fn get(guild_id: Snowflake, user: &mut ChorusUser) -> ChorusResult<Guild> {
        let chorus_request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/guilds/{}",
                user.belongs_to.read().unwrap().urls.api,
                guild_id
            )),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
    }

    /// Creates a new guild.
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
    /// # Notes
    /// This route requires MFA.
    ///
    /// # Reference
    /// <https://discord-userdoccers.vercel.app/resources/guild#modify-guild>
    pub async fn modify(
        guild_id: Snowflake,
        schema: GuildModifySchema,
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
        .with_headers_for(user);

        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
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
    /// ```rs
    /// let mut user: ChorusUser;
    /// let guild_id = Snowflake::from(1234567890);
    ///
    /// match Guild::delete(&mut user, guild_id) {
    ///     Err(e) => println!("Error deleting guild: {:?}", e),
    ///     Ok(_) => println!("Guild deleted successfully"),
    /// }
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
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-preview>
    pub async fn get_preview(
        guild_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildPreview> {
        let chorus_request = ChorusRequest {
            request: Client::new().patch(format!(
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
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-members>
    pub async fn get_members(
        guild_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<GuildMember>> {
        let request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/guilds/{}/members",
                user.belongs_to.read().unwrap().urls.api,
                guild_id,
            )),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.deserialize_response::<Vec<GuildMember>>(user).await
    }

    /// Returns a list of guild member objects whose username or nickname starts with a provided string.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#search-guild-members>
    pub async fn search_members(
        guild_id: Snowflake,
        query: GuildMemberSearchSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<GuildMember>> {
        let mut request = ChorusRequest {
            request: Client::new().get(format!(
                "{}/guilds/{}/members/search",
                user.belongs_to.read().unwrap().urls.api,
                guild_id,
            )),
            limit_type: LimitType::Guild(guild_id),
        }
        .with_headers_for(user);

        request.request = request
            .request
            .query(&[("query", to_string(&query).unwrap())]);

        request.deserialize_response::<Vec<GuildMember>>(user).await
    }

    /// Removes a member from a guild.
    ///
    /// Requires the [KICK_MEMBERS](crate::types::PermissionFlags::KICK_MEMBERS) permission.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#remove-guild-member>
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

        request.handle_request_as_result(user).await
    }

    /// Modifies attributes of a guild member. Returns the updated guild member object on success.
    /// For required Permissions and an API reference, see:
    ///
    /// # Reference:
    /// <https://discord-userdoccers.vercel.app/resources/guild#modify-guild-member>
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

        request.deserialize_response::<GuildMember>(user).await
    }

    /// Modifies the current user's member in the guild.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#modify-current-guild-member>
    pub async fn modify_current_member(
        guild_id: Snowflake,
        schema: ModifyGuildMemberSchema,
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

        request.deserialize_response::<GuildMember>(user).await
    }

    /// Modifies the current user's profile in the guild.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#modify-guild-member-profile>
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
            .deserialize_response::<UserProfileMetadata>(user)
            .await
    }

    /// Returns a list of ban objects for the guild.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-bans>
    pub async fn get_bans(
        user: &mut ChorusUser,
        guild_id: Snowflake,
        query: Option<GuildBansQuery>,
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
            request.request = request.request.query(&to_string(&query).unwrap());
        }

        request.deserialize_response::<Vec<GuildBan>>(user).await
    }

    /// Returns a ban object for the given user.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-ban>
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

    /// Creates a ban from the guild.
    ///
    /// Requires the [BAN_MEMBERS](crate::types::PermissionFlags::BAN_MEMBERS) permission.
    ///
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
