use reqwest::Client;
use serde_json::from_str;
use serde_json::to_string;

use crate::api::limits::Limits;
use crate::errors::InstanceServerError;
use crate::instance::UserMeta;
use crate::limit::LimitedRequester;
use crate::types::{Channel, ChannelCreateSchema, Guild, GuildCreateResponse, GuildCreateSchema};

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
    /// Returns an `InstanceServerError` if the request fails.
    ///
    pub async fn create(
        user: &mut UserMeta,
        guild_create_schema: GuildCreateSchema,
    ) -> Result<Guild, crate::errors::InstanceServerError> {
        let belongs_to = user.belongs_to.borrow_mut();
        let url = format!("{}/guilds/", belongs_to.urls.get_api());
        let mut limits_user = user.limits.get_as_mut();
        let mut limits_instance = &mut user.belongs_to.borrow_mut().limits;
        let request = reqwest::Client::new()
            .post(url.clone())
            .bearer_auth(user.token.clone())
            .body(to_string(&guild_create_schema).unwrap());
        let mut requester = crate::limit::LimitedRequester::new().await;
        let result = match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
                limits_user,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        let id: GuildCreateResponse = from_str(&result.text().await.unwrap()).unwrap();
        let guild = Guild::get(
            belongs_to.urls.get_api(),
            &id.id,
            &user.token,
            &mut limits_user,
            &mut limits_instance,
        )
        .await
        .unwrap();
        Ok(guild)
    }

    /// Deletes a guild.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a `User` instance.
    /// * `instance` - A mutable reference to an `Instance` instance.
    /// * `guild_id` - A `String` representing the ID of the guild to delete.
    ///
    /// # Returns
    ///
    /// An `Option` containing an `InstanceServerError` if an error occurred during the request, otherwise `None`.
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
    pub async fn delete(
        user: &mut UserMeta,
        url_api: &str,
        guild_id: &str,
    ) -> Option<InstanceServerError> {
        let url = format!("{}/guilds/{}/delete/", url_api, guild_id);
        let limits_user = user.limits.get_as_mut();
        let limits_instance = &mut user.belongs_to.borrow_mut().limits;
        let request = reqwest::Client::new()
            .post(url.clone())
            .bearer_auth(user.token.clone());
        let mut requester = crate::limit::LimitedRequester::new().await;
        let result = requester
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
                limits_user,
            )
            .await;
        if result.is_err() {
            Some(result.err().unwrap())
        } else {
            None
        }
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
    /// A `Result` containing a `reqwest::Response` if the request was successful, or an `InstanceServerError` if there was an error.
    pub async fn create_channel(
        &self,
        url_api: &str,
        token: &str,
        schema: ChannelCreateSchema,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Result<Channel, InstanceServerError> {
        Channel::create(
            token,
            url_api,
            &self.id.to_string(),
            schema,
            limits_user,
            limits_instance,
        )
        .await
    }

    /// Returns a `Result` containing a vector of `Channel` structs if the request was successful, or an `InstanceServerError` if there was an error.
    ///
    /// # Arguments
    ///
    /// * `url_api` - A string slice that holds the URL of the API.
    /// * `token` - A string slice that holds the authorization token.
    /// * `limits_user` - A mutable reference to a `Limits` struct containing the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` struct containing the instance's rate limits.
    ///
    pub async fn channels(
        &self,
        url_api: &str,
        token: &str,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Result<Vec<Channel>, InstanceServerError> {
        let request = Client::new()
            .get(format!(
                "{}/guilds/{}/channels/",
                url_api,
                self.id.to_string()
            ))
            .bearer_auth(token);
        let result = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
                limits_user,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        let stringed_response = match result.text().await {
            Ok(value) => value,
            Err(e) => {
                return Err(InstanceServerError::InvalidResponseError {
                    error: e.to_string(),
                })
            }
        };
        let _: Vec<Channel> = match from_str(&stringed_response) {
            Ok(result) => return Ok(result),
            Err(e) => {
                return Err(InstanceServerError::InvalidResponseError {
                    error: e.to_string(),
                })
            }
        };
    }

    /// Returns a `Result` containing a `Guild` struct if the request was successful, or an `InstanceServerError` if there was an error.
    ///
    /// # Arguments
    ///
    /// * `url_api` - A string slice that holds the URL of the API.
    /// * `guild_id` - A string slice that holds the ID of the guild.
    /// * `token` - A string slice that holds the authorization token.
    /// * `limits_user` - A mutable reference to a `Limits` struct containing the user's rate limits.
    /// * `limits_instance` - A mutable reference to a `Limits` struct containing the instance's rate limits.
    ///
    pub async fn get(
        url_api: &str,
        guild_id: &str,
        token: &str,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Result<Guild, InstanceServerError> {
        let request = Client::new()
            .get(format!("{}/guilds/{}/", url_api, guild_id))
            .bearer_auth(token);
        let response = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
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
    /// A `Result` containing a `reqwest::Response` if the request was successful, or an `InstanceServerError` if there was an error.
    pub async fn create(
        token: &str,
        url_api: &str,
        guild_id: &str,
        schema: ChannelCreateSchema,
        limits_user: &mut Limits,
        limits_instance: &mut Limits,
    ) -> Result<Channel, InstanceServerError> {
        let request = Client::new()
            .post(format!("{}/guilds/{}/channels/", url_api, guild_id))
            .bearer_auth(token)
            .body(to_string(&schema).unwrap());
        let mut requester = LimitedRequester::new().await;
        let result = match requester
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                limits_instance,
                limits_user,
            )
            .await
        {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        match from_str::<Channel>(&result.text().await.unwrap()) {
            Ok(object) => Ok(object),
            Err(e) => Err(InstanceServerError::RequestErrorError {
                url: format!("{}/guilds/{}/channels/", url_api, guild_id),
                error: e.to_string(),
            }),
        }
    }
}
