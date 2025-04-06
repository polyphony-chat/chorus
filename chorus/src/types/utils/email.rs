// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use regex::Regex;

use crate::types::errors::{Error, UserError};

pub fn adjust_email(email: &str) -> Result<String, UserError> {
    if email.is_empty() {
        return Err(UserError::InvalidEmail);
    }

    let checked_email =
        email_address::EmailAddress::from_str(email).map_err(|_| UserError::InvalidEmail)?;

    // TODO: check accounts with uncommon email domains
    // TODO: replace .dots and +alternatives -> Gmail Dot Trick https://support.google.com/mail/answer/7436150 and https://generator.email/blog/gmail-generator
    Ok(checked_email.to_string())
}
