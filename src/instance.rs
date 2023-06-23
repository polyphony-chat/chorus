use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::api::limits::Limits;
use crate::errors::{ChorusLibError, ChorusResult, FieldFormatError};
use crate::types::{GeneralConfiguration, User, UserSettings};
use crate::UrlBundle;

#[derive(Debug, Clone)]
/**
The [`Instance`] what you will be using to perform all sorts of actions on the Spacebar server.
 */
pub struct Instance {
    pub urls: UrlBundle,
    pub instance_info: GeneralConfiguration,
    pub limits: RefCell<Limits>,
    pub client: Client,
}

impl Instance {
    /// Creates a new [`Instance`].
    /// # Arguments
    /// * `urls` - The [`URLBundle`] that contains all the URLs that are needed to connect to the Spacebar server.
    /// * `requester` - The [`LimitedRequester`] that will be used to make requests to the Spacebar server.
    /// # Errors
    /// * [`InstanceError`] - If the instance cannot be created.
    pub async fn new(urls: UrlBundle) -> ChorusResult<Instance> {
        let api_url = urls.api.clone();
        let mut instance = Instance {
            urls,
            // Will be overwritten in the next step
            instance_info: GeneralConfiguration::default(),
            limits: Limits::check_limits(api_url).await.into(),
            client: Client::new(),
        };
        instance.instance_info = match instance.general_configuration_schema().await {
            Ok(schema) => schema,
            Err(e) => {
                return Err(ChorusLibError::CantGetInfoError {
                    error: e.to_string(),
                });
            }
        };
        Ok(instance)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct UserMeta {
    pub belongs_to: Rc<Instance>,
    pub token: String,
    pub limits: Limits,
    pub settings: UserSettings,
    pub object: User,
}

impl UserMeta {
    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn new(
        belongs_to: Rc<Instance>,
        token: String,
        limits: Limits,
        settings: UserSettings,
        object: User,
    ) -> UserMeta {
        UserMeta {
            belongs_to,
            token,
            limits,
            settings,
            object,
        }
    }
}
