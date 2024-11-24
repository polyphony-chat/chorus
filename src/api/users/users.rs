// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::{ChorusUser, Instance},
    ratelimiter::ChorusRequest,
    types::{
        AuthorizeConnectionSchema, BurstCreditsInfo, ConnectionType, CreateUserHarvestSchema,
        DeleteDisableUserSchema, GetPomeloEligibilityReturn, GetPomeloSuggestionsReturn,
        GetRecentMentionsSchema, GetUserProfileSchema, GuildAffinities, Harvest,
        HarvestBackendType, LimitType, ModifyUserNoteSchema, PremiumUsage, PublicUser, Snowflake,
        User, UserAffinities, UserModifyProfileSchema, UserModifySchema, UserNote, UserProfile,
        UserProfileMetadata, UserSettings, VerifyUserEmailChangeResponse,
        VerifyUserEmailChangeSchema,
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
    /// If fetching with a pomelo username, discriminator should be set to None.
    ///
    /// This route also permits fetching users with their old pre-pomelo username#discriminator
    /// combo.
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
    pub async fn get_user_by_username(
        &mut self,
        username: &String,
        discriminator: Option<&String>,
    ) -> ChorusResult<PublicUser> {
        User::get_by_username(self, username, discriminator).await
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
    /// # Notes
    /// This route requires MFA.
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
        }
        .with_maybe_mfa(&self.mfa_token);

        chorus_request.deserialize_response::<User>(self).await
    }

    /// Disables the current user's account.
    ///
    /// Invalidates all active tokens.
    ///
    /// Requires the user's current password (if any)
    ///
    /// # Notes
    /// This route requires MFA.
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
        }
        .with_maybe_mfa(&self.mfa_token);

        chorus_request.handle_request_as_result(self).await
    }

    /// Deletes the current user from the Instance.
    ///
    /// Requires the user's current password (if any)
    ///
    /// # Notes
    /// This route requires MFA.
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
        }
        .with_maybe_mfa(&self.mfa_token);

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
    pub async fn get_user_profile(
        &mut self,
        id: Snowflake,
        query_parameters: GetUserProfileSchema,
    ) -> ChorusResult<UserProfile> {
        User::get_profile(self, id, query_parameters).await
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
    /// This endpoint returns a token which can be used with [Self::modify]
    /// to set a new email address (email_token).
    ///
    /// # Notes
    /// Should be the follow-up to [Self::initiate_email_change]
    ///
    /// As of 2024/08/08, Spacebar does not yet implement this endpoint.
    // FIXME: Does this mean PUT users/@me/email is different?
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-email>
    pub async fn verify_email_change(
        &mut self,
        schema: VerifyUserEmailChangeSchema,
    ) -> ChorusResult<VerifyUserEmailChangeResponse> {
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
        chorus_request
            .deserialize_response::<VerifyUserEmailChangeResponse>(self)
            .await
    }

    /// Returns a suggested unique username based on the current user's username.
    ///
    /// # Notes:
    /// "This endpoint is used during the pomelo migration flow.
    ///
    /// The user must be in the rollout to use this endpoint."
    ///
    /// If a user has already migrated, this endpoint will likely return a 401 Unauthorized
    /// ([ChorusError::NoPermission])
    ///
    /// As of 2024/08/08, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-pomelo-suggestions>
    pub async fn get_pomelo_suggestions(&mut self) -> ChorusResult<String> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/pomelo-suggestions",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request
            .deserialize_response::<GetPomeloSuggestionsReturn>(self)
            .await
            .map(|returned| returned.username)
    }

    /// Checks whether a unique username is available.
    ///
    /// Returns whether the username is not taken yet.
    ///
    /// # Notes
    /// As of 2024/08/08, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-pomelo-eligibility>
    pub async fn get_pomelo_eligibility(&mut self, username: &String) -> ChorusResult<bool> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/pomelo-attempt",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            // FIXME: should we create a type for this?
            .body(format!(r#"{{ "username": {:?} }}"#, username))
            .header("Content-Type", "application/json");

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };
        chorus_request
            .deserialize_response::<GetPomeloEligibilityReturn>(self)
            .await
            .map(|returned| !returned.taken)
    }

    /// Migrates the user from the username#discriminator to the unique username system.
    ///
    /// Fires a [UserUpdate](crate::types::UserUpdate) gateway event.
    ///
    /// Updates [Self::object] to an updated representation returned by the server.
    // FIXME: Is this appropriate behaviour?
    ///
    /// # Notes
    /// "This endpoint is used during the pomelo migration flow.
    ///
    /// The user must be in the rollout to use this endpoint."
    ///
    /// If a user has already migrated, this endpoint will likely return a 401 Unauthorized
    /// ([ChorusError::NoPermission])
    //
    /// As of 2024/08/08, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#create-pomelo-migration>
    pub async fn create_pomelo_migration(&mut self, username: &String) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/pomelo",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            // FIXME: should we create a type for this?
            .body(format!(r#"{{ "username": {:?} }}"#, username))
            .header("Content-Type", "application/json");

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        let result = chorus_request.deserialize_response::<User>(self).await;

        // FIXME: Does UserUpdate do this automatically? or would a user need to manually observe ChorusUser::object
        if let Ok(new_object) = result {
            *self.object.write().unwrap() = new_object;
            return ChorusResult::Ok(());
        }

        ChorusResult::Err(result.err().unwrap())
    }

    /// Fetches a list of [Message](crate::types::Message)s that the current user has been
    /// mentioned in during the last 7 days.
    ///
    /// # Notes
    /// As of 2024/08/09, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-recent-mentions>
    pub async fn get_recent_mentions(
        &mut self,
        query_parameters: GetRecentMentionsSchema,
    ) -> ChorusResult<Vec<crate::types::Message>> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/mentions",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            .query(&query_parameters);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request
            .deserialize_response::<Vec<crate::types::Message>>(self)
            .await
    }

    /// Acknowledges a message the current user has been mentioned in.
    ///
    /// Fires a `RecentMentionDelete` gateway event. (Note: yet to be implemented in chorus, see [#545](https://github.com/polyphony-chat/chorus/issues/545))
    ///
    /// # Notes
    /// As of 2024/08/09, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#delete-recent-mention>
    pub async fn delete_recent_mention(&mut self, message_id: Snowflake) -> ChorusResult<()> {
        let request = Client::new()
            .delete(format!(
                "{}/users/@me/mentions/{}",
                self.belongs_to.read().unwrap().urls.api,
                message_id
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.handle_request_as_result(self).await
    }

    /// If it exists, returns the most recent [Harvest] (personal data harvest request).
    ///
    /// To create a new [Harvest], see [Self::create_harvest].
    ///
    /// # Notes
    /// As of 2024/08/09, Spacebar does not yet implement this endpoint. (Or data harvesting)
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-harvest>
    pub async fn get_harvest(&mut self) -> ChorusResult<Option<Harvest>> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/harvest",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        // Manual handling, because a 204 with no harvest is a success state
        // TODO: Maybe make this a method on ChorusRequest if we need it a lot
        let response = chorus_request.send_request(self).await?;
        log::trace!("Got response: {:?}", response);

        if response.status() == http::StatusCode::NO_CONTENT {
            return Ok(None);
        }

        let response_text = match response.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                });
            }
        };

        let object = match serde_json::from_str::<Harvest>(&response_text) {
            Ok(object) => object,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to deserialize the JSON response into requested type T: {}. JSON Response: {}",
                        e, response_text
                    ),
                })
            }
        };
        Ok(Some(object))
    }

    /// Creates a personal data harvest request ([Harvest]) for the current user.
    ///
    /// # Notes
    /// To fetch the latest existing harvest, see [Self::get_harvest].
    ///
    /// Invalid options in the backends array are ignored.
    ///
    /// If the array is empty (after ignoring), it requests all [HarvestBackendType]s.
    ///
    /// As of 2024/08/09, Spacebar does not yet implement this endpoint. (Or data harvesting)
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#create-user-harvest>
    pub async fn create_harvest(
        &mut self,
        backends: Vec<HarvestBackendType>,
    ) -> ChorusResult<Harvest> {
        let schema = if backends.is_empty() {
            CreateUserHarvestSchema { backends: None }
        } else {
            CreateUserHarvestSchema {
                backends: Some(backends),
            }
        };

        let request = Client::new()
            .post(format!(
                "{}/users/@me/harvest",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token())
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Returns a mapping of user IDs ([Snowflake]s) to notes ([String]s) for the current user.
    ///
    /// # Notes
    /// As of 2024/08/21, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-notes>
    pub async fn get_user_notes(&mut self) -> ChorusResult<HashMap<Snowflake, String>> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/notes",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Fetches the note ([UserNote]) for the given user.
    ///
    /// If the current user has no note for the target, this endpoint
    /// returns `Err(NotFound { error: "{\"message\": \"Unknown User\", \"code\": 10013}" })`
    ///
    /// # Notes
    /// This function is a wrapper around [`User::get_note`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-note>
    pub async fn get_user_note(&mut self, target_user_id: Snowflake) -> ChorusResult<UserNote> {
        User::get_note(self, target_user_id).await
    }

    /// Sets the note for the given user.
    ///
    /// Fires a `UserNoteUpdate` gateway event. (Note: yet to be implemented in chorus, see [#546](https://github.com/polyphony-chat/chorus/issues/546))
    ///
    /// # Notes
    /// This function is a wrapper around [`User::set_note`].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-note>
    pub async fn set_user_note(
        &mut self,
        target_user_id: Snowflake,
        note: Option<String>,
    ) -> ChorusResult<()> {
        User::set_note(self, target_user_id, note).await
    }

    /// Fetches the current user's affinity scores for other users.
    ///
    /// (Affinity scores are a measure of how likely a user is to be friends with another user.)
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-affinities>
    pub async fn get_user_affinities(&mut self) -> ChorusResult<UserAffinities> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/affinities/users",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Fetches the current user's affinity scores for their joined guilds.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-guild-affinities>
    pub async fn get_guild_affinities(&mut self) -> ChorusResult<GuildAffinities> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/affinities/guilds",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Fetches the current user's usage of various premium perks ([PremiumUsage] object).
    ///
    /// The local user must have premium (nitro), otherwise the request will fail
    /// with a 404 NotFound error and the message {"message": "Premium usage not available", "code": 10084}.
    ///
    /// # Notes
    /// As of 2024/08/16, Spacebar does not yet implement this endpoint.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-premium-usage>
    pub async fn get_premium_usage(&mut self) -> ChorusResult<PremiumUsage> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/premium-usage",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Fetches info about the current user's burst credits
    /// (how many are remaining, when they will replenish).
    ///
    /// Burst credits are used to create burst reactions.
    ///
    /// # Notes
    /// As of 2024/08/18, Spacebar does not yet implement this endpoint.
    pub async fn get_burst_credits(&mut self) -> ChorusResult<BurstCreditsInfo> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/burst-credits",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
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
    /// If fetching with a pomelo username, discriminator should be set to None.
    ///
    /// This route also permits fetching users with their old pre-pomelo username#discriminator
    /// combo.
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
        discriminator: Option<&String>,
    ) -> ChorusResult<PublicUser> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let url = format!("{}/users/username/{username}", url_api);
        let mut request = reqwest::Client::new()
            .get(url)
            .header("Authorization", user.token());

        if let Some(some_discriminator) = discriminator {
            request = request.query(&[("discriminator", some_discriminator)]);
        }

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
    pub async fn get_profile(
        user: &mut ChorusUser,
        id: Snowflake,
        query_parameters: GetUserProfileSchema,
    ) -> ChorusResult<UserProfile> {
        let url_api = user.belongs_to.read().unwrap().urls.api.clone();
        let request: reqwest::RequestBuilder = Client::new()
            .get(format!("{}/users/{}/profile", url_api, id))
            .header("Authorization", user.token())
            .query(&query_parameters);

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

    /// Fetches the note ([UserNote]) for the given user.
    ///
    /// If the current user has no note for the target, this endpoint
    /// returns `Err(NotFound { error: "{\"message\": \"Unknown User\", \"code\": 10013}" })`
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-note>
    pub async fn get_note(
        user: &mut ChorusUser,
        target_user_id: Snowflake,
    ) -> ChorusResult<UserNote> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/notes/{}",
                user.belongs_to.read().unwrap().urls.api,
                target_user_id
            ))
            .header("Authorization", user.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(user).await
    }

    /// Sets the note for the given user.
    ///
    /// Fires a `UserNoteUpdate` gateway event. (Note: yet to be implemented in chorus, see [#546](https://github.com/polyphony-chat/chorus/issues/546))
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-note>
    pub async fn set_note(
        user: &mut ChorusUser,
        target_user_id: Snowflake,
        note: Option<String>,
    ) -> ChorusResult<()> {
        let schema = ModifyUserNoteSchema { note };

        let request = Client::new()
            .put(format!(
                "{}/users/@me/notes/{}",
                user.belongs_to.read().unwrap().urls.api,
                target_user_id
            ))
            .header("Authorization", user.token())
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.handle_request_as_result(user).await
    }
}
