// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde_json::from_str;

use crate::errors::{ChorusError, ChorusResult};
use crate::instance::Instance;
use crate::types::GeneralConfiguration;

impl Instance {
    /// Gets the instance policies schema.
    ///
    /// # Notes
    /// This is a Spacebar only endpoint.
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/routes/#get-/policies/instance/>
    pub async fn general_configuration_schema(&self) -> ChorusResult<GeneralConfiguration> {
        let endpoint_url = self.urls.api.clone() + "/policies/instance/";
        let request = match self.client.get(&endpoint_url).send().await {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusError::RequestFailed {
                    url: endpoint_url,
                    error: e.to_string(),
                });
            }
        };

        if !request.status().as_str().starts_with('2') {
            return Err(ChorusError::ReceivedErrorCode {
                error_code: request.status().as_u16(),
                error: request.text().await.unwrap(),
            });
        }

        let response_text = match request.text().await {
            Ok(string) => string,
            Err(e) => {
                return Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to process the HTTP response into a String: {}",
                        e
                    ),
                });
            }
        };

        match from_str::<GeneralConfiguration>(&response_text) {
            Ok(object) => Ok(object),
            Err(e) => {
                Err(ChorusError::InvalidResponse {
                    error: format!(
                        "Error while trying to deserialize the JSON response into requested type T: {}. JSON Response: {}",
                        e, response_text
                    ),
                })
            }
        }
    }
}
