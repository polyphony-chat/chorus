mod client;
mod gateway;
mod http;
mod limit;
mod voice;

use url::{ParseError, Url};
#[derive(Clone, Default)]

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
                    return format!("http://{}", url);
                }
                url
            }
            Err(ParseError::RelativeUrlWithoutBase) => {
                let url = format!("http://{}", url);
                Url::parse(&url).unwrap()
            }
            Err(_) => panic!("Invalid URL"),
        };
        url.to_string()[0..url.to_string().len() - 1].to_string() // Remove '/' at the end of the URL
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
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        let mut result = URLBundle::parse_url(String::from("localhost:3000"));
        assert_eq!(result, String::from("http://localhost:3000"));
        result = URLBundle::parse_url(String::from("some.url.com/"));
        assert_eq!(result, String::from("http://some.url.com"))
    }
}
