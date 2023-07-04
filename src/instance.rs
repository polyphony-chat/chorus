use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::api::limits::{Limit, LimitType};
use crate::errors::{ChorusError, ChorusResult, FieldFormatError};
use crate::ratelimiter::ChorusRequest;
use crate::types::types::limit_configuration;
use crate::types::{GeneralConfiguration, LimitsConfiguration, User, UserSettings};
use crate::UrlBundle;

#[derive(Debug, Clone)]
/**
The [`Instance`] what you will be using to perform all sorts of actions on the Spacebar server.
 */
pub struct Instance {
    pub urls: UrlBundle,
    pub instance_info: GeneralConfiguration,
    pub limits_configuration: Option<LimitsConfiguration>,
    pub limits: Option<HashMap<LimitType, Limit>>,
    pub client: Client,
}

impl Instance {
    /// Creates a new [`Instance`].
    /// # Arguments
    /// * `urls` - The [`URLBundle`] that contains all the URLs that are needed to connect to the Spacebar server.
    /// * `requester` - The [`LimitedRequester`] that will be used to make requests to the Spacebar server.
    /// # Errors
    /// * [`InstanceError`] - If the instance cannot be created.
    pub async fn new(urls: UrlBundle, limited: bool) -> ChorusResult<Instance> {
        let limits;
        let limits_configuration;
        if limited {
            limits = Some(Limits::check_limits(urls.api).await?);
            limits_configuration = match ChorusRequest::get_limits_config(&urls.api).await {
                Ok(conf) => Some(conf),
                Err(e) => return Err(e),
            };
        } else {
            limits = None;
            limits_configuration = None;
        }
        let mut instance = Instance {
            urls: urls.clone(),
            // Will be overwritten in the next step
            instance_info: GeneralConfiguration::default(),
            limits,
            limits_configuration,
            client: Client::new(),
        };
        instance.instance_info = match instance.general_configuration_schema().await {
            Ok(schema) => schema,
            Err(e) => {
                return Err(ChorusError::CantGetInfoError {
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
    pub belongs_to: Rc<RefCell<Instance>>,
    pub token: String,
    pub limits: Option<HashMap<LimitType, Limit>>,
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
        belongs_to: Rc<RefCell<Instance>>,
        token: String,
        limits: Ratelimits,
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
