use crate::api::limits::Limits;
use crate::errors::{FieldFormatError, InstanceServerError};
use crate::URLBundle;
use polyphony_types::config::GeneralConfiguration;

use std::fmt;

#[derive(Debug, Clone)]
/**
The [`Instance`] what you will be using to perform all sorts of actions on the Spacebar server.
 */
pub struct Instance {
    pub urls: URLBundle,
    pub instance_info: GeneralConfiguration,
    pub limits: Limits,
}

impl Instance {
    /// Creates a new [`Instance`].
    /// # Arguments
    /// * `urls` - The [`URLBundle`] that contains all the URLs that are needed to connect to the Spacebar server.
    /// * `requester` - The [`LimitedRequester`] that will be used to make requests to the Spacebar server.
    /// # Errors
    /// * [`InstanceError`] - If the instance cannot be created.
    pub async fn new(urls: URLBundle) -> Result<Instance, InstanceServerError> {
        let mut instance = Instance {
            urls: urls.clone(),
            instance_info: GeneralConfiguration::new(
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
            limits: Limits::check_limits(urls.api).await,
        };
        instance.instance_info = match instance.instance_policies_schema().await {
            Ok(schema) => schema,
            Err(e) => {
                return Err(InstanceServerError::CantGetInfoError {
                    error: e.to_string(),
                })
            }
        };
        Ok(instance)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
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
    pub fn new(username: String) -> Result<Username, FieldFormatError> {
        if username.len() < 2 || username.len() > 32 {
            return Err(FieldFormatError::UsernameError);
        }
        Ok(Username { username })
    }
}
