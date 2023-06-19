use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::deserialize_response,
    errors::ChorusLibError,
    instance::UserMeta,
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
    ) -> Result<Option<Vec<RoleObject>>, ChorusLibError> {
        let url = format!(
            "{}/guilds/{}/roles/",
            user.belongs_to.borrow().urls.get_api(),
            guild_id
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        let roles = deserialize_response::<Vec<RoleObject>>(
            request,
            user,
            crate::api::limits::LimitType::Guild,
        )
        .await
        .unwrap();
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
    ) -> Result<RoleObject, ChorusLibError> {
        let url = format!(
            "{}/guilds/{}/roles/{}/",
            user.belongs_to.borrow().urls.get_api(),
            guild_id,
            role_id
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        deserialize_response(request, user, crate::api::limits::LimitType::Guild).await
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
        let url = format!(
            "{}/guilds/{}/roles/",
            user.belongs_to.borrow().urls.get_api(),
            guild_id
        );
        let body = to_string::<RoleCreateModifySchema>(&role_create_schema).map_err(|e| {
            ChorusLibError::FormCreationError {
                error: e.to_string(),
            }
        })?;
        let request = Client::new().post(url).bearer_auth(user.token()).body(body);
        deserialize_response(request, user, crate::api::limits::LimitType::Guild).await
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
        let url = format!(
            "{}/guilds/{}/roles/",
            user.belongs_to.borrow().urls.get_api(),
            guild_id
        );
        let body = to_string(&role_position_update_schema).map_err(|e| {
            ChorusLibError::FormCreationError {
                error: e.to_string(),
            }
        })?;
        let request = Client::new()
            .patch(url)
            .bearer_auth(user.token())
            .body(body);
        deserialize_response::<RoleObject>(request, user, crate::api::limits::LimitType::Guild)
            .await
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
        let url = format!(
            "{}/guilds/{}/roles/{}",
            user.belongs_to.borrow().urls.get_api(),
            guild_id,
            role_id
        );
        let body = to_string::<RoleCreateModifySchema>(&role_create_schema).map_err(|e| {
            ChorusLibError::FormCreationError {
                error: e.to_string(),
            }
        })?;
        let request = Client::new()
            .patch(url)
            .bearer_auth(user.token())
            .body(body);
        deserialize_response::<RoleObject>(request, user, crate::api::limits::LimitType::Guild)
            .await
    }
}
