// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt::{Display, Formatter};

#[cfg(feature = "client")]
use crate::{instance::ChorusUser, ratelimiter::ChorusRequest};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde_as]
/// Tracking information for the current client, which is sent to
/// the discord servers in every gateway connection and http api request.
///
/// This includes data about the os, distribution, browser, client, device
/// and even window manager the user is running.
///
/// Chorus by default uses the most common data possible, pretending to be
/// an official client.
///
/// Note that these values are (for now) written into the source code, meaning that
/// older version of the crate will be less effective.
///
/// Unless a user of the library specifically goes out of their way to implement
/// it, chorus never sends real tracking data.
///
/// # Why?
///
/// You may ask, why would a library such as chorus even implement this?
///
/// There are two main reasons:
///
/// ## 1. Identifiability
///
/// Not sending tracking data at all makes users of software built with chorus
/// stick out like a sore thumb; it screams "I am using a 3rd party client".
///
/// Ideally, users of chorus-based software would blend in with users of the
/// official clients.
///
/// ## 2. Anti-abuse systems
///
/// Sadly, tracking data is also the most common way distinguish between real users
/// and poorly made self-bots abusing the user api.
///
/// # Disabling
///
/// By setting [ClientProperties::send_telemetry_headers] to false, it is possible to disable
/// sending these properties via headers in the HTTP API.
///
/// (Sending them via the gateway is required, since it is a non-optional field in the schema)
///
/// **Note that unless connecting to a server you specifically know doesn't care about these
/// headers, it is recommended to leave them enabled.**
///
/// # Profiles
///
/// Chorus contains a bunch of premade profiles:
/// - [ClientProperties::minimal]
/// - [ClientProperties::common_desktop_windows] - the most common data for a desktop client on
/// windows - this is also the profile used by default ([ClientProperties::default])
/// - [ClientProperties::common_web_windows] - the most common settings for a web client on windows
/// - [ClientProperties::common_desktop_mac_os] - the most common settings for a desktop client on macos
/// - [ClientProperties::common_desktop_linux] - the most common settings for a desktop client on linux
///
/// If you wish to create your own profile, please use `..ClientProperties::minimal()` instead of `..ClientProperties::default()` for unset fields.
///
/// # Reference
/// See <https://docs.discord.sex/reference#client-properties>
pub struct ClientProperties {
    /// **Not part of the sent data**
    ///
    /// If set to false, disables sending X-Super-Properties, X-Discord-Locale and X-Debug-Options
    /// headers in the HTTP API.
    ///
    /// Note that unless connecting to a server you specifically know doesn't care about these
    /// headers, it is recommended to leave them enabled.
    #[serde(skip)]
    pub send_telemetry_headers: bool,

    /// Always sent, must be provided
    ///
    /// See [ClientOs] for more details
    #[serde(default)]
    pub os: ClientOs,

    /// Always sent, must be provided
    ///
    /// Can also be an empty string
    ///
    /// See [ClientOsVersion] for more details
    #[serde(default)]
    pub os_version: ClientOsVersion,

    /// The operating system SDK version
    ///
    /// Not required
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_sdk_version: Option<String>,

    /// The operating system architecture
    ///
    /// e.g. "x64" or "arm64"
    ///
    /// Not required
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,

    /// The architecture of the desktop app
    ///
    /// e.g. "x64" or "arm64"
    ///
    /// Not required
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arch: Option<String>,

    /// Always sent, must be provided
    ///
    /// Also includes desktop clients, not just web browsers
    ///
    /// See [ClientBrowser] for more details
    #[serde(default)]
    pub browser: ClientBrowser,

    /// Always sent, must be provided
    ///
    /// May be an empty string for mobile clients
    ///
    /// See [ClientUserAgent] for more details
    #[serde(default)]
    #[serde(rename = "browser_user_agent")]
    pub user_agent: ClientUserAgent,

    /// Always sent, must be provided
    ///
    /// ex: "130.0.0.0"
    #[serde(default)]
    pub browser_version: String,

    /// Always sent, must be provided
    ///
    /// Current is ~ 355624
    ///
    /// See [ClientBuildNumber] for more details
    #[serde(default)]
    pub client_build_number: ClientBuildNumber,

    /// The native metadata version of the desktop client, if using the new update system.
    #[serde(default)]
    pub native_build_number: Option<u64>,

    /// The mobile client version
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,

    /// The alternate event source this request originated from
    ///
    /// # Reference
    /// See <https://docs.discord.sex/reference#client-event-source>
    #[serde(default)]
    pub client_event_source: Option<String>,

    /// The release channel of the desktop / web client
    ///
    /// See [ClientReleaseChannel] for more details
    #[serde(default)]
    pub release_channel: ClientReleaseChannel,

    /// Always sent, must be provided
    ///
    /// most commonly "en-US" ([ClientSystemLocale::en_us])
    ///
    /// See [ClientSystemLocale] for more details
    #[serde(default)]
    pub system_locale: ClientSystemLocale,

    /// Sometimes not sent, acceptable to be an empty string
    ///
    /// Speculation:
    /// Only sent for mobile devices
    ///
    /// ex: "BlackBerry", "Windows Phone", "Android", "iPhone", "iPad", ""
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    /// A unique identifier for the mobile device,
    /// (random UUID on android, IdentifierForVendor on iOS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_vendor_id: Option<String>,

    /// The linux window manager running the client.
    ///
    /// Acquired by the official client as
    /// (env.XDG_CURRENT_DESKTOP ?? "unknown" + "," + env.GDMSESSION ?? "unknown")
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_manager: Option<String>,

    /// The linux distribution running the client.
    ///
    /// Acquired by the official client as the output of lsb_release -ds
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distro: Option<String>,

    /// The url that referred the user to the client
    ///
    /// Sent as an empty string if there is no referrer
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer: Option<String>,

    /// Same as referrer, but for the current session
    ///
    /// Sent as an empty string if there is no referrer
    #[serde_as(as = "NoneAsEmptyString")]
    pub referrer_current: Option<String>,

    /// The domain of the url that referred the user to the client
    ///
    /// Sent as an empty string if there is no referrer
    #[serde_as(as = "NoneAsEmptyString")]
    pub referring_domain: Option<String>,

    /// Same as referring_domain but for the current session
    ///
    /// Sent as an empty string if there is no referrer
    #[serde_as(as = "NoneAsEmptyString")]
    pub referring_domain_current: Option<String>,

    /// The search engine which referred the user to the client.
    ///
    /// Common values are "google", "bing", "yahoo" and "duckduckgo"
    ///
    /// # Reference
    /// See <https://docs.discord.sex/reference#search-engine>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_engine: Option<String>,

    /// Same as search_engine, but for the current session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_engine_current: Option<String>,

    /// Whether the client has modifications, e.g. BetterDiscord
    ///
    /// Note that these modifications usually don't make themselves known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_client_mods: Option<bool>,
}

impl Default for ClientProperties {
    /// Uses the most common, data
    fn default() -> Self {
        Self::common()
    }
}

impl ClientProperties {
    /// Returns a minimal, least data possible default
    ///
    /// If creating your own profile, use `..Self::minimal()` instead of `..Self::default()`
    pub fn minimal() -> Self {
        Self {
            send_telemetry_headers: true,
            os: ClientOs::custom(String::new()),
            os_version: ClientOsVersion::custom(String::new()),
            browser: ClientBrowser::custom(String::new()),
            system_locale: ClientSystemLocale::en_us(),
            user_agent: ClientUserAgent::custom(String::new()),
            browser_version: String::new(),
            release_channel: ClientReleaseChannel::stable(),
            client_build_number: ClientBuildNumber::custom(0),
            os_sdk_version: None,
            os_arch: None,
            app_arch: None,
            native_build_number: None,
            client_version: None,
            device: None,
            device_vendor_id: None,
            window_manager: None,
            distro: None,
            referrer: None,
            referrer_current: None,
            referring_domain: None,
            referring_domain_current: None,
            search_engine: None,
            search_engine_current: None,
            has_client_mods: None,
            client_event_source: None,
        }
    }

    /// Returns the most common properties for desktop web on windows
    ///
    /// Currently chrome 132.0.0 on windows 10
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_web_windows() -> Self {
        // 24% of the web
        Self {
            os: ClientOs::Windows,
            os_version: ClientOsVersion::common_windows(),
            browser: ClientBrowser::chrome_desktop(),
            browser_version: String::from("132.0.0"),
            user_agent: ClientUserAgent::common_web_windows(),
            system_locale: ClientSystemLocale::en_us(),
            client_build_number: ClientBuildNumber::latest(),
            release_channel: ClientReleaseChannel::stable(),
            has_client_mods: Some(false),
            ..Self::minimal()
        }
    }

    /// Returns the most common properties for the desktop client on windows
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_desktop_windows() -> Self {
        Self {
            os: ClientOs::Windows,
            os_version: ClientOsVersion::common_windows(),
            browser: ClientBrowser::discord_desktop(),
            browser_version: String::from("130.0.0"),
            user_agent: ClientUserAgent::common_desktop_windows(),
            system_locale: ClientSystemLocale::en_us(),
            client_build_number: ClientBuildNumber::latest(),
            os_arch: Some(String::from("x64")),
            app_arch: Some(String::from("x64")),
            has_client_mods: Some(false),
            release_channel: ClientReleaseChannel::stable(),
            ..Self::minimal()
        }
    }

    /// Returns the most common properties for the desktop client on mac os
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_desktop_mac_os() -> Self {
        Self {
            os: ClientOs::MacOs,
            os_version: ClientOsVersion::common_mac_os(),
            browser: ClientBrowser::discord_desktop(),
            browser_version: String::from("130.0.0"),
            user_agent: ClientUserAgent::common_desktop_macos(),
            system_locale: ClientSystemLocale::en_us(),
            client_build_number: ClientBuildNumber::latest(),
            os_arch: Some(String::from("arm64")),
            app_arch: Some(String::from("arm64")),
            has_client_mods: Some(false),
            release_channel: ClientReleaseChannel::stable(),
            ..Self::minimal()
        }
    }

    /// Returns the most common properties for the desktop client on linux
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_desktop_linux() -> Self {
        Self {
            os: ClientOs::Linux,
            os_version: ClientOsVersion::latest_linux(),
            browser: ClientBrowser::discord_desktop(),
            browser_version: String::from("130.0.0"),
            user_agent: ClientUserAgent::common_desktop_windows(),
            system_locale: ClientSystemLocale::en_us(),
            client_build_number: ClientBuildNumber::latest(),
            os_arch: Some(String::from("x64")),
            app_arch: Some(String::from("x64")),
            has_client_mods: Some(false),
            release_channel: ClientReleaseChannel::stable(),
            ..Self::minimal()
        }
    }

    /// Returns the most common properties to reduce tracking, currently pretends to be a desktop client on Windows 10
    pub fn common() -> Self {
        Self::common_desktop_windows()
    }

    /// Encodes self to base64, for the X-Super-Properties header
    pub fn to_base64(&self) -> String {
        let as_json = serde_json::to_string(self).unwrap();

        base64::prelude::BASE64_STANDARD.encode(as_json)
    }
}

#[cfg(feature = "client")]
impl ChorusRequest {
    /// Adds client telemetry data to the request.
    ///
    /// Sets the X-Super-Properties, X-Discord-Locale and X-Debug-Options headers
    ///
    /// For more info, see [ClientProperties]
    pub(crate) fn with_client_properties(self, properties: &ClientProperties) -> ChorusRequest {
        // If they are specifically disabled, just return the unmodified request
        if !properties.send_telemetry_headers {
            return self;
        }

        let mut request = self;

        let properties_as_b64 = properties.to_base64();

        request.request = request
            .request
            .header("X-Super-Properties", properties_as_b64);

        // Fake discord locale as being the same as system locale
        request.request = request
            .request
            .header("X-Discord-Locale", &properties.system_locale.0);
        request.request = request
            .request
            .header("X-Debug-Options", "bugReporterEnabled");

        // TODO: X-Discord-Timezone
        //
        // Does spoofing this have any real effect?
        //
        // Also, there is no clear "most common" option
        // the most populous is UTC+08, but who's to say
        // it's discord's most common one
        //
        // The US has the biggest market share per country (30%),
        // which makes something like eastern time an option
        //
        // My speculation however is that just sending EST still
        // sticks out, since most users will send a timezone like
        // America/Toronto
        //
        // koza, 30/12/2024

        // Note: User-Agent is set in ChorusRequest::send_request, since
        // we want to send it for every request, even if it isn't
        // to discord's servers directly

        request
    }

    /// Adds client telemetry data for a [ChorusUser] to the request.
    ///
    /// Sets the X-Super-Properties, X-Discord-Locale and X-Debug-Options headers
    ///
    /// For more info, see [ClientProperties]
    pub(crate) fn with_client_properties_for(self, user: &ChorusUser) -> ChorusRequest {
        self.with_client_properties(&user.client_properties)
    }
}

/// The operating system the client is running on.
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/reference#operating-system-type>
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
pub enum ClientOs {
    #[default]
    Windows,
    #[serde(rename = "osx")]
    MacOs,
    Linux,
    Android,
    IOS,
    Playstation,
    Unknown,
    #[serde(untagged)]
    Custom(String),
}

impl From<String> for ClientOs {
    fn from(value: String) -> Self {
        ClientOs::Custom(value)
    }
}

impl Display for ClientOs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientOs::Windows => write!(f, "Windows"),
            ClientOs::MacOs => write!(f, "Mac OS"),
            ClientOs::Linux => write!(f, "Linux"),
            ClientOs::Android => write!(f, "Android"),
            ClientOs::IOS => write!(f, "iOS"),
            ClientOs::Playstation => write!(f, "Playstation"),
            ClientOs::Unknown => write!(f, "Unknown"),
            ClientOs::Custom(s) => write!(f, "{s}"),
        }
    }
}

impl ClientOs {
    /// The most common os, currently Windows
    pub fn common() -> ClientOs {
        Self::Windows
    }

    pub fn custom(value: String) -> ClientOs {
        value.into()
    }
}

/// The operating system version the client is running on.
///
/// For windows, this is 10, 11, ...
///
/// For linux, this is the kernel version
///
/// For android, this is the sdk version
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/reference#client-properties-structure>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ClientOsVersion(String);

impl From<String> for ClientOsVersion {
    fn from(value: String) -> Self {
        ClientOsVersion(value)
    }
}

impl From<ClientOsVersion> for String {
    fn from(value: ClientOsVersion) -> Self {
        value.0
    }
}

impl ClientOsVersion {
    /// The latest os version for [ClientOs::android]
    // See https://apilevels.com/ and https://telemetrydeck.com/survey/android/Android/sdkVersions/
    pub fn latest_android() -> ClientOsVersion {
        ClientOsVersion(String::from("35"))
    }

    /// The currently most common os version for [ClientOs::android]
    // See https://apilevels.com/ and https://telemetrydeck.com/survey/android/Android/sdkVersions/
    pub fn common_android() -> ClientOsVersion {
        ClientOsVersion(String::from("34"))
    }

    /// The latest os version for [ClientOs::mac_os]
    // See https://en.wikipedia.org/wiki/MacOS_version_history and https://www.statista.com/statistics/944559/worldwide-macos-version-market-share/
    pub fn latest_mac_os() -> ClientOsVersion {
        ClientOsVersion(String::from("15"))
    }

    /// The currently most common os version for [ClientOs::mac_os]
    // See https://en.wikipedia.org/wiki/MacOS_version_history and https://www.statista.com/statistics/944559/worldwide-macos-version-market-share/
    pub fn common_mac_os() -> ClientOsVersion {
        ClientOsVersion(String::from("10.15"))
    }

    /// The latest os version for [ClientOs::ios]
    // See https://en.wikipedia.org/wiki/IOS_version_history and https://gs.statcounter.com/ios-version-market-share/
    pub fn latest_ios() -> ClientOsVersion {
        ClientOsVersion(String::from("18.3.2"))
    }

    /// The currently most common os version for [ClientOs::ios]
    // See https://en.wikipedia.org/wiki/IOS_version_history and https://gs.statcounter.com/ios-version-market-share/
    pub fn common_ios() -> ClientOsVersion {
        ClientOsVersion(String::from("18.1"))
    }

    /// The latest os version for [ClientOs::linux]
    // See https://en.wikipedia.org/wiki/Linux_kernel_version_history
    pub fn latest_linux() -> ClientOsVersion {
        ClientOsVersion(String::from("6.13"))
    }

    // Note: I couldn't find which is the most commonly used

    /// The latest os version for [ClientOs::windows]
    // See https://en.wikipedia.org/wiki/List_of_Microsoft_Windows_versions and https://gs.statcounter.com/os-version-market-share/windows/desktop/worldwide
    pub fn latest_windows() -> ClientOsVersion {
        ClientOsVersion(String::from("11"))
    }

    /// The currently most common os version for [ClientOs::windows]
    // See https://en.wikipedia.org/wiki/List_of_Microsoft_Windows_versions and https://gs.statcounter.com/os-version-market-share/windows/desktop/worldwide
    pub fn common_windows() -> ClientOsVersion {
        ClientOsVersion(String::from("10"))
    }

    pub fn custom(value: String) -> ClientOsVersion {
        value.into()
    }
}

// Empty string by default
impl Default for ClientOsVersion {
    fn default() -> Self {
        ClientOsVersion::custom(String::new())
    }
}

/// The browser the client is running on.
///
/// This also includes the desktop clients.
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/reference#browser-type>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ClientBrowser(String);

impl From<String> for ClientBrowser {
    fn from(value: String) -> Self {
        ClientBrowser(value)
    }
}

impl From<ClientBrowser> for String {
    fn from(value: ClientBrowser) -> Self {
        value.0
    }
}

impl ClientBrowser {
    /// The official discord client, on desktop
    pub fn discord_desktop() -> ClientBrowser {
        ClientBrowser(String::from("Discord Client"))
    }

    /// The official discord client, on android
    pub fn discord_android() -> ClientBrowser {
        ClientBrowser(String::from("Discord Android"))
    }

    /// The official discord client, on ios
    pub fn discord_ios() -> ClientBrowser {
        ClientBrowser(String::from("Discord iOS"))
    }

    /// The official discord client, on e.g. consoles, Xbox
    pub fn discord_embedded() -> ClientBrowser {
        ClientBrowser(String::from("Discord Embedded"))
    }

    pub fn chrome_android() -> ClientBrowser {
        ClientBrowser(String::from("Android Chrome"))
    }

    pub fn chrome_ios() -> ClientBrowser {
        ClientBrowser(String::from("Chrome iOS"))
    }

    pub fn chrome_desktop() -> ClientBrowser {
        ClientBrowser(String::from("Chrome"))
    }

    /// Generic android web browser
    pub fn generic_android() -> ClientBrowser {
        ClientBrowser(String::from("Android Mobile"))
    }

    /// Blackberry web browser
    pub fn blackberry() -> ClientBrowser {
        ClientBrowser(String::from("BlackBerry"))
    }

    /// Legacy microsoft edge
    ///
    /// Probably shouldn't be used
    pub fn edge() -> ClientBrowser {
        ClientBrowser(String::from("Edge"))
    }

    /// Facebook mobile browser
    pub fn facebook_mobile() -> ClientBrowser {
        ClientBrowser(String::from("Facebook Mobile"))
    }

    pub fn firefox() -> ClientBrowser {
        ClientBrowser(String::from("Firefox"))
    }

    pub fn internet_explorer() -> ClientBrowser {
        ClientBrowser(String::from("Internet Explorer"))
    }

    pub fn kde_konqueror() -> ClientBrowser {
        ClientBrowser(String::from("Konqueror"))
    }

    pub fn safari_ios() -> ClientBrowser {
        ClientBrowser(String::from("Mobile Safari"))
    }

    pub fn safari_desktop() -> ClientBrowser {
        ClientBrowser(String::from("Safari"))
    }

    /// Generic Mozilla-like browser
    pub fn generic_mozilla() -> ClientBrowser {
        ClientBrowser(String::from("Mozilla"))
    }

    pub fn opera() -> ClientBrowser {
        ClientBrowser(String::from("Opera"))
    }

    pub fn opera_mini() -> ClientBrowser {
        ClientBrowser(String::from("Opera Mini"))
    }

    /// The most common web browser, currently chrome on desktop
    // See https://en.wikipedia.org/wiki/Usage_share_of_web_browsers
    pub fn common() -> ClientBrowser {
        Self::chrome_desktop()
    }

    pub fn custom(value: String) -> ClientBrowser {
        value.into()
    }
}

impl Default for ClientBrowser {
    fn default() -> Self {
        ClientBrowser::common()
    }
}

/// The user agent of the browser the client is running on.
///
/// May be blank on mobile clients
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/reference#browser-type>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ClientUserAgent(pub(crate) String);

impl From<String> for ClientUserAgent {
    fn from(value: String) -> Self {
        ClientUserAgent(value)
    }
}

impl From<ClientUserAgent> for String {
    fn from(value: ClientUserAgent) -> Self {
        value.0
    }
}

impl ClientUserAgent {
    /// Returns the most common user agent used for the web client on windows
    ///
    /// Currently Chrome 131.0.0 on Windows 10, 24% of the web
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_web_windows() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3"))
    }

    /// Returns the most common user agent used on Android web
    ///
    /// Currently Chrome 132.0.0 on android
    ///
    /// See <https://www.useragents.me/#most-common-mobile-useragents>
    pub fn common_web_android() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Mobile Safari/537.3"))
    }

    /// Returns the most common user agent used for the ios web
    ///
    /// Currently Safari on ios 18.1.1
    ///
    /// See <https://www.useragents.me/#most-common-mobile-useragents>
    pub fn common_web_ios() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (iPhone; CPU iPhone OS 18_1_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1.1 Mobile/15E148 Safari/604."))
    }

    /// Returns the most common user agent used for the Mac os web client
    ///
    /// Currently Safari 18.1.1 on Mac os 10.15.7
    ///
    /// See <https://www.useragents.me/#most-common-mobile-useragents>
    pub fn common_web_mac_os() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1.1 Safari/605.1.1"))
    }

    /// Returns the most common user agent used for the Linux web client
    ///
    /// Currently Chrome 132.0.0 on Linux
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_web_linux() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3"))
    }

    /// Returns the most common user agent used for the desktop client on Windows
    ///
    /// (this is mostly a guess, since we can't get statistics from discord themselves)
    ///
    /// Desktop useragents look similar to ones on Chrome;
    /// this behaves like Windows 10 on Chrome 132.0.0.0
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_desktop_windows() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3"))
    }

    /// Returns the most common user agent used for the desktop client on macos
    ///
    /// (this is mostly a guess, since we can't get statistics from discord themselves)
    ///
    /// Desktop useragents look similar to ones on Chrome;
    /// this behaves like a Mac 10.15.7 running Chrome 132.0.0.0
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_desktop_macos() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3"))
    }

    /// Returns the most common user agent used for the desktop client on linux
    ///
    /// (this is mostly a guess, since we can't get statistics from discord themselves)
    ///
    /// Desktop useragents look similar to ones on Chrome;
    /// this behaves like Linux running Chrome 132.0.0.0
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_desktop_linux() -> ClientUserAgent {
        ClientUserAgent(String::from("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3"))
    }

    /// Returns the most common user agent used for the web client
    ///
    /// Currently Chrome 132.0.0 on Windows 10, 40% of desktop users
    ///
    /// See <https://www.useragents.me/#most-common-desktop-useragents>
    pub fn common_web() -> ClientUserAgent {
        Self::common_web_windows()
    }

    /// Returns the most common user agent
    pub fn common() -> ClientUserAgent {
        Self::common_desktop_windows()
    }

    pub fn custom(value: String) -> ClientUserAgent {
        value.into()
    }
}

impl Default for ClientUserAgent {
    fn default() -> Self {
        ClientUserAgent::common()
    }
}

/// The build number of the client we are running.
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/reference#client-properties-structure>
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ClientBuildNumber(u64);

impl From<u64> for ClientBuildNumber {
    fn from(value: u64) -> Self {
        ClientBuildNumber(value)
    }
}

impl From<ClientBuildNumber> for u64 {
    fn from(value: ClientBuildNumber) -> Self {
        value.0
    }
}

impl ClientBuildNumber {
    pub fn latest() -> ClientBuildNumber {
        377993.into()
    }

    pub fn custom(value: u64) -> ClientBuildNumber {
        value.into()
    }
}

impl Default for ClientBuildNumber {
    fn default() -> Self {
        ClientBuildNumber::latest()
    }
}

/// The release channel of the official client we are running on.
///
/// The main channels are
/// - [Self::stable]
/// - [Self::ptb] (public test build)
/// - [Self::canary] (alpha test build)
///
/// The desktop client has an additional [Self::development] channel, which follows
/// the [Self::canary] channel but is not recommended, as it can be broken or unstable at any time.
///
/// For more information about the main channels, see <https://support.discord.com/hc/en-us/articles/360035675191-Discord-Testing-Clients>
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/topics/client-distribution#desktop-release-channel> and <https://docs.discord.sex/topics/client-distribution#web-release-channel>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ClientReleaseChannel(String);

impl From<String> for ClientReleaseChannel {
    fn from(value: String) -> Self {
        ClientReleaseChannel(value)
    }
}

impl From<ClientReleaseChannel> for String {
    fn from(value: ClientReleaseChannel) -> Self {
        value.0
    }
}

impl ClientReleaseChannel {
    /// Stable web / desktop channel
    pub fn stable() -> ClientReleaseChannel {
        ClientReleaseChannel::custom(String::from("stable"))
    }

    /// Public test build web / desktop channel
    pub fn ptb() -> ClientReleaseChannel {
        ClientReleaseChannel::custom(String::from("ptb"))
    }

    /// Alpha test build web channel
    pub fn canary() -> ClientReleaseChannel {
        ClientReleaseChannel::custom(String::from("canary"))
    }

    /// Alpha test build desktop only channel, can be unstable or broken
    pub fn development() -> ClientReleaseChannel {
        ClientReleaseChannel::custom(String::from("development"))
    }

    /// The most commonly used release channel, currently stable
    pub fn common() -> ClientReleaseChannel {
        Self::stable()
    }

    pub fn custom(value: String) -> ClientReleaseChannel {
        value.into()
    }
}

impl Default for ClientReleaseChannel {
    fn default() -> Self {
        ClientReleaseChannel::common()
    }
}

/// The locale ([IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag)) of the system
/// running the client.
///
/// # Notes
/// This is used for [ClientProperties]
///
/// # Reference
/// See <https://docs.discord.sex/reference#client-properties-structure>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct ClientSystemLocale(String);

impl From<String> for ClientSystemLocale {
    fn from(value: String) -> Self {
        ClientSystemLocale(value)
    }
}

impl From<ClientSystemLocale> for String {
    fn from(value: ClientSystemLocale) -> Self {
        value.0
    }
}

impl ClientSystemLocale {
    /// The en-US locale
    pub fn en_us() -> ClientSystemLocale {
        ClientSystemLocale(String::from("en-US"))
    }

    /// The most commonly used system locale, currently en-US
    pub fn common() -> ClientSystemLocale {
        Self::en_us()
    }

    pub fn custom(value: String) -> ClientSystemLocale {
        value.into()
    }
}

impl Default for ClientSystemLocale {
    fn default() -> Self {
        ClientSystemLocale::common()
    }
}
