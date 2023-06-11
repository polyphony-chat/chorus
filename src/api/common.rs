use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::from_str;

use crate::{errors::ChorusLibError, instance::UserMeta, limit::LimitedRequester};

use super::limits::LimitType;

/// Sends a request to wherever it needs to go and performs some basic error handling.
pub async fn handle_request(
    request: RequestBuilder,
    user: &mut UserMeta,
    limit_type: LimitType,
) -> Result<reqwest::Response, crate::errors::ChorusLibError> {
    let mut belongs_to = user.belongs_to.borrow_mut();
    match LimitedRequester::new()
        .await
        .send_request(
            request,
            limit_type,
            &mut belongs_to.limits,
            &mut user.limits,
        )
        .await
    {
        Ok(response) => return Ok(response),
        Err(e) => return Err(e),
    }
}

/// Sends a request to wherever it needs to go. Returns [`None`] on success and
/// [`Some(ChorusLibError)`] on failure.
pub async fn handle_request_as_option(
    request: RequestBuilder,
    user: &mut UserMeta,
    limit_type: LimitType,
) -> Option<ChorusLibError> {
    match handle_request(request, user, limit_type).await {
        Ok(_) => None,
        Err(e) => Some(ChorusLibError::InvalidResponseError {
            error: e.to_string(),
        }),
    }
}

pub async fn deserialize_response<T: for<'a> Deserialize<'a>>(
    request: RequestBuilder,
    user: &mut UserMeta,
    limit_type: LimitType,
) -> Result<T, ChorusLibError> {
    let response = handle_request(request, user, limit_type).await.unwrap();
    let response_text = match response.text().await {
        Ok(string) => string,
        Err(e) => {
            return Err(ChorusLibError::InvalidResponseError {
                error: format!(
                    "Error while trying to process the HTTP response into a String: {}",
                    e.to_string()
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
                    e.to_string()
                ),
            })
        }
    };
    Ok(object)
}
