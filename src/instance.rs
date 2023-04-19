use regex::internal::Inst;

use crate::api::instance;
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
    pub urls: URLBundle,
    pub instance_info: InstancePoliciesSchema,
    pub requester: LimitedRequester,
    //pub gateway: Gateway,
    //pub users: HashMap<Token, Username>,
}

impl Instance {
    /// Creates a new [`Instance`].
    /// # Arguments
    /// * `urls` - The [`URLBundle`] that contains all the URLs that are needed to connect to the Spacebar server.
    /// * `requester` - The [`LimitedRequester`] that will be used to make requests to the Spacebar server.
    /// # Errors
    /// * [`InstanceError`] - If the instance cannot be created.
    pub async fn new(
        urls: URLBundle,
        requester: LimitedRequester,
    ) -> Result<Instance, InstanceError> {
        let mut instance = Instance {
            urls,
            instance_info: InstancePoliciesSchema::new(
                // This is okay, because the instance_info will be overwritten by the instance_policies_schema() function.
                "".to_string(),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            requester,
            //gateway: (),
            //users: (),
        };
        instance.instance_info = match instance.instance_policies_schema().await {
            Ok(schema) => schema,
            Err(e) => return Err(InstanceError{message: format!("Something seems to be wrong with the instance. Cannot get information about the instance: {}", e)}),
        };
        Ok(instance)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstanceError {
    pub message: String,
}

impl InstanceError {
    fn new(message: String) -> Self {
        InstanceError { message }
    }
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InstanceError {}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Username {
    pub username: String,
}

impl Username {
    /// Creates a new [`Username`].
    /// # Arguments
    /// * `username` - The username that will be used to create the [`Username`].
    /// # Errors
    /// * [`UsernameFormatError`] - If the username is not between 2 and 32 characters.
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
