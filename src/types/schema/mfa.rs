use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{errors::ChorusError, types::Snowflake};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct MfaRequiredSchema {
    pub message: String,
    pub code: i32,
    pub mfa: MfaVerificationSchema,
}

impl Display for MfaRequiredSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MfaRequired")
            .field("message", &self.message)
            .field("code", &self.code)
            .field("mfa", &self.mfa)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct MfaVerificationSchema {
    pub ticket: String,
    pub methods: Vec<MfaMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct MfaMethod {
    #[serde(rename = "type")]
    pub kind: MfaAuthenticationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes_allowed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
/// Types of [MfaAuthenticator]s.
///
/// Not to be confused with [MfaAuthenticationType], which covers other cases of authentication as well. (Such as backup codes or a password)
pub enum MfaAuthenticatorType {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct MfaVerifySchema {
    pub ticket: String,
    pub mfa_type: MfaAuthenticationType,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaTokenSchema {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMfaSmsSchema {
    pub ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub secret: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Internal return schema for the Enable TOTP MFA route
///
/// Similar to [EanbleTOTPMFAReturn], except it also includes a token field
/// that we don't expose to users
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#enable-totp-mfa>
pub(crate) struct EnableTotpMfaResponse {
    pub(crate) token: String,
    pub backup_codes: Vec<MfaBackupCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response type for the Enable TOTP MFA route
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#enable-totp-mfa>
pub struct EnableTotpMfaReturn {
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
