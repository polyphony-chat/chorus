#![allow(clippy::module_inception)]

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

#[derive(Clone, Default, Debug, PartialEq, Eq)]
/// A URLBundle is a struct which bundles together the API-, Gateway- and CDN-URLs of a Spacebar
/// instance.
pub struct UrlBundle {
    pub api: String,
    pub wss: String,
    pub cdn: String,
}

impl UrlBundle {
    pub fn new(api: String, wss: String, cdn: String) -> Self {
        Self {
            api: UrlBundle::parse_url(api),
            wss: UrlBundle::parse_url(wss),
            cdn: UrlBundle::parse_url(cdn),
        }
    }

    /// parse(url: String) parses a URL using the Url library and formats it in a standardized
    /// way. If no protocol is given, HTTP (not HTTPS) is assumed.
    /// # Example:
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
