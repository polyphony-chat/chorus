// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Instance and ChorusUser objects.

use std::collections::HashMap;
use std::fmt;

use std::sync::{Arc, RwLock};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::errors::ChorusResult;
use crate::gateway::{Gateway, GatewayHandle, GatewayOptions};
use crate::ratelimiter::ChorusRequest;
use crate::types::types::subconfigs::limits::rates::RateLimits;
use crate::types::{
    GeneralConfiguration, Limit, LimitType, LimitsConfiguration, Shared, User, UserSettings,
};
use crate::UrlBundle;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// The [`Instance`]; what you will be using to perform all sorts of actions on the Spacebar server.
///
/// If `limits_information` is `None`, then the instance will not be rate limited.
pub struct Instance {
    pub urls: UrlBundle,
    pub instance_info: GeneralConfiguration,
    pub limits_information: Option<LimitsInformation>,
    #[serde(skip)]
    pub client: Client,
    #[serde(skip)]
    pub gateway_options: GatewayOptions,
}

impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        self.urls == other.urls
            && self.instance_info == other.instance_info
            && self.limits_information == other.limits_information
    }
}

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
    pub(crate) fn clone_limits_if_some(&self) -> Option<HashMap<LimitType, Limit>> {
        if self.limits_information.is_some() {
            return Some(self.limits_information.as_ref().unwrap().ratelimits.clone());
        }
        None
    }

    /// Creates a new [`Instance`] from the [relevant instance urls](UrlBundle).
    ///
    /// To create an Instance from one singular url, use [`Instance::new()`].
    pub async fn from_url_bundle(urls: UrlBundle) -> ChorusResult<Instance> {
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
            gateway_options: GatewayOptions::default(),
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

    /// Creates a new [`Instance`] by trying to get the [relevant instance urls](UrlBundle) from a root url.
    ///
    /// Shorthand for `Instance::from_url_bundle(UrlBundle::from_root_domain(root_domain).await?)`.
    pub async fn new(root_url: &str) -> ChorusResult<Instance> {
        let urls = UrlBundle::from_root_url(root_url).await?;
        Instance::from_url_bundle(urls).await
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

    /// Sets the [`GatewayOptions`] the instance will use when spawning new connections.
    ///
    /// These options are used on the gateways created when logging in and registering.
    pub fn set_gateway_options(&mut self, options: GatewayOptions) {
        self.gateway_options = options;
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
    pub belongs_to: Shared<Instance>,
    pub token: String,
    pub limits: Option<HashMap<LimitType, Limit>>,
    pub settings: Shared<UserSettings>,
    pub object: Shared<User>,
    pub gateway: GatewayHandle,
}

impl PartialEq for ChorusUser {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
            && self.limits == other.limits
            && self.gateway.url == other.gateway.url
    }
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
    /// This isn't the preferred way to create a ChorusUser.
    /// See [Instance::login_account] and [Instance::register_account] instead.
    pub fn new(
        belongs_to: Shared<Instance>,
        token: String,
        limits: Option<HashMap<LimitType, Limit>>,
        settings: Shared<UserSettings>,
        object: Shared<User>,
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
    pub(crate) async fn shell(instance: Shared<Instance>, token: String) -> ChorusUser {
        let settings = Arc::new(RwLock::new(UserSettings::default()));
        let object = Arc::new(RwLock::new(User::default()));
        let wss_url = instance.read().unwrap().urls.wss.clone();
        // Dummy gateway object
        let gateway = Gateway::spawn(wss_url, GatewayOptions::default())
            .await
            .unwrap();
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
