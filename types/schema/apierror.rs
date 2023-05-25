#[cfg(feature = "poem")]
use poem::{http::StatusCode, IntoResponse, Response};
use serde_json::{json, Value};

#[derive(Debug, thiserror::Error)]
pub enum APIError {
    #[error(transparent)]
    Auth(#[from] AuthError),
}

impl APIError {
    pub fn error_payload(&self) -> Value {
        match self {
            APIError::Auth(auth_err) => auth_err.error_payload(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("INVALID_LOGIN")]
    InvalidLogin,
    #[error("INVALID_CAPTCHA")]
    InvalidCaptcha,
}

impl AuthError {
    pub fn error_code(&self) -> &str {
        match self {
            AuthError::InvalidLogin => "INVALID_LOGIN",
            AuthError::InvalidCaptcha => "INVALID_CATPCA",
        }
    }

    pub fn error_payload(&self) -> Value {
        match self {
            AuthError::InvalidLogin => json!({
                "login": {
                    "message": "auth:login.INVALID_LOGIN",
                    "code": self.error_code()
                }
            }),
            AuthError::InvalidCaptcha => json!([json!({
                "captcha_key": "TODO",
                "captcha_sitekey": "TODO",
                "captcha_service": "TODO"
            })]),
        }
    }
}

#[cfg(feature = "poem")]
impl poem::error::ResponseError for APIError {
    fn status(&self) -> StatusCode {
        match self {
            APIError::Auth(auth_err) => match auth_err {
                AuthError::InvalidLogin => StatusCode::UNAUTHORIZED,
                AuthError::InvalidCaptcha => StatusCode::BAD_REQUEST,
            },
        }
    }

    fn as_response(&self) -> Response
    where
        Self: std::error::Error + Send + Sync + 'static,
    {
        Response::builder()
            .status(self.status())
            .body(self.error_payload().to_string())
            .into_response()
    }
}
