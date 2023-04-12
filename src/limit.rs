use crate::api::limits::{LimitType, Limits};

use reqwest::{Client, RequestBuilder, Response};
use std::collections::VecDeque;

// Note: There seem to be some overlapping request limiters. We need to make sure that sending a
// request checks for all the request limiters that apply, and blocks if any of the limiters are 0

#[allow(dead_code)]
pub struct LimitedRequester {
    http: Client,
    requests: VecDeque<RequestBuilder>,
    last_reset_epoch: i64,
    limits_rate: Limits,
}

impl LimitedRequester {
    /// Create a new `LimitedRequester`. `LimitedRequester`s use a `VecDeque` to store requests and
    /// send them to the server using a `Client`. It keeps track of the remaining requests that can
    /// be send within the `Limit` of an external API Ratelimiter, and looks at the returned request
    /// headers to see if it can find Ratelimit info to update itself.
    #[allow(dead_code)]
    pub async fn new(api_url: String) -> Self {
        LimitedRequester {
            http: Client::new(),
            requests: VecDeque::new(),
            last_reset_epoch: chrono::Utc::now().timestamp(),
            limits_rate: Limits::check_limits(api_url).await,
        }
    }

    fn add_to_queue(request: RequestBuilder, queue: &mut VecDeque<RequestBuilder>) {
        queue.push_back(request);
    }

    fn update_limits(&mut self, response: Response, limit_types: Vec<LimitType>) {
        // TODO: Make this work {
        let remaining = match response.headers().get("X-RateLimit-Remaining") {
            Some(remaining) => remaining,
            None => return, //false,
        };
        let limit = match response.headers().get("X-RateLimit-Limit") {
            Some(limit) => limit,
            None => return, //false,
        };
        let reset = match response.headers().get("X-RateLimit-Reset") {
            Some(reset) => reset,
            None => return, //false,
        };
    }
}
