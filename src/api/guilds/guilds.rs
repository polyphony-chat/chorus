use reqwest::Client;
use serde_json::from_str;
use serde_json::to_string;

use crate::api::LimitType;
use crate::errors::ChorusError;
use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::ratelimiter::ChorusRequest;
use crate::types::{
    Channel, ChannelCreateSchema, Guild, GuildBanCreateSchema, GuildCreateSchema, GuildMember,
    GuildMemberSearchSchema, GuildModifySchema, GuildPreview, ModifyGuildMemberSchema,
};
use crate::types::{GuildBan, Snowflake};

impl Guild {
    /// Creates a new guild.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#create-guild>
    pub async fn create(
        user: &mut ChorusUser,
        guild_create_schema: GuildCreateSchema,
    ) -> ChorusResult<Guild> {
        let url = format!("{}/guilds", user.belongs_to.borrow().urls.api);
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(url.clone())
                .header("Authorization", user.token.clone())
                .header("Content-Type", "application/json")
                .body(to_string(&guild_create_schema).unwrap()),
            limit_type: LimitType::Global,
        };
        chorus_request.deserialize_response::<Guild>(user).await
    }

    /// Deletes a guild by its id.
    ///
    /// User must be the owner.
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut user = User::new();
    /// let mut instance = Instance::new();
    /// let guild_id = String::from("1234567890");
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
            user.belongs_to.borrow().urls.api,
            guild_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(url.clone())
                .header("Authorization", user.token.clone())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Global,
        };
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
            request: Client::new()
                .get(format!(
                    "{}/guilds/{}/channels",
                    user.belongs_to.borrow().urls.api,
                    self.id
                ))
                .header("Authorization", user.token()),
            limit_type: LimitType::Channel(self.id),
        };
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

    /// Fetches a guild by its id.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild>
    pub async fn get(guild_id: Snowflake, user: &mut ChorusUser) -> ChorusResult<Guild> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/guilds/{}",
                    user.belongs_to.borrow().urls.api,
                    guild_id
                ))
                .header("Authorization", user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
    }

    pub async fn create_ban(
        guild_id: Snowflake,
        user_id: Snowflake,
        schema: GuildBanCreateSchema,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildBan> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .put(format!(
                    "{}/guilds/{}/bans/{}",
                    user.belongs_to.borrow().urls.api,
                    guild_id,
                    user_id
                ))
                .header("Authorization", user.token())
                .body(to_string(&schema).unwrap()),
            limit_type: LimitType::Guild(guild_id),
        };
        let response = chorus_request
            .deserialize_response::<GuildBan>(user)
            .await?;
        Ok(response)
    }

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
                    user.belongs_to.borrow().urls.api,
                    guild_id,
                ))
                .header("Authorization", user.token())
                .header("Content-Type", "application/json")
                .body(to_string(&schema).unwrap()),
            limit_type: LimitType::Guild(guild_id),
        };
        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
    }

    /// Returns a guild preview object for the given guild ID. If the user is not in the guild, the guild must be discoverable.
    /// # Reference:
    ///
    /// See <https://discord-userdoccers.vercel.app/resources/guild#get-guild-preview>
    pub async fn get_preview(
        guild_id: Snowflake,
        user: &mut ChorusUser,
    ) -> ChorusResult<GuildPreview> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .patch(format!(
                    "{}/guilds/{}/preview",
                    user.belongs_to.borrow().urls.api,
                    guild_id,
                ))
                .header("Authorization", user.token())
                .header("Content-Type", "application/json"),
            limit_type: LimitType::Guild(guild_id),
        };
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
        let request = ChorusRequest::new(
            http::Method::GET,
            format!(
                "{}/guilds/{}/members",
                user.belongs_to.borrow().urls.api,
                guild_id,
            )
            .as_str(),
            None,
            None,
            None,
            Some(user),
            LimitType::Guild(guild_id),
        );
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
        let mut request = ChorusRequest::new(
            http::Method::GET,
            format!(
                "{}/guilds/{}/members/search",
                user.belongs_to.borrow().urls.api,
                guild_id,
            )
            .as_str(),
            None,
            None,
            None,
            Some(user),
            LimitType::Guild(guild_id),
        );
        request.request = request
            .request
            .query(&[("query", to_string(&query).unwrap())]);
        request.deserialize_response::<Vec<GuildMember>>(user).await
    }

    /// Removes a member from a guild. Requires the KICK_MEMBERS permission. Returns a 204 empty response on success.
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/guild#remove-guild-member>
    pub async fn remove_member(
        guild_id: Snowflake,
        member_id: Snowflake,
        audit_log_reason: Option<String>,
        user: &mut ChorusUser,
    ) -> ChorusResult<()> {
        let request = ChorusRequest::new(
            http::Method::DELETE,
            format!(
                "{}/guilds/{}/members/{}",
                user.belongs_to.borrow().urls.api,
                guild_id,
                member_id,
            )
            .as_str(),
            None,
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Guild(guild_id),
        );
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
    ) -> ChorusResult<()> {
        let request = ChorusRequest::new(
            http::Method::PATCH,
            format!(
                "{}/guilds/{}/members/{}",
                user.belongs_to.borrow().urls.api,
                guild_id,
                member_id,
            )
            .as_str(),
            Some(to_string(&schema).unwrap()),
            audit_log_reason.as_deref(),
            None,
            Some(user),
            LimitType::Guild(guild_id),
        );
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
        let mut request = Client::new()
            .post(format!(
                "{}/guilds/{}/channels",
                user.belongs_to.borrow().urls.api,
                guild_id
            ))
            .header("Authorization", user.token())
            .header("Content-Type", "application/json")
            .body(to_string(&schema).unwrap());
        if let Some(reason) = audit_log_reason {
            request = request.header("X-Audit-Log-Reason", reason);
        }
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request.deserialize_response::<Channel>(user).await
    }
}
