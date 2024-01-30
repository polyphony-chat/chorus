// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::events::{PresenceUpdate, WebSocketEvent};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: GatewayIdentifyConnectionProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i16>,
    //default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<(i32, i32)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceUpdate>,
    // What is the difference between these two?
    // Intents is documented, capabilities is used in users
    // I wonder if these are interchangeable...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<i32>,
}

impl Default for GatewayIdentifyPayload {
    fn default() -> Self {
        Self::common()
    }
}

impl GatewayIdentifyPayload {
    /// Uses the most common, 25% data along with client capabilities
    ///
    /// Basically pretends to be an official client on Windows 10, with Chrome 113.0.0.0
    pub fn common() -> Self {
        Self {
            token: "".to_string(),
            properties: GatewayIdentifyConnectionProps::default(),
            compress: Some(false),
            large_threshold: None,
            shard: None,
            presence: None,
            intents: None,
            capabilities: Some(8189),
        }
    }
}

impl GatewayIdentifyPayload {
    /// Creates an identify payload with the same default capabilities as the official client
    pub fn default_w_client_capabilities() -> Self {
        Self {
            capabilities: Some(8189), // Default capabilities for a client
            ..Self::default()
        }
    }

    /// Creates an identify payload with all possible capabilities
    pub fn default_w_all_capabilities() -> Self {
        Self {
            capabilities: Some(i32::MAX), // Since discord uses bitwise for capabilities, this has almost every bit as 1, so all capabilities
            ..Self::default()
        }
    }
}

impl WebSocketEvent for GatewayIdentifyPayload {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde_as]
pub struct GatewayIdentifyConnectionProps {
    /// Almost always sent
    ///
    /// ex: "Linux", "Windows", "Mac OS X"
    ///
    /// ex (mobile): "Windows Mobile", "iOS", "Android", "BlackBerry"
    pub os: String,
    /// Almost always sent
    ///
    /// ex: "Firefox", "Chrome", "Opera Mini", "Opera", "Blackberry", "Facebook Mobile", "Chrome iOS", "Mobile Safari", "Safari", "Android Chrome", "Android Mobile", "Edge", "Konqueror", "Internet Explorer", "Mozilla", "Discord Client"
    pub browser: String,
    /// Sometimes not sent, acceptable to be ""
    ///
    /// Speculation:
    /// Only sent for mobile devices
    ///
    /// ex: "BlackBerry", "Windows Phone", "Android", "iPhone", "iPad", ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub device: Option<String>,
    /// Almost always sent, most commonly en-US
    ///
    /// ex: "en-US"
    pub system_locale: String,
    /// Almost always sent
    ///
    /// ex: any user agent, most common is "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36"
    pub browser_user_agent: String,
    /// Almost always sent
    ///
    /// ex: "113.0.0.0"
    pub browser_version: String,
    /// Sometimes not sent, acceptable to be ""
    ///
    /// ex: "10" (For os = "Windows")
    #[serde_as(as = "NoneAsEmptyString")]
    pub os_version: Option<String>,
    /// Sometimes not sent, acceptable to be ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer: Option<String>,
    /// Sometimes not sent, acceptable to be ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub referring_domain: Option<String>,
    /// Sometimes not sent, acceptable to be ""
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer_current: Option<String>,
    /// Almost always sent, most commonly "stable"
    pub release_channel: String,
    /// Almost always sent, identifiable if default is 0, should be around 199933
    pub client_build_number: u64,
    //pub client_event_source: Option<?>
}

impl Default for GatewayIdentifyConnectionProps {
    /// Uses the most common, 25% data
    fn default() -> Self {
        Self::common()
    }
}

impl GatewayIdentifyConnectionProps {
    /// Returns a minimal, least data possible default
    fn minimal() -> Self {
        Self {
            os: String::new(),
            browser: String::new(),
            device: None,
            system_locale: String::from("en-US"),
            browser_user_agent: String::new(),
            browser_version: String::new(),
            os_version: None,
            referrer: None,
            referring_domain: None,
            referrer_current: None,
            release_channel: String::from("stable"),
            client_build_number: 0,
        }
    }

    /// Returns the most common connection props so we can't be tracked
    pub fn common() -> Self {
        Self {
            // See https://www.useragents.me/#most-common-desktop-useragents
            // 25% of the web
            //default.browser_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36".to_string();
            browser: String::from("Chrome"),
            browser_version: String::from("113.0.0.0"),
            system_locale: String::from("en-US"),
            os: String::from("Windows"),
            os_version: Some(String::from("10")),
            client_build_number: 222963,
            release_channel: String::from("stable"),
            ..Self::minimal()
        }
    }
}
