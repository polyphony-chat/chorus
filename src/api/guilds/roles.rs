use reqwest::Client;
use serde_json::{from_str, to_string};

use crate::{
    errors::ChorusLibError,
    instance::UserMeta,
    limit::LimitedRequester,
    types::{self, RoleCreateModifySchema, RoleObject},
};

impl types::RoleObject {
    /// Retrieves all roles for a given guild.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `guild_id` - The ID of the guild to retrieve roles from.
    ///
    /// # Returns
    ///
    /// An `Option` containing a `Vec` of [`RoleObject`]s if roles were found, or `None` if no roles were found.
    ///
    /// # Errors
    ///
    /// Returns a [`ChorusLibError`] if the request fails or if the response is invalid.
    pub async fn get_all(
        user: &mut UserMeta,
        guild_id: &str,
    ) -> Result<Option<Vec<RoleObject>>, crate::errors::ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!("{}/guilds/{}/roles/", belongs_to.urls.get_api(), guild_id);
        let request = Client::new().get(url).bearer_auth(user.token());
        let requester = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(e),
        };
        let roles: Vec<RoleObject> = from_str(&requester.text().await.unwrap()).unwrap();

        if roles.is_empty() {
            return Ok(None);
        }

        Ok(Some(roles))
    }

    /// Retrieves a single role for a given guild.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `guild_id` - The ID of the guild to retrieve the role from.
    /// * `role_id` - The ID of the role to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the retrieved [`RoleObject`] if successful, or a [`ChorusLibError`] if the request fails or if the response is invalid.
    ///
    /// # Errors
    ///
    /// Returns a [`ChorusLibError`] if the request fails or if the response is invalid.
    pub async fn get(
        user: &mut UserMeta,
        guild_id: &str,
        role_id: &str,
    ) -> Result<RoleObject, crate::errors::ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/guilds/{}/roles/{}/",
            belongs_to.urls.get_api(),
            guild_id,
            role_id
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        let requester = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(e),
        };
        let role: RoleObject = from_str(&requester.text().await.unwrap()).unwrap();

        Ok(role)
    }

    /// Creates a new role for a given guild.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `guild_id` - The ID of the guild to create the role in.
    /// * `role_create_schema` - A [`RoleCreateModifySchema`] instance containing the properties of the role to be created.
    ///
    /// # Returns
    ///
    /// A `Result` containing the newly created [`RoleObject`] if successful, or a [`ChorusLibError`] if the request fails or if the response is invalid.
    ///
    /// # Errors
    ///
    /// Returns a [`ChorusLibError`] if the request fails or if the response is invalid.
    pub async fn create(
        user: &mut UserMeta,
        guild_id: &str,
        role_create_schema: RoleCreateModifySchema,
    ) -> Result<RoleObject, ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!("{}/guilds/{}/roles/", belongs_to.urls.get_api(), guild_id);
        let body = match to_string::<RoleCreateModifySchema>(&role_create_schema) {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                })
            }
        };
        let request = Client::new().post(url).bearer_auth(user.token()).body(body);
        let result = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(e),
        };
        let role: RoleObject = match from_str(&result.text().await.unwrap()) {
            Ok(role) => role,
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                })
            }
        };
        Ok(role)
    }

    /// Updates the position of a role in the guild's hierarchy.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `guild_id` - The ID of the guild to update the role position in.
    /// * `role_position_update_schema` - A [`RolePositionUpdateSchema`] instance containing the new position of the role.
    ///
    /// # Returns
    ///
    /// A `Result` containing the updated [`RoleObject`] if successful, or a [`ChorusLibError`] if the request fails or if the response is invalid.
    ///
    /// # Errors
    ///
    /// Returns a [`ChorusLibError`] if the request fails or if the response is invalid.
    pub async fn position_update(
        user: &mut UserMeta,
        guild_id: &str,
        role_position_update_schema: types::RolePositionUpdateSchema,
    ) -> Result<RoleObject, ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!("{}/guilds/{}/roles/", belongs_to.urls.get_api(), guild_id);
        let body = match to_string(&role_position_update_schema) {
            Ok(body) => body,
            Err(e) => {
                return Err(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                })
            }
        };
        let request = Client::new()
            .patch(url)
            .bearer_auth(user.token())
            .body(body);
        let response = LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
            .unwrap();
        let role: RoleObject = match from_str(&response.text().await.unwrap()) {
            Ok(role) => role,
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                })
            }
        };
        Ok(role)
    }

    /// Updates a role in a guild.
    ///
    /// # Arguments
    ///
    /// * `user` - A mutable reference to a [`UserMeta`] instance.
    /// * `guild_id` - The ID of the guild to update the role in.
    /// * `role_id` - The ID of the role to update.
    /// * `role_create_schema` - A [`RoleCreateModifySchema`] instance containing the new properties of the role.
    ///
    /// # Returns
    ///
    /// A `Result` containing the updated [`RoleObject`] if successful, or a [`ChorusLibError`] if the request fails or if the response is invalid.
    ///
    /// # Errors
    ///
    /// Returns a [`ChorusLibError`] if the request fails or if the response is invalid.
    pub async fn update(
        user: &mut UserMeta,
        guild_id: &str,
        role_id: &str,
        role_create_schema: RoleCreateModifySchema,
    ) -> Result<RoleObject, ChorusLibError> {
        let mut belongs_to = user.belongs_to.borrow_mut();
        let url = format!(
            "{}/guilds/{}/roles/{}",
            belongs_to.urls.get_api(),
            guild_id,
            role_id
        );
        let body = match to_string::<RoleCreateModifySchema>(&role_create_schema) {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusLibError::FormCreationError {
                    error: e.to_string(),
                })
            }
        };
        let request = Client::new()
            .patch(url)
            .bearer_auth(user.token())
            .body(body);
        let result = match LimitedRequester::new()
            .await
            .send_request(
                request,
                crate::api::limits::LimitType::Guild,
                &mut belongs_to.limits,
                &mut user.limits,
            )
            .await
        {
            Ok(request) => request,
            Err(e) => return Err(e),
        };
        let role: RoleObject = match from_str(&result.text().await.unwrap()) {
            Ok(role) => role,
            Err(e) => {
                return Err(ChorusLibError::InvalidResponseError {
                    error: e.to_string(),
                })
            }
        };
        Ok(role)
    }
}
