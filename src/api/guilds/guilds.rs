use serde_json::from_str;
use serde_json::to_string;

use crate::api::schemas;
use crate::api::types;
use crate::errors::InstanceServerError;

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
    /// ```rs
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
        url_api: &str,
        guild_create_schema: schemas::GuildCreateSchema,
    ) -> Result<String, crate::errors::InstanceServerError> {
        let url = format!("{}/guilds/", url_api);
        let limits_user = user.limits.get_as_mut();
        let limits_instance = &mut user.belongs_to.limits;
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
        let id: types::GuildCreateResponse = from_str(&result.text().await.unwrap()).unwrap();
        Ok(id.id)
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
        user: &mut types::User<'a>,
        url_api: &str,
        guild_id: String,
    ) -> Option<InstanceServerError> {
        let url = format!("{}/guilds/{}/delete/", url_api, guild_id);
        let limits_user = user.limits.get_as_mut();
        let limits_instance = &mut user.belongs_to.limits;
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
}

#[cfg(test)]
mod test {
    use crate::api::schemas;
    use crate::api::types;
    use crate::instance::Instance;

    #[tokio::test]
    async fn guild_creation_deletion() {
        let mut instance = Instance::new(
            crate::URLBundle {
                api: "http://localhost:3001/api".to_string(),
                wss: "ws://localhost:3001/".to_string(),
                cdn: "http://localhost:3001".to_string(),
            },
            crate::limit::LimitedRequester::new().await,
        )
        .await
        .unwrap();
        let login_schema: schemas::LoginSchema = schemas::LoginSchema::new(
            schemas::AuthUsername::new("user@test.xyz".to_string()).unwrap(),
            "transrights".to_string(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let mut user = instance.login_account(&login_schema).await.unwrap();

        let guild_create_schema = schemas::GuildCreateSchema {
            name: Some("test".to_string()),
            region: None,
            icon: None,
            channels: None,
            guild_template_code: None,
            system_channel_id: None,
            rules_channel_id: None,
        };

        let guild =
            types::Guild::create(&mut user, "http://localhost:3001/api", guild_create_schema)
                .await
                .unwrap();

        println!("{}", guild);

        match types::Guild::delete(&mut user, "http://localhost:3001/api", guild).await {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
}
