use reqwest::Client;
use serde_json::from_str;
use serde_json::to_string;

use crate::api::LimitType;
use crate::errors::ChorusError;
use crate::errors::ChorusResult;
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::Snowflake;
use crate::types::{Channel, ChannelCreateSchema, Guild, GuildCreateSchema};

impl Guild {
    /// Creates a new guild.
    pub async fn create(
        user: &mut UserMeta,
        guild_create_schema: GuildCreateSchema,
    ) -> ChorusResult<Guild> {
        let url = format!("{}/guilds/", user.belongs_to.borrow().urls.api);
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(url.clone())
                .bearer_auth(user.token.clone())
                .body(to_string(&guild_create_schema).unwrap()),
            limit_type: LimitType::Global,
        };
        chorus_request.deserialize_response::<Guild>(user).await
    }

    /// Deletes a guild by its id.
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
    pub async fn delete(user: &mut UserMeta, guild_id: Snowflake) -> ChorusResult<()> {
        let url = format!(
            "{}/guilds/{}/delete/",
            user.belongs_to.borrow().urls.api,
            guild_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(url.clone())
                .bearer_auth(user.token.clone()),
            limit_type: LimitType::Global,
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Creates a new channel in a guild.
    pub async fn create_channel(
        &self,
        user: &mut UserMeta,
        schema: ChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        Channel::create(user, self.id, schema).await
    }

    /// Returns a list of the guild's channels
    pub async fn channels(&self, user: &mut UserMeta) -> ChorusResult<Vec<Channel>> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/guilds/{}/channels/",
                    user.belongs_to.borrow().urls.api,
                    self.id
                ))
                .bearer_auth(user.token()),
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
    pub async fn get(guild_id: Snowflake, user: &mut UserMeta) -> ChorusResult<Guild> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(format!(
                    "{}/guilds/{}/",
                    user.belongs_to.borrow().urls.api,
                    guild_id
                ))
                .bearer_auth(user.token()),
            limit_type: LimitType::Guild(guild_id),
        };
        let response = chorus_request.deserialize_response::<Guild>(user).await?;
        Ok(response)
    }
}

impl Channel {
    /// Creates a new channel in a guild.
    pub async fn create(
        user: &mut UserMeta,
        guild_id: Snowflake,
        schema: ChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        let chorus_request = ChorusRequest {
            request: Client::new()
                .post(format!(
                    "{}/guilds/{}/channels/",
                    user.belongs_to.borrow().urls.api,
                    guild_id
                ))
                .bearer_auth(user.token())
                .body(to_string(&schema).unwrap()),
            limit_type: LimitType::Guild(guild_id),
        };
        chorus_request.deserialize_response::<Channel>(user).await
    }
}
