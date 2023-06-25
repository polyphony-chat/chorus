use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct RegisterSchema {
    pub username: String,
    pub password: Option<String>,
    pub consent: bool,
    pub email: Option<String>,
    pub fingerprint: Option<String>,
    pub invite: Option<String>,
    pub date_of_birth: Option<String>,
    pub gift_code_sku_id: Option<String>,
    pub captcha_key: Option<String>,
    pub promotional_email_opt_in: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct LoginSchema {
    /// For Discord, usernames must be between 2 and 32 characters,
    /// but other servers may have different limits.
    pub login: String,
    /// For Discord, must be between 1 and 72 characters,
    /// but other servers may have different limits.
    pub password: String,
    pub undelete: Option<bool>,
    pub captcha_key: Option<String>,
    pub login_source: Option<String>,
    pub gift_code_sku_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TotpSchema {
    code: String,
    ticket: String,
    gift_code_sku_id: Option<String>,
    login_source: Option<String>,
}
