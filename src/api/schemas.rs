pub mod schemas {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct RegisterSchema {
        username: String,
        password: Option<String>,
        consent: bool,
        email: Option<String>,
        fingerprint: Option<String>,
        invite: Option<String>,
        date_of_birth: Option<String>,
        gift_code_sku_id: Option<String>,
        captcha_key: Option<String>,
        promotional_email_opt_in: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct LoginSchema {
        login: String,
        password: String,
        undelete: Option<bool>,
        captcha_key: Option<String>,
        login_source: Option<String>,
        gift_code_sku_id: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct TotpSchema {
        code: String,
        ticket: String,
        gift_code_sku_id: Option<String>,
        login_source: Option<String>,
    }
}
