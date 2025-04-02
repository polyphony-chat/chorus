// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use reqwest::RequestBuilder;

use crate::ratelimiter::ChorusRequest;

#[derive(Debug, Clone)]
/// A Token used to bypass mfa for five minutes.
pub struct MfaToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl MfaToken {
    /// Add the MFA bypass token to a reqwest request builder.
    ///
    /// This is used to provide the token in requests that require MFA.
    pub fn add_to_request_builder(&self, request: RequestBuilder) -> RequestBuilder {
        request.header("X-Discord-MFA-Authorization", &self.token)
    }

    /// Add the MFA bypass token to a [ChorusRequest].
    ///
    /// This is used to provide the token in requests that require MFA.
    pub fn add_to_request(&self, request: ChorusRequest) -> ChorusRequest {
        let mut request = request;

        let request_builder = request.request;

        request.request = self.add_to_request_builder(request_builder);
        request
    }

    /// Returns whether or not the token is still valid
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

impl ChorusRequest {
    /// Adds an [MfaToken] to the request.
    ///
    /// Used for requests that need MFA.
    pub fn with_mfa(self, token: &MfaToken) -> ChorusRequest {
        token.add_to_request(self)
    }

    /// Adds an [MfaToken] to the request, if the token is [Some].
    ///
    /// Used for requests that need MFA, when we might or might not have a token already
    pub fn with_maybe_mfa(self, token: &Option<MfaToken>) -> ChorusRequest {
        if let Some(mfa_token) = token {
            return mfa_token.add_to_request(self);
        }

        self
    }
}
