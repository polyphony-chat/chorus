use serde_json::to_string;

use crate::api::schemas;
use crate::api::types;

impl<'a> types::Guild {
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
    /// A `Result<String>` containing the ID of the newly created guild, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// Returns an `InstanceServerError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let guild_create_schema = chorus::api::schemas::GuildCreateSchema::new(insert args here);
    ///
    /// let result = Guild::create(&mut user, &mut instance, &guild_create_schema).await;
    ///
    /// match result {
    ///     Ok(guild_id) => println!("Created guild with ID {}", guild_id),
    ///     Err(e) => println!("Failed to create guild: {}", e),
    /// }
    /// ```
    pub async fn create(
        user: &mut types::User<'a>,
        instance: &mut crate::instance::Instance,
        guild_create_schema: schemas::GuildCreateSchema,
    ) -> Result<String, crate::errors::InstanceServerError> {
        let url = format!("{}/guilds/", instance.urls.get_api().to_string());
        let limits_user = user.limits.get_as_mut();
        let limits_instance = instance.limits.get_as_mut();
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
        return Ok(match result.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(crate::errors::InstanceServerError::RequestErrorError {
                    url: url.to_string(),
                    error: e.to_string(),
                })
            }
        });
    }
    pub async fn get(
        user: &mut types::User<'a>,
        instance: &mut crate::instance::Instance,
        id: String,
    ) {
        let url = format!("{}/guilds/{}/", instance.urls.get_api().to_string(), id);
        let limits_user = user.limits.get_as_mut();
        let limits_instance = instance.limits.get_as_mut();
        let request = reqwest::Client::new()
            .get(url.clone())
            .bearer_auth(user.token.clone());
    }
}
