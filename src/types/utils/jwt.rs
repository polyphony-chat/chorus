// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::utils::Snowflake;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

pub fn generate_token(id: &Snowflake, email: &str, jwt_key: &str) -> String {
    let claims = Claims::new(email, id);

    build_token(&claims, jwt_key).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    /// When the token expires, unix epoch
    pub exp: i64,
    /// When the token was issued
    pub iat: i64,
    pub email: String,
    pub id: Snowflake,
}

impl Claims {
    pub fn new(user: &str, id: &Snowflake) -> Self {
        let unix = chrono::Utc::now().timestamp();
        Self {
            exp: unix + (60 * 60 * 24),
            id: *id,
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

pub fn decode_token(token: &str, jwt_secret: &str) -> Result<TokenData<Claims>, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    //TODO: What is this?
    //validation.sub = Some("quartzauth".to_string());
    decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )
}
