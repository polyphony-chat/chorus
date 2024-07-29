// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::{ChorusUser, Instance},
    ratelimiter::ChorusRequest,
    types::{
        DeleteDisableUserSchema, LimitType, PublicUser, Snowflake, User, UserModifyProfileSchema,
        UserModifySchema, UserProfile, UserProfileMetadata, UserSettings, VerifyUserEmailChangeResponse, VerifyUserEmailChangeSchema,
    },
};

impl ChorusUser {
    /// Gets the local / current user.
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get_current`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-current-user>
    pub async fn get_current_user(&mut self) -> ChorusResult<User> {
        User::get_current(self).await
    }

    /// Gets a non-local user by their id
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user>
    pub async fn get_user(&mut self, id: Snowflake) -> ChorusResult<PublicUser> {
        User::get(self, id).await
    }

    /// Gets a non-local user by their unique username.
    ///
    /// As of 2024/07/28, Spacebar does not yet implement this endpoint.
    ///
    /// Note:
    ///
    /// "Unless the target user is a bot, you must be able to add
    /// the user as a friend to resolve them by username.
    ///
    /// Due to this restriction, you are not able to resolve your own username."
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get_by_username`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-by-username>
    pub async fn get_user_by_username(&mut self, username: &String) -> ChorusResult<PublicUser> {
        User::get_by_username(self, username).await
    }

    /// Gets the user's settings.
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get_settings`].
    pub async fn get_settings(&mut self) -> ChorusResult<UserSettings> {
        User::get_settings(self).await
    }

    /// Modifies the current user's representation. (See [`User`])
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#modify-current-user>
    pub async fn modify(&mut self, modify_schema: UserModifySchema) -> ChorusResult<User> {
        // See <https://docs.discord.sex/resources/user#json-params>, note 1
        let requires_current_password = modify_schema.username.is_some()
            || modify_schema.discriminator.is_some()
            || modify_schema.email.is_some()
            || modify_schema.date_of_birth.is_some()
            || modify_schema.new_password.is_some();

        if requires_current_password && modify_schema.current_password.is_none() {
            return Err(ChorusError::PasswordRequired);
        }

        let request = Client::new()
            .patch(format!(
                "{}/users/@me",
                self.belongs_to.read().unwrap().urls.api
            ))
            .body(to_string(&modify_schema).unwrap())
            .header("Authorization", self.token())
            .header("Content-Type", "application/json");
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.deserialize_response::<User>(self).await
    }

    /// Disables the current user's account.
    ///
    /// Invalidates all active tokens.
    ///
    /// Requires the user's current password (if any)
    ///
    /// # Notes
    /// Requires MFA
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#disable-user>
    pub async fn disable(&mut self, schema: DeleteDisableUserSchema) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/disable",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            .json(&schema);
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.handle_request_as_result(self).await
    }

    /// Deletes the current user from the Instance.
    ///
    /// Requires the user's current password (if any)
    ///
    /// # Notes
    /// Requires MFA
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#delete-user>
    pub async fn delete(&mut self, schema: DeleteDisableUserSchema) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/delete",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            .json(&schema);
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.handle_request_as_result(self).await
    }

    /// Gets a user's profile object by their id.
    ///
    /// This endpoint requires one of the following:
    ///
    /// - The other user is a bot
    /// - The other user shares a mutual guild with the current user
    /// - The other user is a friend of the current user
    /// - The other user is a friend suggestion of the current user
    /// - The other user has an outgoing friend request to the current user
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get_profile`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-profile>
    pub async fn get_user_profile(&mut self, id: Snowflake) -> ChorusResult<UserProfile> {
        User::get_profile(self, id).await
    }

    /// Modifies the current user's profile.
    ///
    /// Returns the updated [UserProfileMetadata].
    ///
    /// # Notes
    /// This function is a wrapper around [`User::modify_profile`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-profile>
    pub async fn modify_profile(
        &mut self,
        schema: UserModifyProfileSchema,
    ) -> ChorusResult<UserProfileMetadata> {
        User::modify_profile(self, schema).await
    }

    /// Initiates the email change process.
    ///
    /// Sends a verification code to the current user's email.
	 ///
	 /// Should be followed up with [Self::verify_email_change]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-email>
    pub async fn initiate_email_change(&mut self) -> ChorusResult<()> {
        let request = Client::new()
            .put(format!(
                "{}/users/@me/email",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.handle_request_as_result(self).await
    }

	 /// Verifies a code sent to change the current user's email.
    ///
	 /// Should be the follow-up to [Self::initiate_email_change]
	 ///
	 /// This endpoint returns a token which can be used with [Self::modify]
	 /// to set a new email address (email_token).
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-email>
    pub async fn verify_email_change(&mut self, schema: VerifyUserEmailChangeSchema) -> ChorusResult<VerifyUserEmailChangeResponse> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/email/verify-code",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
				.json(&schema);
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request.deserialize_response::<VerifyUserEmailChangeResponse>(self).await
    }
}

impl User {
    /// Gets the local / current user.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-current-user>
    pub async fn get_current(user: &mut ChorusUser) -> ChorusResult<User> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let url = format!("{}/users/@me", url_api);
        let request = reqwest::Client::new()
            .get(url)
            .header("Authorization", user.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        chorus_request.deserialize_response::<User>(user).await
    }

    /// Gets a non-local user by their id
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/user#get-user>
    pub async fn get(user: &mut ChorusUser, id: Snowflake) -> ChorusResult<PublicUser> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let url = format!("{}/users/{}", url_api, id);
        let request = reqwest::Client::new()
            .get(url)
            .header("Authorization", user.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<PublicUser>(user)
            .await
    }

    /// Gets a user by their unique username.
    ///
    /// As of 2024/07/28, Spacebar does not yet implement this endpoint.
    ///
    /// Note:
    ///
    /// "Unless the target user is a bot, you must be able to add
    /// the user as a friend to resolve them by username.
    ///
    /// Due to this restriction, you are not able to resolve your own username."
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-by-username>
    pub async fn get_by_username(
        user: &mut ChorusUser,
        username: &String,
    ) -> ChorusResult<PublicUser> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let url = format!("{}/users/username/{username}", url_api);
        let request = reqwest::Client::new()
            .get(url)
            .header("Authorization", user.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<PublicUser>(user)
            .await
    }

    /// Gets the current user's settings.
    ///
    /// # Reference
    /// See <https://luna.gitlab.io/discord-unofficial-docs/docs/user_settings.html#get-usersmesettings>
    pub async fn get_settings(user: &mut ChorusUser) -> ChorusResult<UserSettings> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let request: reqwest::RequestBuilder = Client::new()
            .get(format!("{}/users/@me/settings", url_api))
            .header("Authorization", user.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<UserSettings>(user)
            .await
    }

    /// Gets a user's profile object by their id.
    ///
    /// This endpoint requires one of the following:
    ///
    /// - The other user is a bot
    /// - The other user shares a mutual guild with the current user
    /// - The other user is a friend of the current user
    /// - The other user is a friend suggestion of the current user
    /// - The other user has an outgoing friend request to the current user
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-profile>
    // TODO: Implement query string parameters for this endpoint
    pub async fn get_profile(user: &mut ChorusUser, id: Snowflake) -> ChorusResult<UserProfile> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let request: reqwest::RequestBuilder = Client::new()
            .get(format!("{}/users/{}/profile", url_api, id))
            .header("Authorization", user.token());
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<UserProfile>(user)
            .await
    }

    /// Modifies the current user's profile.
    ///
    /// Returns the updated [UserProfileMetadata].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-profile>
    pub async fn modify_profile(
        user: &mut ChorusUser,
        schema: UserModifyProfileSchema,
    ) -> ChorusResult<UserProfileMetadata> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let request: reqwest::RequestBuilder = Client::new()
            .patch(format!("{}/users/@me/profile", url_api))
            .header("Authorization", user.token())
            .json(&schema);
        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<UserProfileMetadata>(user)
            .await
    }
}
