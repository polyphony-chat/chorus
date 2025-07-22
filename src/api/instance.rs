// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Contains miscellaneous api routes, such as /version and /ping
use reqwest::Client;
use serde_json::from_str;

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::Instance;
use crate::ratelimiter::ChorusRequest;
use crate::types::{GeneralConfiguration, LimitType, PingReturn, VersionReturn};

impl Instance {
    /// Pings the instance, also fetches instance info.
    ///
    /// See: [PingReturn]
    ///
    /// # Notes
    /// This is a Spacebar only endpoint.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#get-/ping/>
    pub async fn ping(&mut self) -> ChorusResult<PingReturn> {
        let url = format!("{}/ping", self.urls.api.clone());

        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Global,
        }
        .with_client_properties(&self.default_client_properties);

        chorus_request
            .send_anonymous_and_deserialize_response(self)
            .await
    }

    /// Fetches the instance's software implementation and version.
    ///
    /// See: [VersionReturn]
    ///
    /// # Notes
    /// This is a Symfonia only endpoint. (For now, we hope that spacebar will adopt it as well)
    pub async fn get_version(&mut self) -> ChorusResult<VersionReturn> {
        let url = format!("{}/version", self.urls.api.clone());

        let chorus_request = ChorusRequest {
            request: Client::new().get(url.clone()),
            limit_type: LimitType::Global,
        }
        .with_client_properties(&self.default_client_properties);

        chorus_request
            .send_anonymous_and_deserialize_response(self)
            .await
    }
}
