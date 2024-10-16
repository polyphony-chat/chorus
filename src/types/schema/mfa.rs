use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    types::Snowflake,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
/// Error received when mfa is required
pub struct MfaRequiredSchema {
    pub message: String,
    pub code: i32,
    #[serde(rename = "mfa")]
    pub mfa_challenge: MfaChallenge,
}

impl Display for MfaRequiredSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MfaRequired")
            .field("message", &self.message)
            .field("code", &self.code)
            .field("mfa", &self.mfa_challenge)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
/// A challenge to verify the local user's identity with mfa.
///
/// (Normally returned in [MfaRequiredSchema] as [ChorusError::MfaRequired])
///
/// To complete the challenge, see [ChorusUser::complete_mfa_challenge].
pub struct MfaChallenge {
    /// A unique ticket which identifies this challenge
    pub ticket: String,
    /// The ways we can verify the user's identity
    pub methods: Vec<MfaMethod>,
}

impl MfaChallenge {
    /// Attempts to complete the [MfaChallenge] with authentication data from the user.
    ///
    /// If successful, the MFA verification JWT returned is set on the provided [ChorusUser].
    ///
    /// The JWT token expires after 5 minutes.
    ///
    /// # Arguments
    /// `authentication_type` is the way the user has chosen to authenticate.
    ///
    /// It must be the type of one of the provided `methods` in the challenge.
    ///
    /// `data` is specific to the `authentication_type`.
    ///
    /// For example, a totp authenticator uses a 6 digit code as the `data`.
    ///
    /// # Notes
    /// Alias of [ChorusUser::complete_mfa_challenge]
    pub async fn complete(
        self,
        user: &mut ChorusUser,
        authentication_type: MfaAuthenticationType,
        data: String,
    ) -> ChorusResult<()> {
        let schema =
            MfaVerifySchema::from_challenge_and_verification_data(self, authentication_type, data);

        user.complete_mfa_challenge(schema).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
/// A way we can verify the user's identity, found in [MfaChallenge]
pub struct MfaMethod {
    /// The type of authentication we can perform
    #[serde(rename = "type")]
    pub kind: MfaAuthenticationType,

    /// A challenge string unique to the authentication type, [None] if the type does not need a challenge string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,

    /// Whether or not we can use a backup code for this authentication type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes_allowed: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
/// A multi-factor authentication authenticator.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#authenticator-object>
pub struct MfaAuthenticator {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub authenticator_type: MfaAuthenticatorType,
    pub name: String,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
/// Types of [MfaAuthenticator]s.
///
/// Not to be confused with [MfaAuthenticationType], which covers other cases of authentication as well. (Such as backup codes or a password)
pub enum MfaAuthenticatorType {
    #[default]
    WebAuthn = 1,
    TOTP = 2,
    SMS = 3,
}

impl TryFrom<u8> for MfaAuthenticatorType {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WebAuthn),
            2 => Ok(Self::TOTP),
            3 => Ok(Self::SMS),
            _ => Err(ChorusError::InvalidArguments {
                error: "Value is not a valid MfaAuthenticatorType".to_string(),
            }),
        }
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for MfaAuthenticatorType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for MfaAuthenticatorType {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::from(*self as u8);
        sqlx_pg_uint.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for MfaAuthenticatorType {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::decode(value)?;
        MfaAuthenticatorType::try_from(sqlx_pg_uint.to_uint()).map_err(|e| e.into())
    }
}

impl MfaAuthenticatorType {
    /// Converts self into [MfaAuthenticationType]
    pub fn into_authentication_type(self) -> MfaAuthenticationType {
        match self {
            Self::WebAuthn => MfaAuthenticationType::WebAuthn,
            Self::TOTP => MfaAuthenticationType::TOTP,
            Self::SMS => MfaAuthenticationType::SMS,
        }
    }
}

impl From<MfaAuthenticatorType> for MfaAuthenticationType {
    fn from(value: MfaAuthenticatorType) -> Self {
        value.into_authentication_type()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
/// Ways to perform multi factor authentication.
pub enum MfaAuthenticationType {
    WebAuthn,
    TOTP,
    SMS,
    Backup,
    Password,
}

impl Display for MfaAuthenticationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MfaAuthenticationType::TOTP => "totp",
                MfaAuthenticationType::SMS => "sms",
                MfaAuthenticationType::Backup => "backup",
                MfaAuthenticationType::WebAuthn => "webauthn",
                MfaAuthenticationType::Password => "password",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
/// An mfa backup code.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#backup-code-object>
pub struct MfaBackupCode {
    pub user_id: Snowflake,
    pub code: String,
    /// Whether or not the backup code has been used
    pub consumed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// A schema used for the [ChorusUser::complete_mfa_challenge] route.
pub struct MfaVerifySchema {
    /// Usually obtained from [MfaChallenge]
    pub ticket: String,
    /// The way the user has chosen to authenticate
    ///
    /// Must be one of the available methods in from the [MfaChallenge]
    pub mfa_type: MfaAuthenticationType,
    /// Data unique to the authentication type (ex. a 6 digit totp code for totp, a password)
    pub data: String,
}

impl MfaVerifySchema {
    /// Creates the verify schema from an [MfaChallenge] and data needed to complete it.
    ///
    /// Shorthand for initializing [Self] with mfa_type, data and ticket = challenge.ticket
    pub fn from_challenge_and_verification_data(
        challenge: MfaChallenge,
        mfa_type: MfaAuthenticationType,
        data: String,
    ) -> Self {
        Self {
            ticket: challenge.ticket,
            mfa_type,
            data,
        }
    }

    /// Uses the verification schema to attempt to complete an [MfaChallenge].
    ///
    /// If successful, the MFA verification JWT returned is set on the provided [ChorusUser].
    ///
    /// The JWT token expires after 5 minutes.
    ///
    /// # Notes
    /// Alias of [ChorusUser::complete_mfa_challenge]
    pub async fn verify_mfa(self, user: &mut ChorusUser) -> ChorusResult<()> {
        user.complete_mfa_challenge(self).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An MFA token generated by the server after completing and mfa challenge ([crate::instance::ChorusUser::complete_mfa_challenge])
pub struct MfaTokenSchema {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Schema for the Send Mfa SMS route ([crate::instance::Instance::send_mfa_sms])
pub struct SendMfaSmsSchema {
    pub ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Return type for the Send Mfa SMS route ([crate::instance::Instance::send_mfa_sms])
pub struct SendMfaSmsResponse {
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Json schema for the Enable TOTP MFA route
///
/// # Notes
/// Secret and code are optional so that clients
/// may first verify the password is correct before
/// letting the user save the secrets.
///
/// If the password is valid, the request will fail with a 60005
/// json error code. However note that JSON error codes are not yet
/// implemented in chorus. (<https://github.com/polyphony-chat/chorus/issues/569>)
/// To implement this kind of check, you would need to manually deserialize into
/// the json error code object.
// TODO: Json error codes
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#enable-totp-mfa>
pub struct EnableTotpMfaSchema {
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response type for the Enable TOTP MFA route
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#enable-totp-mfa>
pub struct EnableTotpMfaResponse {
    pub token: String,
    pub backup_codes: Vec<MfaBackupCode>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema for SMS MFA Enable and Disable routes.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#enable-sms-mfa> and
/// <https://docs.discord.sex/resources/user#disable-sms-mfa>
pub struct SmsMfaRouteSchema {
    /// The user's current password
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A return type for the [ChorusUser::begin_webauthn_authenticator_creation] route (Create WebAuthn Authenticator with no arguments).
///
/// Includes the MFA ticket and a stringified JSON object of the public key credential challenge.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#create-webauthn-authenticator>
pub struct BeginWebAuthnAuthenticatorCreationReturn {
    pub ticket: String,
    /// Stringified JSON public key credential request options challenge
    pub challenge: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema for the [ChorusUser::finish_webauthn_authenticator_creation] route (Create WebAuthn Authenticator).
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#create-webauthn-authenticator>
pub struct FinishWebAuthnAuthenticatorCreationSchema {
    /// Name of the authenticator to create (1 - 32 characters)
    pub name: String,
    /// The MFA ticket returned by the (begin creation)[ChorusUser::begin_webauthn_authenticator_creation] endpoint
    pub ticket: String,
    /// A stringified JSON object of the public key credential response.
    pub credential: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A return type for the [ChorusUser::finish_webauthn_authenticator_creation] route (Create WebAuthn Authenticator).
///
/// Includes the MFA ticket and a stringified JSON object of the public key credential challenge.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#create-webauthn-authenticator>
pub struct FinishWebAuthnAuthenticatorCreationReturn {
    #[serde(flatten)]
    /// The created authenticator object
    pub authenticator: MfaAuthenticator,
    /// A list of MFA backup codes
    pub backup_codes: Vec<MfaBackupCode>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema for the Modify WebAuthn Authenticator route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#modify-webauthn-authenticator>
pub struct ModifyWebAuthnAuthenticatorSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// New name of the authenticator (1 - 32 characters)
    pub name: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema for the Send Backup Codes Challenge route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#send-backup-codes-challenge>
pub struct SendBackupCodesChallengeSchema {
    /// The user's current password
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A return type for the Send Backup Codes Challenge route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#send-backup-codes-challenge>
pub struct SendBackupCodesChallengeReturn {
    /// A one-time verification nonce used to view the backup codes
    ///
    /// Send this in the [ChorusUser::get_backup_codes] endpoint as the nonce if you want to view
    /// the existing codes
    #[serde(rename = "nonce")]
    pub view_nonce: String,
    /// A one-time verification nonce used to regenerate the backup codes
    ///
    /// Send this in the [ChorusUser::get_backup_codes] endpoint as the nonce if you want to
    /// regenerate the backup codes
    pub regenerate_nonce: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema for the Get Backup Codes route.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#get-backup-codes>
pub struct GetBackupCodesSchema {
    /// The one-time verification nonce used to view or regenerate the backup codes.
    ///
    /// Obtained from the [ChorusUser::send_backup_codes_challenge] route.
    pub nonce: String,
    /// The backup verification key received in the email
    pub key: String,
    /// Whether or not to regenerate the backup codes
    ///
    /// If set to true, nonce should be the regenerate_nonce
    /// otherwise it should be the view_nonce
    pub regenerate: bool,
}
