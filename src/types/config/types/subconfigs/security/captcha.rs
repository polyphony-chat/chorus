// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
