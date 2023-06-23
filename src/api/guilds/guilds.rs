use reqwest::Client;
use serde_json::from_str;
use serde_json::to_string;

use crate::api::deserialize_response;
use crate::api::handle_request;
use crate::api::handle_request_as_result;
use crate::api::limits::Limits;
use crate::errors::ChorusLibError;
use crate::errors::ChorusResult;
use crate::instance::Instance;
use crate::instance::UserMeta;
use crate::limit::LimitedRequester;
use crate::types::Snowflake;
use crate::types::{Channel, ChannelCreateSchema, Guild, GuildCreateSchema};

impl Guild {
    /// Creates a new guild with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to the user creating the guild.
    /// * `instance` - A mutable reference to the instance where the guild will be created.
    /// * `guild_create_schema` - A reference to the schema containing the guild creation parameters.
    ///
    /// # Returns
    ///
    /// A `Result<Guild>` containing the object of the newly created guild, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an `ChorusLibError` if the request fails.
    ///
    pub async fn create(
        user: &mut UserMeta,
        guild_create_schema: GuildCreateSchema,
    ) -> ChorusResult<Guild> {
        let url = format!("{}/guilds/", user.belongs_to.urls.api);
        let request = reqwest::Client::new()
            .post(url.clone())
            .bearer_auth(user.token.clone())
            .body(to_string(&guild_create_schema).unwrap());
        deserialize_response::<Guild>(request, user, crate::api::limits::LimitType::Guild).await
    }

    /// Deletes a guild.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a `User` instance.
    /// * `instance` - A mutable reference to an `Instance` instance.
    /// * `guild_id` - ID of the guild to delete.
    ///
    /// # Returns
    ///
    /// An `Result` containing an `ChorusLibError` if an error occurred during the request, otherwise `()`.
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut user = User::new();
    /// let mut instance = Instance::new();
    /// let guild_id = String::from("1234567890");
    ///
    /// match Guild::delete(&mut user, &mut instance, guild_id) {
    ///     Some(e) => println!("Error deleting guild: {:?}", e),
    ///     None => println!("Guild deleted successfully"),
    /// }
    /// ```
    pub async fn delete(user: &mut UserMeta, guild_id: Snowflake) -> ChorusResult<()> {
        let url = format!("{}/guilds/{}/delete/", user.belongs_to.urls.api, guild_id);
        let request = reqwest::Client::new()
            .post(url.clone())
            .bearer_auth(user.token.clone());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Guild).await
    }

    /// Sends a request to create a new channel in the guild.
    ///
    /// # Arguments
    ///
    /// * `url_api` - The base URL for the Discord API.
    /// * `token` - A Discord bot token.
    /// * `schema` - A `ChannelCreateSchema` struct containing the properties of the new channel.
    /// * `limits_user` - A mutable reference to a `Limits` struct containing the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` struct containing the instance's rate limits.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `reqwest::Response` if the request was successful, or an `ChorusLibError` if there was an error.
    pub async fn create_channel(
        &self,
        user: &mut UserMeta,
        schema: ChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        Channel::_create(
            &user.token,
            self.id,
            schema,
            &mut user.limits,
            &user.belongs_to,
        )
        .await
    }

    /// Returns a `Result` containing a vector of `Channel` structs if the request was successful, or an `ChorusLibError` if there was an error.
    ///
    /// # Arguments
    ///
    /// * `url_api` - A string slice that holds the URL of the API.
    /// * `token` - A string slice that holds the authorization token.
    /// * `limits_user` - A mutable reference to a `Limits` struct containing the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` struct containing the instance's rate limits.
    ///
    pub async fn channels(&self, user: &mut UserMeta) -> ChorusResult<Vec<Channel>> {
        let request = Client::new()
            .get(format!(
                "{}/guilds/{}/channels/",
                user.belongs_to.urls.api, self.id
            ))
            .bearer_auth(user.token());
        let result = handle_request(request, user, crate::api::limits::LimitType::Channel)
            .await
            .unwrap();
        let stringed_response = match result.text().await {
            Ok(value) => value,
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                });
            }
        };
        let _: Vec<Channel> = match from_str(&stringed_response) {
            Ok(result) => return Ok(result),
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                });
            }
        };
    }

    /// Returns a `Result` containing a `Guild` struct if the request was successful, or an `ChorusLibError` if there was an error.
    ///
    /// # Arguments
    ///
    /// * `url_api` - A string slice that holds the URL of the API.
    /// * `guild_id` - ID of the guild.
    /// * `token` - A string slice that holds the authorization token.
    /// * `limits_user` - A mutable reference to a `Limits` struct containing the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` struct containing the instance's rate limits.
    ///
    pub async fn get(user: &mut UserMeta, guild_id: Snowflake) -> ChorusResult<Guild> {
        Guild::_get(guild_id, &user.token, &mut user.limits, &user.belongs_to).await
    }

    /// For internal use. Does the same as the public get method, but does not require a second, mutable
    /// borrow of `UserMeta::belongs_to`, when used in conjunction with other methods, which borrow `UserMeta::belongs_to`.
    async fn _get(
        guild_id: Snowflake,
        token: &str,
        limits_user: &mut Limits,
        instance: &Instance,
    ) -> ChorusResult<Guild> {
        let request = Client::new()
            .get(format!("{}/guilds/{}/", instance.urls.api, guild_id))
            .bearer_auth(token);
        let response = match LimitedRequester::send_request(
            request,
            crate::api::limits::LimitType::Guild,
            instance,
            limits_user,
        )
        .await
        {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let guild: Guild = from_str(&response.text().await.unwrap()).unwrap();
        Ok(guild)
    }
}

impl Channel {
    /// Sends a request to create a new channel in a guild.
    ///
    /// # Arguments
    ///
    /// * `token` - A Discord bot token.
    /// * `url_api` - The base URL for the Discord API.
    /// * `guild_id` - The ID of the guild where the channel will be created.
    /// * `schema` - A `ChannelCreateSchema` struct containing the properties of the new channel.
    /// * `limits_user` - A mutable reference to a `Limits` struct containing the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` struct containing the instance's rate limits.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `reqwest::Response` if the request was successful, or an `ChorusLibError` if there was an error.
    pub async fn create(
        user: &mut UserMeta,
        guild_id: Snowflake,
        schema: ChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        Channel::_create(
            &user.token,
            guild_id,
            schema,
            &mut user.limits,
            &user.belongs_to,
        )
        .await
    }

    async fn _create(
        token: &str,
        guild_id: Snowflake,
        schema: ChannelCreateSchema,
        limits_user: &mut Limits,
        instance: &Instance,
    ) -> ChorusResult<Channel> {
        let request = Client::new()
            .post(format!(
                "{}/guilds/{}/channels/",
                instance.urls.api, guild_id
            ))
            .bearer_auth(token)
            .body(to_string(&schema).unwrap());
        let result = match LimitedRequester::send_request(
            request,
            crate::api::limits::LimitType::Guild,
            instance,
            limits_user,
        )
        .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        match from_str::<Channel>(&result.text().await.unwrap()) {
            Ok(object) => Ok(object),
            Err(e) => Err(ChorusLibError::RequestErrorError {
                url: format!("{}/guilds/{}/channels/", instance.urls.api, guild_id),
                error: e.to_string(),
            }),
        }
    }
}
