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
use crate::types::{
    GeneralConfiguration, Limit, LimitType, LimitsConfiguration, User, UserSettings,
};
use crate::UrlBundle;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// The [`Instance`]; what you will be using to perform all sorts of actions on the Spacebar server.
/// If `limits_information` is `None`, then the instance will not be rate limited.
pub struct Instance {
    pub urls: UrlBundle,
    pub instance_info: GeneralConfiguration,
    pub limits_information: Option<LimitsInformation>,
    #[serde(skip)]
    pub client: Client,
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.urls == other.urls
            && self.instance_info == other.instance_info
            && self.limits_information == other.limits_information
    }
}

impl Eq for Instance {}

impl std::hash::Hash for Instance {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.urls.hash(state);
        self.instance_info.hash(state);
        if let Some(inf) = &self.limits_information {
            inf.hash(state);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq)]
pub struct LimitsInformation {
    pub ratelimits: HashMap<LimitType, Limit>,
    pub configuration: RateLimits,
}

impl std::hash::Hash for LimitsInformation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, v) in self.ratelimits.iter() {
            k.hash(state);
            v.hash(state);
        }
        self.configuration.hash(state);
    }
}

impl PartialEq for LimitsInformation {
    fn eq(&self, other: &Self) -> bool {
        self.ratelimits.iter().eq(other.ratelimits.iter())
            && self.configuration == other.configuration
    }
}

impl Instance {
    /// Creates a new [`Instance`] from the [relevant instance urls](UrlBundle). To create an Instance from one singular url, use [`Instance::from_root_url()`].
    pub async fn new(urls: UrlBundle) -> ChorusResult<Instance> {
        let is_limited: Option<LimitsConfiguration> = Instance::is_limited(&urls.api).await?;
        let limit_information;

        if let Some(limits_configuration) = is_limited {
            let limits = ChorusRequest::limits_config_to_hashmap(&limits_configuration.rate);
            limit_information = Some(LimitsInformation {
                ratelimits: limits,
                configuration: limits_configuration.rate,
            });
        } else {
            limit_information = None
        }
        let mut instance = Instance {
            urls: urls.clone(),
            // Will be overwritten in the next step
            instance_info: GeneralConfiguration::default(),
            limits_information: limit_information,
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

    /// Creates a new [`Instance`] by trying to get the [relevant instance urls](UrlBundle) from a root url.
    /// Shorthand for `Instance::new(UrlBundle::from_root_domain(root_domain).await?)`.
    ///
    /// If `limited` is `true`, then Chorus will track and enforce rate limits for this instance.
    pub async fn from_root_url(root_url: &str) -> ChorusResult<Instance> {
        let urls = UrlBundle::from_root_url(root_url).await?;
        Instance::new(urls).await
    }

    pub async fn is_limited(api_url: &str) -> ChorusResult<Option<LimitsConfiguration>> {
        let api_url = UrlBundle::parse_url(api_url.to_string());
        let client = Client::new();
        let request = client
            .get(format!("{}/policies/instance/limits", &api_url))
            .header(http::header::ACCEPT, "application/json")
            .build()?;
        let resp = match client.execute(request).await {
            Ok(response) => response,
            Err(_) => return Ok(None),
        };
        match resp.json::<LimitsConfiguration>().await {
            Ok(limits) => Ok(Some(limits)),
            Err(_) => Ok(None),
        }
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

impl PartialEq for ChorusUser {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
            && self.limits == other.limits
            && self.gateway.url == other.gateway.url
    }
}

impl Eq for ChorusUser {}

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
        let gateway = Gateway::spawn(wss_url).await.unwrap();
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
