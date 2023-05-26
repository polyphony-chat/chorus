use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptchaService {
    Recaptcha,
    HCaptcha,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptchaConfiguration {
    pub enabled: bool,
    pub service: CaptchaService,
    pub sitekey: Option<String>,
    pub secret: Option<String>,
}

impl Default for CaptchaConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            service: CaptchaService::HCaptcha,
            sitekey: None,
            secret: None,
        }
    }
}
