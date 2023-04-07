use reqwest::{Client, Request};

struct Limit {
    limit: i64,
    remaining: i64,
    reset: i64,
}

pub struct Ratelimiter {
    http: Client,
    limit: Limit,
    requests: Box<[Request]>, // wow, amazing
}

impl Ratelimiter {
    pub fn new() -> Self {
        Ratelimiter {
            limit: Limit {
                limit: 1,
                remaining: 1,
                reset: 0,
            },
            http: Client::new(),
            requests: Box::new([]),
        }
    }
}
