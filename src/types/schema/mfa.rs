use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::Snowflake;

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
    pub kind: AuthenticatorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_codes_allowed: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticatorType {
    TOTP,
    SMS,
    Backup,
    WebAuthn,
    Password,
}

impl Display for AuthenticatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AuthenticatorType::TOTP => "totp",
                AuthenticatorType::SMS => "sms",
                AuthenticatorType::Backup => "backup",
                AuthenticatorType::WebAuthn => "webauthn",
                AuthenticatorType::Password => "password",
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct MfaVerifySchema {
    pub ticket: String,
    pub mfa_type: AuthenticatorType,
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
