// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Clone, Debug)]
/// Represents the result of the `$rooturl/.well-known/spacebar` endpoint.
///
/// See <https://docs.spacebar.chat/setup/server/wellknown/> for more information.
pub struct WellKnownResponse {
    pub api: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Represents the result of the `$api/policies/instance/domains` endpoint.
pub struct Domains {
    pub cdn: String,
    pub gateway: String,
    pub api_endpoint: String,
    pub default_api_version: String,
}

impl std::fmt::Display for Domains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tCDN URL: {},\n\tGateway URL: {},\n\tAPI Endpoint: {},\n\tDefault API Version: {}\n}}",
            self.cdn, self.gateway, self.api_endpoint, self.default_api_version
        )
    }
}
