use reqwest::{Client, Request};
use std::collections::VecDeque;

struct Limit {
    limit: i64,
    remaining: i64,
    reset: i64,
}

pub struct LimitedRequester {
    http: Client,
    limit: Limit,
    requests: VecDeque<Request>, // wow, amazing
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    pub fn new() -> Self {
        LimitedRequester {
            limit: Limit {
                limit: 1,
                remaining: 1,
                reset: 0,
            },
            http: Client::new(),
            requests: VecDeque::new(),
        }
    }
}
