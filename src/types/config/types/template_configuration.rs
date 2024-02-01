// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateConfiguration {
    pub enabled: bool,
    pub allow_template_creation: bool,
    pub allow_discord_templates: bool,
    pub allow_raws: bool,
}

impl Default for TemplateConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_template_creation: true,
            allow_discord_templates: true,
            allow_raws: true,
        }
    }
}
