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
    urls: URLBundle,
    instance_info: InstancePoliciesSchema,
    requester: LimitedRequester,
    gateway: Gateway,
    users: HashMap<Token, Username>,
}

impl Instance {
    pub fn new() {}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Username {
    pub username: String,
}

impl Username {
    pub fn new(username: String) -> Result<Username, UsernameFormatError> {
        if username.len() < 2 || username.len() > 32 {
            return Err(UsernameFormatError::new(
                "Username must be between 2 and 32 characters".to_string(),
            ));
        }
        return Ok(Username { username });
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UsernameFormatError {
    pub message: String,
}

impl UsernameFormatError {
    fn new(message: String) -> Self {
        UsernameFormatError { message }
    }
}

impl fmt::Display for UsernameFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UsernameFormatError {}
