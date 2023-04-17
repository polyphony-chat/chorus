use regex::Regex;

use crate::api::schemas::schemas::InstancePoliciesSchema;
use crate::gateway::Gateway;
use crate::limit::LimitedRequester;
use crate::URLBundle;

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
/**
The [`Instance`] what you will be using to perform all sorts of actions on the Spacebar server.
 */
pub struct Instance {
    main_url: String,
    urls: URLBundle,
    instance_info: InstancePoliciesSchema,
    requester: LimitedRequester,
    gateway: Gateway,
    users: HashMap<Token, String>,
}

impl Instance {
    pub fn new() {}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: String,
}

impl Token {
    pub fn new(token: String) -> Result<Token, TokenFormatError> {
        let token_regex = Regex::new(r"/[\w-]{24}\.[\w-]{6}\.[\w-]{27}/").unwrap();
        let mfa_token_regex = Regex::new(r"/mfa\.[\w-]{84}/").unwrap();
        if !token_regex.is_match(&token.as_str()) && !mfa_token_regex.is_match(&token.as_str()) {
            return Err(TokenFormatError {
                message: "This does not seem to be a valid token.".to_string(),
            });
        }
        Ok(Token { token })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenFormatError {
    pub message: String,
}

impl TokenFormatError {
    fn new(message: String) -> Self {
        TokenFormatError { message }
    }
}

impl fmt::Display for TokenFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TokenFormatError {}
