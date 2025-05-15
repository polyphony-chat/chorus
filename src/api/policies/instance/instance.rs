// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde_json::from_str;

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::Instance;
use crate::ratelimiter::ChorusRequest;
use crate::types::{GeneralConfiguration, LimitType};

impl Instance {
    /// Gets the instance policies schema.
    ///
    /// # Notes
    /// This is a Spacebar only endpoint.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#get-/policies/instance/>
    pub async fn general_configuration_schema(&mut self) -> ChorusResult<GeneralConfiguration> {
        let url = self.urls.api.clone() + "/policies/instance/";

        let chorus_request = ChorusRequest {
            request: self.client.get(&url),
            limit_type: LimitType::Global,
        };

        chorus_request
            .send_anonymous_and_deserialize_response(self)
            .await
    }
}
