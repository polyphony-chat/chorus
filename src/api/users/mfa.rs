use reqwest::Client;

use crate::{
    errors::ChorusResult,
    instance::{ChorusUser, Token},
    ratelimiter::ChorusRequest,
    types::{
        MfaAuthenticator, EnableTotpMfaResponse, EnableTotpMfaReturn, EnableTotpMfaSchema, LimitType,
        SmsMfaRouteSchema,
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
    /// See <https://docs.discord.sex/resources/user#enable-totp-mfa>
    pub async fn enable_totp_mfa(
        &mut self,
        schema: EnableTotpMfaSchema,
    ) -> ChorusResult<EnableTotpMfaReturn> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/totp/enable",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token())
            .json(&schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        let response: EnableTotpMfaResponse = chorus_request.deserialize_response(self).await?;

        self.token = response.token;

        Ok(EnableTotpMfaReturn {
            backup_codes: response.backup_codes,
        })
    }

    /// Disables TOTP based multi-factor authentication for the current user.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// MFA cannot be disabled for administrators of guilds with published creator monetization listings.
    ///
    /// Fires a [`UserUpdate`](crate::types::UserUpdate) gateway event.
    ///
    /// Updates the authorization token for the current session.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#disable-totp-mfa>
    pub async fn disable_totp_mfa(&mut self) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/totp/disable",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_maybe_mfa(&self.mfa_token);

        let response: Token = chorus_request.deserialize_response(self).await?;

        self.token = response.token;

        Ok(())
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
    /// See <https://docs.discord.sex/resources/user#enable-sms-mfa>
    pub async fn enable_sms_mfa(&mut self, schema: SmsMfaRouteSchema) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/sms/enable",
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

    /// Disables SMS based multi-factor authentication for the current user.
    ///
    /// # Notes
    /// Requires MFA.
    ///
    /// Fires a [`UserUpdate`](crate::types::UserUpdate) gateway event.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#disable-sms-mfa>
    pub async fn disable_sms_mfa(&mut self, schema: SmsMfaRouteSchema) -> ChorusResult<()> {
        let request = Client::new()
            .post(format!(
                "{}/users/@me/mfa/sms/disable",
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

    /// Fetches a list of [WebAuthn](crate::types::MfaAuthenticatorType::WebAuthn)
    /// [MfaAuthenticator]s for the current user.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-webauthn-authenticators>
    pub async fn get_webauthn_authenticators(&mut self) -> ChorusResult<Vec<MfaAuthenticator>> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/mfa/webauthn/credentials",
                self.belongs_to.read().unwrap().urls.api
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }
}
