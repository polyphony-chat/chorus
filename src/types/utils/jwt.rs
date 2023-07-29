#[allow(missing_docs)]
use crate::types::utils::Snowflake;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub fn generate_token(id: &Snowflake, email: String, jwt_key: &str) -> String {
    let claims = Claims::new(&email, id);

    build_token(&claims, jwt_key).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    /// When the token expires, unix epoch
    pub exp: i64,
    /// When the token was issued
    pub iat: i64,
    pub email: String,
    pub id: String,
}

impl Claims {
    pub fn new(user: &str, id: &Snowflake) -> Self {
        let unix = chrono::Utc::now().timestamp();
        Self {
            exp: unix + (60 * 60 * 24),
            id: id.to_string(),
            iat: unix,
            email: user.to_string(),
        }
    }
}

pub fn build_token(claims: &Claims, jwt_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_key.as_bytes()),
    )
}

/*pub fn decode_token(token: &str) -> Result<TokenData<Claims>, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("quartzauth".to_string());
    decode(token, &DecodingKey::from_secret(JWT_SECRET), &validation)
}*/
