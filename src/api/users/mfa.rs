// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;

use crate::{
    errors::ChorusResult,
    instance::{ChorusUser, Token},
    ratelimiter::ChorusRequest,
    types::{
        BeginWebAuthnAuthenticatorCreationReturn, EnableTotpMfaResponse, EnableTotpMfaSchema,
        FinishWebAuthnAuthenticatorCreationReturn, FinishWebAuthnAuthenticatorCreationSchema,
        GetBackupCodesSchema, LimitType, MfaAuthenticator, MfaBackupCode,
        ModifyWebAuthnAuthenticatorSchema, SendBackupCodesChallengeReturn,
        SendBackupCodesChallengeSchema, SmsMfaRouteSchema, Snowflake,
    },
};

impl ChorusUser {
    /// Enables TOTP based multi-factor authentication for the current user.
    ///
    /// # Notes
    /// Fires a [`UserUpdate`](crate::types::UserUpdate) gateway event.
    ///
    /// Updates the authorization token for the current session.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#enable-totp-mfa>
    pub async fn enable_totp_mfa(
        &mut self,
        schema: EnableTotpMfaSchema,
    ) -> ChorusResult<EnableTotpMfaResponse> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/totp/enable",
                self.belongs_to.read().unwrap().urls.api
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        let response: EnableTotpMfaResponse = chorus_request.send_and_deserialize_response(self).await?;

        self.token = response.token.clone();

        Ok(response)
    }

    /// Disables TOTP based multi-factor authentication for the current user.
    ///
    /// Updates the authorization token for the current session and returns the new token.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// MFA cannot be disabled for administrators of guilds with published creator monetization listings.
    ///
    /// Fires a [`UserUpdate`](crate::types::UserUpdate) gateway event.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#disable-totp-mfa>
    pub async fn disable_totp_mfa(&mut self) -> ChorusResult<Token> {
        let request = Client::new().post(format!(
            "{}/users/@me/mfa/totp/disable",
            self.belongs_to.read().unwrap().urls.api
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        let response: Token = chorus_request.send_and_deserialize_response(self).await?;

        self.token = response.token.clone();

        Ok(response)
    }

    /// Enables SMS based multi-factor authentication for the current user.
    ///
    /// Requires that TOTP based MFA is already enabled and the user has a verified phone number.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// Fires a [`UserUpdate`](crate::types::UserUpdate) gateway event.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#enable-sms-mfa>
    pub async fn enable_sms_mfa(&mut self, schema: SmsMfaRouteSchema) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/sms/enable",
                self.belongs_to.read().unwrap().urls.api
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        chorus_request.send_and_handle_as_result(self).await
    }

    /// Disables SMS based multi-factor authentication for the current user.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// Fires a [`UserUpdate`](crate::types::UserUpdate) gateway event.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#disable-sms-mfa>
    pub async fn disable_sms_mfa(&mut self, schema: SmsMfaRouteSchema) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/sms/disable",
                self.belongs_to.read().unwrap().urls.api
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        chorus_request.send_and_handle_as_result(self).await
    }

    /// Fetches a list of [WebAuthn](crate::types::MfaAuthenticatorType::WebAuthn)
    /// [MfaAuthenticator]s for the current user.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#get-webauthn-authenticators>
    pub async fn get_webauthn_authenticators(&mut self) -> ChorusResult<Vec<MfaAuthenticator>> {
        let request = Client::new().get(format!(
            "{}/users/@me/mfa/webauthn/credentials",
            self.belongs_to.read().unwrap().urls.api
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Begins creation of a [WebAuthn](crate::types::MfaAuthenticatorType::WebAuthn)
    /// [MfaAuthenticator] for the current user.
    ///
    /// Returns [BeginWebAuthnAuthenticatorCreationReturn], which includes the MFA ticket
    /// and a stringified JSON object of the public key credential challenge.
    ///
    /// Once you have obtained the credential from the user, you should call
    /// [ChorusUser::finish_webauthn_authenticator_creation]
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#create-webauthn-authenticator>
    ///
    /// Note: for an easier to use API, we've split this one route into two methods
    pub async fn begin_webauthn_authenticator_creation(
        &mut self,
    ) -> ChorusResult<BeginWebAuthnAuthenticatorCreationReturn> {
        let request = Client::new().post(format!(
            "{}/users/@me/mfa/webauthn/credentials",
            self.belongs_to.read().unwrap().urls.api
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Finishes creation of a [WebAuthn](crate::types::MfaAuthenticatorType::WebAuthn)
    /// [MfaAuthenticator] for the current user.
    ///
    /// Returns [FinishWebAuthnAuthenticatorCreationReturn], which includes the created
    /// authenticator and a list of backup codes.
    ///
    /// To create a Webauthn authenticator from start to finish, call
    /// [ChorusUser::begin_webauthn_authenticator_creation] first.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// Fires [AuthenticatorCreate](crate::types::AuthenticatorCreate) and
    /// [UserUpdate](crate::types::UserUpdate) events.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#create-webauthn-authenticator>
    ///
    /// Note: for an easier to use API, we've split this one route into two methods
    pub async fn finish_webauthn_authenticator_creation(
        &mut self,
        schema: FinishWebAuthnAuthenticatorCreationSchema,
    ) -> ChorusResult<FinishWebAuthnAuthenticatorCreationReturn> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/webauthn/credentials",
                self.belongs_to.read().unwrap().urls.api
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Modifies a [WebAuthn](crate::types::MfaAuthenticatorType::WebAuthn)
    /// [MfaAuthenticator] (currently just renames) for the current user.
    ///
    /// Returns the updated authenticator.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// Fires an [AuthenticatorUpdate](crate::types::AuthenticatorUpdate) event.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#modify-webauthn-authenticator>
    pub async fn modify_webauthn_authenticator(
        &mut self,
        authenticator_id: Snowflake,
        schema: ModifyWebAuthnAuthenticatorSchema,
    ) -> ChorusResult<MfaAuthenticator> {
        let request = Client::new()
            .patch(format!(
                "{}/users/@me/mfa/webauthn/credentials/{}",
                self.belongs_to.read().unwrap().urls.api,
                authenticator_id
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Deletes a [WebAuthn](crate::types::MfaAuthenticatorType::WebAuthn)
    /// [MfaAuthenticator] for the current user.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// Fires [AuthenticatorDelete](crate::types::AuthenticatorDelete) and
    /// [UserUpdate](crate::types::UserUpdate) events.
    ///
    /// If this is the last remaining authenticator, this disables MFA for the current user.
    ///
    /// MFA cannot be disabled for administrators of guilds with published creator monetization listings.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#delete-webauthn-authenticator>
    pub async fn delete_webauthn_authenticator(
        &mut self,
        authenticator_id: Snowflake,
    ) -> ChorusResult<()> {
        let request = Client::new().delete(format!(
            "{}/users/@me/mfa/webauthn/credentials/{}",
            self.belongs_to.read().unwrap().urls.api,
            authenticator_id
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token)
        .with_headers_for(self);

        chorus_request.send_and_handle_as_result(self).await
    }

    /// Sends an email to the current user with a verification code
    /// that allows them to view or regenerate their backup codes.
    ///
    /// For the request to actually view the backup codes, see [ChorusUser::get_backup_codes].
    ///
    /// # Notes
    /// The two returned nonces can only be used once and expire after 30 minutes.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#send-backup-codes-challenge>
    pub async fn send_backup_codes_challenge(
        &mut self,
        schema: SendBackupCodesChallengeSchema,
    ) -> ChorusResult<SendBackupCodesChallengeReturn> {
        let request = Client::new()
            .post(format!(
                "{}/auth/verify/view-backup-codes-challenge",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Fetches the user's [MfaBackupCode]s.
    ///
    /// Before using this endpoint, you must use [ChorusUser::send_backup_codes_challenge] and
    /// obtain a key from the user's email.
    ///
    /// # Notes
    /// The nonces in the schema are returned by the [ChorusUser::send_backup_codes_challenge]
    /// endpoint.
    ///
    /// If regenerate is set to true, the nonce in the schema must be the regenerate_nonce.
    /// Otherwise it should be the view_nonce.
    ///
    /// Each nonce can only be used once and expires after 30 minutes.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#get-backup-codes>
    pub async fn get_backup_codes(
        &mut self,
        schema: GetBackupCodesSchema,
    ) -> ChorusResult<Vec<MfaBackupCode>> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/codes-verification",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }
}
