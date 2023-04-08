use reqwest::{Client, Request};
use std::collections::VecDeque;

// Note: There seem to be some overlapping request limiters. We need to make sure that sending a
// request checks for all the request limiters that apply, and blocks if any of the limiters are 0

pub struct Limit {
    limit: i64,
    remaining: i64,
    reset: i64,
    bucket: String,
}

pub struct LimitedRequester {
    http: Client,
    limit: Vec<Limit>,
    requests: VecDeque<Request>,
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    pub async fn new(api_url: String) -> Self {
        LimitedRequester {
            limit: LimitedRequester::check_limits(api_url).await,
            http: Client::new(),
            requests: VecDeque::new(),
        }
    }

    pub async fn check_limits(url: String) -> Vec<Limit> {
        let client = Client::new();
        let url_parsed = crate::URLBundle::parse_url(url) + "/api/policies/instance/limits";
        let result = client
            .get(url_parsed)
            .send()
            .await
            .unwrap_or_else(|e| panic!("An error occured while performing the request: {}", e))
            .text()
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "An error occured while parsing the request body string: {}",
                    e
                )
            });
        /*
        2. extract rate and absolute rate limits from response result
        3. put each different rate limit as a new object in the limit vector
        4. yeah
         */
        let mut limit_vector = Vec::new();
        limit_vector.push(Limit {
            limit: -1,
            remaining: -1,
            reset: -1,
            bucket: String::new(),
        }); // TODO: Implement
        limit_vector
    }
}

/* #[cfg(test)]  Tests work here as well, neat!
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        assert_eq!(1, 1)
    }
} */
