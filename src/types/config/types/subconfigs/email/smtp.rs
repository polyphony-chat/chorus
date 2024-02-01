// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SMTPConfiguration {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub secure: bool,
    pub username: Option<String>,
    pub password: Option<String>,
}
