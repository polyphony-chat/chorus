// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use crate::types::{config::types::subconfigs::register::{
    DateOfBirthConfiguration, PasswordConfiguration, RegistrationEmailConfiguration,
}, Rights};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterConfiguration {
    pub email: RegistrationEmailConfiguration,
    pub date_of_birth: DateOfBirthConfiguration,
    pub password: PasswordConfiguration,
    pub disabled: bool,
    pub require_captcha: bool,
    pub require_invite: bool,
    pub guests_require_invite: bool,
    pub allow_new_registration: bool,
    pub allow_multiple_accounts: bool,
    pub block_proxies: bool,
    pub incrementing_discriminators: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub default_rights: Rights,
}

impl Default for RegisterConfiguration {
    fn default() -> Self {
        Self {
            email: RegistrationEmailConfiguration::default(),
            date_of_birth: DateOfBirthConfiguration::default(),
            password: PasswordConfiguration::default(),
            disabled: false,
            require_captcha: true,
            require_invite: false,
            guests_require_invite: true,
            allow_new_registration: true,
            allow_multiple_accounts: true,
            block_proxies: true,
            incrementing_discriminators: false,
            default_rights: Rights::from_bits(648540060672).expect("failed to parse default_rights"),
        }
    }
}
