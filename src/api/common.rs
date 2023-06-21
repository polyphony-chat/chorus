use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::from_str;

use crate::{
    errors::{ChorusLibError, ChorusResult},
    instance::UserMeta,
    limit::LimitedRequester,
};

use super::limits::LimitType;

/// Sends a request to wherever it needs to go and performs some basic error handling.
pub async fn handle_request(
    request: RequestBuilder,
    user: &mut UserMeta,
    limit_type: LimitType,
) -> Result<reqwest::Response, crate::errors::ChorusLibError> {
    LimitedRequester::send_request(
        request,
        limit_type,
        &mut user.belongs_to.borrow_mut(),
        &mut user.limits,
    )
    .await
}

/// Sends a request to wherever it needs to go. Returns [`Ok(())`] on success and
/// [`Err(ChorusLibError)`] on failure.
pub async fn handle_request_as_result(
    request: RequestBuilder,
    user: &mut UserMeta,
    limit_type: LimitType,
) -> ChorusResult<()> {
    match handle_request(request, user, limit_type).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ChorusLibError::InvalidResponseError {
            error: e.to_string(),
        }),
    }
}

pub async fn deserialize_response<T: for<'a> Deserialize<'a>>(
    request: RequestBuilder,
    user: &mut UserMeta,
    limit_type: LimitType,
) -> ChorusResult<T> {
    let response = handle_request(request, user, limit_type).await.unwrap();
    let response_text = match response.text().await {
        Ok(string) => string,
        Err(e) => {
            return Err(ChorusLibError::InvalidResponseError {
                error: format!(
                    "Error while trying to process the HTTP response into a String: {}",
                    e
                ),
            });
        }
    };
    let object = match from_str::<T>(&response_text) {
        Ok(object) => object,
        Err(e) => {
            return Err(ChorusLibError::InvalidResponseError {
                error: format!(
                    "Error while trying to deserialize the JSON response into T: {}",
                    e
                ),
            })
        }
    };
    Ok(object)
}
