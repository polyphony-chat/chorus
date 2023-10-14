//! A library for interacting with one or multiple Spacebar-compatible APIs and Gateways.
//!
//! # About
//!Chorus is a Rust library that allows developers to interact with multiple Spacebar-compatible APIs and Gateways simultaneously. The library provides a simple and efficient way to communicate with these services, making it easier for developers to build applications that rely on them. Chorus is open-source and welcomes contributions from the community.
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/polyphony-chat/design/main/branding/polyphony-chorus-round-8bit.png"
)]
#![allow(clippy::module_inception)]
#![deny(
    missing_debug_implementations,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion
)]

use url::{ParseError, Url};

#[cfg(feature = "client")]
pub mod api;
pub mod errors;
#[cfg(feature = "client")]
pub mod gateway;
#[cfg(feature = "client")]
pub mod instance;
#[cfg(feature = "client")]
pub mod ratelimiter;
pub mod types;
#[cfg(feature = "client")]
pub mod voice;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
/// A URLBundle bundles together the API-, Gateway- and CDN-URLs of a Spacebar instance.
///
/// # Notes
/// All the urls can be found on the /api/policies/instance/domains endpoint of a spacebar server
pub struct UrlBundle {
    /// The api's url.
    /// Ex: `https://old.server.spacebar.chat/api`
    pub api: String,
    /// The gateway websocket url.
    /// Note that because this is a websocket url, it will always start with `wss://` or `ws://`
    /// Ex: `wss://gateway.old.server.spacebar.chat`
    pub wss: String,
    /// The CDN's url.
    /// Ex: `https://cdn.old.server.spacebar.chat`
    pub cdn: String,
}

impl UrlBundle {
    /// Creates a new UrlBundle from the relevant urls.
    pub fn new(api: String, wss: String, cdn: String) -> Self {
        Self {
            api: UrlBundle::parse_url(api),
            wss: UrlBundle::parse_url(wss),
            cdn: UrlBundle::parse_url(cdn),
        }
    }

    /// Parses a URL using the Url library and formats it in a standardized way.
    /// If no protocol is given, HTTP (not HTTPS) is assumed.
    ///
    /// # Examples:
    /// ```rs
    /// let url = parse_url("localhost:3000");
    /// ```
    /// `-> Outputs "http://localhost:3000".`
    pub fn parse_url(url: String) -> String {
        let url = match Url::parse(&url) {
            Ok(url) => {
                if url.scheme() == "localhost" {
                    return UrlBundle::parse_url(format!("http://{}", url));
                }
                url
            }
            Err(ParseError::RelativeUrlWithoutBase) => {
                let url_fmt = format!("http://{}", url);
                return UrlBundle::parse_url(url_fmt);
            }
            Err(_) => panic!("Invalid URL"),
        };
        // if the last character of the string is a slash, remove it.
        let mut url_string = url.to_string();
        if url_string.ends_with('/') {
            url_string.pop();
        }
        url_string
    }
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn test_parse_url() {
        let mut result = UrlBundle::parse_url(String::from("localhost:3000/"));
        assert_eq!(result, String::from("http://localhost:3000"));
        result = UrlBundle::parse_url(String::from("https://some.url.com/"));
        assert_eq!(result, String::from("https://some.url.com"));
        result = UrlBundle::parse_url(String::from("https://some.url.com/"));
        assert_eq!(result, String::from("https://some.url.com"));
        result = UrlBundle::parse_url(String::from("https://some.url.com"));
        assert_eq!(result, String::from("https://some.url.com"));
    }
}
