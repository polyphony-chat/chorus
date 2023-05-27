use crate::types::events::{PresenceUpdate, WebSocketEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: GatewayIdentifyConnectionProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i16>, //default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<(i32, i32)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceUpdate>,
    // What is the difference between these two?
    // Intents is documented, capabilities is used in users
    // I wonder if these are interchangable..
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
    /// Basically pretends to be an official client on windows 10, with chrome 113.0.0.0
    pub fn common() -> Self {
        Self { token: "".to_string(), properties: GatewayIdentifyConnectionProps::default(), compress: Some(false), large_threshold: None, shard: None, presence: None, intents: None, capabilities: Some(8189)  }
    }
}

impl GatewayIdentifyPayload {
    /// Creates an identify payload with the same default capabilities as the official client
    pub fn default_w_client_capabilities() -> Self {
        let mut def = Self::default();
        def.capabilities = Some(8189); // Default capabilities for a client
        def
    }

    /// Creates an identify payload with all possible capabilities
    pub fn default_w_all_capabilities() -> Self {
        let mut def = Self::default();
        def.capabilities = Some(i32::MAX); // Since discord uses bitwise for capabilities, this has almost every bit as 1, so all capabilities
        def
    }
}

impl WebSocketEvent for GatewayIdentifyPayload {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GatewayIdentifyConnectionProps {
    /// Almost always sent
    /// 
    /// ex: "Linux", "Windows", "Mac OS X"
    /// 
    /// ex (mobile): "Windows Mobile", "iOS", "Android", "BlackBerry"
    pub os: String,
    /// Almost always sent
    /// 
    /// ex: "Firefox", "Chrome", "Opera Mini", "Opera", "Blackberry", "Facebook Mobile", "Chrome iOS", "Mobile Safari", "Safari", "Android Chrome", "Android Mobile", "Edge", "Konqueror", "Internet Explorer", "Mozilla"
    pub browser: String,
    /// Sometimes not sent, acceptable to be ""
    /// 
    /// Speculation:
    /// Only sent for mobile devices
    /// 
    /// ex: "BlackBerry", "Windows Phone", "Android", "iPhone", "iPad", ""
    pub device: String,
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
    pub os_version: String,
    /// Sometimes not sent, acceptable to be ""
    pub referrer: String,
    /// Sometimes not sent, acceptable to be ""
    pub referring_domain: String,
    /// Sometimes not sent, acceptable to be ""
    pub referrer_current: String,
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
            device: String::new(),
            system_locale: String::from("en-US"),
            browser_user_agent: String::new(),
            browser_version: String::new(),
            os_version: String::new(),
            referrer: String::new(),
            referring_domain: String::new(),
            referrer_current: String::new(),
            release_channel: String::from("stable"),
            client_build_number: 199933,
        }
    }

    /// Returns the most common connection props so we can't be tracked
    pub fn common() -> Self {
        let mut default = Self::minimal();

        // See https://www.useragents.me/#most-common-desktop-useragents
        // 25% of the web
        //default.browser_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36".to_string();
        default.browser = String::from("Chrome");
        default.browser_version = String::from("113.0.0.0");

        default.system_locale = String::from("en-US");

        default.os = String::from("Windows");
        default.os_version = String::from("10");

        default.client_build_number = 199933;
        default.release_channel = String::from("stable");

        return default;
    }
}