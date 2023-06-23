use http::response;
use reqwest::{RequestBuilder, Response};

use crate::{
    api::limits::LimitType,
    errors::{ChorusLibError, ChorusResult},
    instance::UserMeta,
};

pub struct ChorusRequest {
    pub request: RequestBuilder,
    pub limit_type: LimitType,
}

impl ChorusRequest {
    pub async fn send_request(self, user: &mut UserMeta) -> ChorusResult<Response> {
        let belongs_to = user.belongs_to.borrow();
        if belongs_to.limits.is_some()
            && !ChorusRequest::can_send_request(belongs_to.limits.unwrap(), &self.limit_type)
        {
            return Err(ChorusLibError::RateLimited {
                bucket: format!("{:?}", self.limit_type),
            });
        }
        let result = match belongs_to
            .client
            .execute(self.request.build().unwrap())
            .await
        {
            Ok(result) => result,
            Err(e) => {
                return Err(ChorusLibError::ReceivedErrorCodeError {
                    error_code: e.to_string(),
                })
            }
        };
        drop(belongs_to);
        if (!result.status().is_success()) {
            return Err(ChorusRequest::interpret_error(result));
        }
        ChorusRequest::update_rate_limits(user, &self.limit_type);
        Ok(result)
    }

    fn can_send_request(ratelimits: &Ratelimits, limit_type: &LimitType) -> bool {}

    fn interpret_error(response: reqwest::Response) -> ChorusLibError {}

    fn update_rate_limits(user: &mut UserMeta, limit_type: &LimitType) {}
}
