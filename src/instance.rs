//! Instance and ChorusUser objects.

use std::collections::HashMap;
use std::fmt;

use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::errors::ChorusResult;
use crate::gateway::{Gateway, GatewayHandle};
use crate::ratelimiter::ChorusRequest;
use crate::types::types::subconfigs::limits::rates::RateLimits;
use crate::types::{GeneralConfiguration, Limit, LimitType, User, UserSettings};
use crate::UrlBundle;

#[derive(Debug, Clone, Default)]
/// The [`Instance`]; what you will be using to perform all sorts of actions on the Spacebar server.
/// If `limits_information` is `None`, then the instance will not be rate limited.
pub struct Instance {
    pub urls: UrlBundle,
    pub instance_info: GeneralConfiguration,
    pub limits_information: Option<LimitsInformation>,
    pub client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LimitsInformation {
    pub ratelimits: HashMap<LimitType, Limit>,
    pub configuration: RateLimits,
}

impl Instance {
    /// Creates a new [`Instance`] from the [relevant instance urls](UrlBundle), where `limited` is whether or not to automatically use rate limits.
    pub async fn new(urls: UrlBundle, limited: bool) -> ChorusResult<Instance> {
        let limits_information;
        if limited {
            let limits_configuration =
                Some(ChorusRequest::get_limits_config(&urls.api).await?.rate);
            let limits = Some(ChorusRequest::limits_config_to_hashmap(
                limits_configuration.as_ref().unwrap(),
            ));
            limits_information = Some(LimitsInformation {
                ratelimits: limits.unwrap(),
                configuration: limits_configuration.unwrap(),
            });
        } else {
            limits_information = None;
        }
        let mut instance = Instance {
            urls: urls.clone(),
            // Will be overwritten in the next step
            instance_info: GeneralConfiguration::default(),
            limits_information,
            client: Client::new(),
        };
        instance.instance_info = match instance.general_configuration_schema().await {
            Ok(schema) => schema,
            Err(e) => {
                log::warn!("Could not get instance configuration schema: {}", e);
                GeneralConfiguration::default()
            }
        };
        Ok(instance)
    }
    pub(crate) fn clone_limits_if_some(&self) -> Option<HashMap<LimitType, Limit>> {
        if self.limits_information.is_some() {
            return Some(self.limits_information.as_ref().unwrap().ratelimits.clone());
        }
        None
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

#[derive(Debug, Clone)]
/// A ChorusUser is a representation of an authenticated user on an [Instance].
/// It is used for most authenticated actions on a Spacebar server.
/// It also has its own [Gateway] connection.
pub struct ChorusUser {
    pub belongs_to: Arc<RwLock<Instance>>,
    pub token: String,
    pub limits: Option<HashMap<LimitType, Limit>>,
    pub settings: Arc<RwLock<UserSettings>>,
    pub object: Arc<RwLock<User>>,
    pub gateway: GatewayHandle,
}

impl ChorusUser {
    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    /// Creates a new [ChorusUser] from existing data.
    ///
    /// # Notes
    /// This isn't the prefered way to create a ChorusUser.
    /// See [Instance::login_account] and [Instance::register_account] instead.
    pub fn new(
        belongs_to: Arc<RwLock<Instance>>,
        token: String,
        limits: Option<HashMap<LimitType, Limit>>,
        settings: Arc<RwLock<UserSettings>>,
        object: Arc<RwLock<User>>,
        gateway: GatewayHandle,
    ) -> ChorusUser {
        ChorusUser {
            belongs_to,
            token,
            limits,
            settings,
            object,
            gateway,
        }
    }

    /// Creates a new 'shell' of a user. The user does not exist as an object, and exists so that you have
    /// a ChorusUser object to make Rate Limited requests with. This is useful in scenarios like
    /// registering or logging in to the Instance, where you do not yet have a User object, but still
    /// need to make a RateLimited request. To use the [`GatewayHandle`], you will have to identify
    /// first.
    pub(crate) async fn shell(instance: Arc<RwLock<Instance>>, token: String) -> ChorusUser {
        let settings = Arc::new(RwLock::new(UserSettings::default()));
        let object = Arc::new(RwLock::new(User::default()));
        let wss_url = instance.read().unwrap().urls.wss.clone();
        // Dummy gateway object
        let gateway = Gateway::new(wss_url).await.unwrap();
        ChorusUser {
            token,
            belongs_to: instance.clone(),
            limits: instance
                .read()
                .unwrap()
                .limits_information
                .as_ref()
                .map(|info| info.ratelimits.clone()),
            settings,
            object,
            gateway,
        }
    }
}
