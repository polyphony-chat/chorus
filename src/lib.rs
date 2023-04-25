mod api;
mod errors;
mod gateway;
mod instance;
mod limit;
mod voice;

use url::{ParseError, Url};
#[derive(Clone, Default, Debug, PartialEq, Eq)]

/// A URLBundle is a struct which bundles together the API-, Gateway- and CDN-URLs of a Spacebar
/// instance.
pub struct URLBundle {
    pub api: String,
    pub wss: String,
    pub cdn: String,
}

impl URLBundle {
    pub fn new(api: String, wss: String, cdn: String) -> Self {
        Self {
            api: URLBundle::parse_url(api),
            wss: URLBundle::parse_url(wss),
            cdn: URLBundle::parse_url(cdn),
        }
    }

    /// parse(url: String) parses a URL using the Url library and formats it in a standardized
    /// way. If no protocol is given, HTTP (not HTTPS) is assumed.
    /// # Example:
    /// ```rs
    /// let url = parse_url("localhost:3000");
    /// println!("{}", url);  
    /// ```
    /// `-> Outputs "http://localhost:3000".`
    pub fn parse_url(url: String) -> String {
        let url = match Url::parse(&url) {
            Ok(url) => {
                if url.scheme() == "localhost" {
                    return URLBundle::parse_url(format!("http://{}", url));
                }
                url
            }
            Err(ParseError::RelativeUrlWithoutBase) => {
                let url_fmt = format!("http://{}", url);
                return URLBundle::parse_url(url_fmt);
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

    pub fn get_api(&self) -> &str {
        &self.api
    }

    pub fn get_cdn(&self) -> &str {
        &self.cdn
    }

    pub fn get_wss(&self) -> &str {
        &self.wss
    }
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn test_parse_url() {
        let mut result = URLBundle::parse_url(String::from("localhost:3000/"));
        assert_eq!(result, String::from("http://localhost:3000"));
        result = URLBundle::parse_url(String::from("https://some.url.com/"));
        assert_eq!(result, String::from("https://some.url.com"));
        result = URLBundle::parse_url(String::from("https://some.url.com/"));
        assert_eq!(result, String::from("https://some.url.com"));
        result = URLBundle::parse_url(String::from("https://some.url.com"));
        assert_eq!(result, String::from("https://some.url.com"));
    }
}
