// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Instance and ChorusUser objects.

use std::collections::HashMap;
use std::fmt;

use std::sync::{Arc, RwLock};
use std::time::Duration;

use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::errors::ChorusResult;
use crate::gateway::{events::Events, Gateway, GatewayHandle, GatewayOptions};
use crate::ratelimiter::ChorusRequest;
use crate::types::types::subconfigs::limits::rates::RateLimits;
use crate::types::{
    ClientProperties, GatewayIdentifyPayload, GeneralConfiguration, Limit, LimitType,
    LimitsConfiguration, MfaToken, MfaTokenSchema, MfaVerifySchema, Shared, User, UserSettings,
};
use crate::UrlBundle;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Represents a Spacebar-compatible [`Instance`].
///
/// This struct is most commonly used for [`Instance::login_account`],
/// [`Instance::login_with_token`] and [`Instance::register_account`].
///
/// If `limits_information` is `None`, then the instance will not be rate limited.
pub struct Instance {
    /// The URLs of the instance
    pub urls: UrlBundle,

    /// General information about the instance,
    /// including its name, description, image, ...
    ///
    /// (This is set by the instance admins)
    pub instance_info: GeneralConfiguration,

    pub(crate) software: InstanceSoftware,

    /// Ratelimit information about the instance.
    ///
    /// If this field is `None`, then the instance will not be rate limited.
    pub limits_information: Option<LimitsInformation>,

    #[serde(skip)]
    /// The reqwest HTTP request client
    pub client: Client,

    #[serde(skip)]
    pub(crate) gateway_options: GatewayOptions,

    #[serde(skip)]
    /// The default gateway [`Events`] new gateway connections will inherit.
    ///
    /// This field can be used to subscribe to events that are received before we get access to the
    /// gateway handle object on new [ChorusUser]s created with [Instance::login_account],
    /// [Instance::login_with_token] and [Instance::register_account]
    pub default_gateway_events: Events,
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

#[cfg(not(tarpaulin_include))]
impl PartialEq for LimitsInformation {
    fn eq(&self, other: &Self) -> bool {
        self.ratelimits.iter().eq(other.ratelimits.iter())
            && self.configuration == other.configuration
    }
}

impl Instance {
    #[allow(unused)]
    pub(crate) fn clone_limits_if_some(&self) -> Option<HashMap<LimitType, Limit>> {
        if self.limits_information.is_some() {
            return Some(self.limits_information.as_ref().unwrap().ratelimits.clone());
        }
        None
    }

    /// Creates a new [`Instance`] from the [relevant instance urls](UrlBundle).
    ///
    /// If `options` is `None`, the default [`GatewayOptions`] will be used.
    ///
    /// To create an Instance from one singular url, use [`Instance::new()`].
    // Note: maybe make this just take urls and then add another method which creates an instance
    // from urls and custom gateway options, since gateway options will be automatically generated?
    pub async fn from_url_bundle(
        urls: UrlBundle,
        options: Option<GatewayOptions>,
    ) -> ChorusResult<Instance> {
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
            gateway_options: options.unwrap_or_default(),
            // Will also be detected soon
            software: InstanceSoftware::Other,
            default_gateway_events: Events::default(),
        };

        instance.instance_info = match instance.general_configuration_schema().await {
            Ok(schema) => schema,
            Err(e) => {
                log::warn!("Could not get instance configuration schema: {}", e);
                GeneralConfiguration::default()
            }
        };

        instance.software = instance.detect_software().await;

        if options.is_none() {
            instance.gateway_options = GatewayOptions::for_instance_software(instance.software());
        }

        Ok(instance)
    }

    /// Creates a new [`Instance`] by trying to get the [relevant instance urls](UrlBundle) from a root url.
    ///
    /// If `options` is `None`, the default [`GatewayOptions`] will be used.
    ///
    /// Shorthand for `Instance::from_url_bundle(UrlBundle::from_root_domain(root_domain).await?)`.
    pub async fn new(root_url: &str, options: Option<GatewayOptions>) -> ChorusResult<Instance> {
        let urls = UrlBundle::from_root_url(root_url).await?;
        Instance::from_url_bundle(urls, options).await
    }

    pub async fn is_limited(api_url: &str) -> ChorusResult<Option<LimitsConfiguration>> {
        let api_url = UrlBundle::parse_url(api_url);
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

    /// Detects which [InstanceSoftware] the instance is running.
    pub async fn detect_software(&mut self) -> InstanceSoftware {
        if let Ok(version) = self.get_version().await {
            match version.server.to_lowercase().as_str() {
                "symfonia" => return InstanceSoftware::Symfonia,
                // We can dream this will be implemented one day
                "spacebar" => return InstanceSoftware::SpacebarTypescript,
                _ => {}
            }
        }

        // We know it isn't a symfonia server now, work around spacebar
        // not really having a version endpoint
        let ping = self.ping().await;

        if ping.is_ok() {
            return InstanceSoftware::SpacebarTypescript;
        }

        InstanceSoftware::Other
    }

    /// Returns the [`GatewayOptions`] the instance uses when spawning new connections.
    ///
    /// These options are used on the gateways created when logging in and registering.
    pub fn gateway_options(&self) -> GatewayOptions {
        self.gateway_options
    }

    /// Manually sets the [`GatewayOptions`] the instance should use when spawning new connections.
    ///
    /// These options are used on the gateways created when logging in and registering.
    pub fn set_gateway_options(&mut self, options: GatewayOptions) {
        self.gateway_options = options;
    }

    /// Returns which [`InstanceSoftware`] the instance is running.
    pub fn software(&self) -> InstanceSoftware {
        self.software
    }

    /// Manually sets which [`InstanceSoftware`] the instance is running.
    ///
    /// Note: you should only use this if you are absolutely sure about an instance (e. g. you run it).
    /// If set to an incorrect value, this may cause unexpected errors or even undefined behaviours.
    ///
    /// Manually setting the software is generally discouraged. Chorus should automatically detect
    /// which type of software the instance is running.
    pub fn set_software(&mut self, software: InstanceSoftware) {
        self.software = software;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// The software implementation the spacebar-compatible instance is running.
///
/// This is useful since some softwares may support additional features,
/// while other do not fully implement the api yet.
pub enum InstanceSoftware {
    /// The official typescript Spacebar server, available
    /// at <https://github.com/spacebarchat/server>
    SpacebarTypescript,
    /// The Polyphony server written in rust, available at
    /// at <https://github.com/polyphony-chat/symfonia>
    Symfonia,
    /// We could not determine the instance software or it
    /// is one we don't specifically differentiate.
    ///
    /// Assume it implements all features of the spacebar protocol.
    #[default]
    Other,
}

impl InstanceSoftware {
    /// Returns whether the software supports z-lib stream compression on the gateway
    pub fn supports_gateway_zlib(self) -> bool {
        match self {
            InstanceSoftware::SpacebarTypescript => true,
            InstanceSoftware::Symfonia => false,
            InstanceSoftware::Other => true,
        }
    }

    /// Returns whether the software supports sending data in the Erlang external term format on the gateway
    pub fn supports_gateway_etf(self) -> bool {
        match self {
            InstanceSoftware::SpacebarTypescript => true,
            InstanceSoftware::Symfonia => false,
            InstanceSoftware::Other => true,
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
///
/// It is used for most authenticated actions on a Spacebar server.
///
/// It also has its own [Gateway] connection.
pub struct ChorusUser {
    /// A reference to the [Instance] the user is registered on
    pub belongs_to: Shared<Instance>,

    /// The user's authentication token
    pub token: String,

    /// Telemetry data sent to the instance.
    ///
    /// See [ClientProperties] for more information
    pub client_properties: ClientProperties,

    /// A token for bypassing mfa, if any
    pub mfa_token: Option<MfaToken>,

    /// Ratelimit data
    pub limits: Option<HashMap<LimitType, Limit>>,

    /// The user's settings
    pub settings: Shared<UserSettings>,

    /// Information about the user
    pub object: Shared<User>,

    /// The user's connection to the gateway
    pub gateway: GatewayHandle,
}

impl ChorusUser {
    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = token.to_string();
    }

    /// Creates a new [ChorusUser] from existing data.
    ///
    /// # Notes
    /// This isn't the preferred way to create a ChorusUser.
    /// See [Instance::login_account] and [Instance::register_account] instead.
    pub fn new(
        belongs_to: Shared<Instance>,
        token: String,
        client_properties: ClientProperties,
        limits: Option<HashMap<LimitType, Limit>>,
        settings: Shared<UserSettings>,
        object: Shared<User>,
        gateway: GatewayHandle,
    ) -> ChorusUser {
        ChorusUser {
            belongs_to,
            token,
            client_properties,
            mfa_token: None,
            limits,
            settings,
            object,
            gateway,
        }
    }

    /// Updates a shell user after the login process.
    ///
    /// Fetches all the other required data from the api.
    ///
    /// The received_settings can be None, since not all login methods
    /// return user settings. If this is the case, we'll fetch them via an api route.
    pub(crate) async fn update_with_login_data(
        &mut self,
        token: String,
        received_settings: Option<Shared<UserSettings>>,
    ) -> ChorusResult<()> {
        self.token = token.clone();

        let instance_default_events = self
            .belongs_to
            .read()
            .unwrap()
            .default_gateway_events
            .clone();

        *self.gateway.events.lock().await = instance_default_events;

        let mut identify = GatewayIdentifyPayload::default_w_client_capabilities();
        identify.token = token;
        identify.properties = self.client_properties.clone();
        self.gateway.send_identify(identify).await;

        *self.object.write().unwrap() = self.get_current_user().await?;

        if let Some(passed_settings) = received_settings {
            self.settings = passed_settings;
        } else {
            *self.settings.write().unwrap() = self.get_settings().await?;
        }

        Ok(())
    }

    /// Creates a new 'shell' of a user. The user does not exist as an object, and exists so that you have
    /// a ChorusUser object to make Rate Limited requests with. This is useful in scenarios like
    /// registering or logging in to the Instance, where you do not yet have a User object, but still
    /// need to make a RateLimited request. To use the [`GatewayHandle`], you will have to identify
    /// first.
    pub(crate) async fn shell(instance: Shared<Instance>, token: &str) -> ChorusUser {
        let settings = Arc::new(RwLock::new(UserSettings::default()));
        let object = Arc::new(RwLock::new(User::default()));

        let wss_url = &instance.read().unwrap().urls.wss.clone();
        let gateway_options = instance.read().unwrap().gateway_options;

        // Dummy gateway object
        let gateway = Gateway::spawn(wss_url, gateway_options).await.unwrap();
        ChorusUser {
            token: token.to_string(),
            client_properties: ClientProperties::default(),
            mfa_token: None,
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

    /// Sends a request to complete an MFA challenge.
    ///
    /// If successful, the MFA verification JWT returned is set on the current [ChorusUser] executing the
    /// request.
    ///
    /// The JWT token expires after 5 minutes.
    ///
    /// This route is usually used in response to [ChorusError::MfaRequired](crate::ChorusError::MfaRequired).
    ///
    /// # Reference
    /// See <https://docs.discord.food/authentication#verify-mfa>
    pub async fn complete_mfa_challenge(
        &mut self,
        mfa_verify_schema: MfaVerifySchema,
    ) -> ChorusResult<()> {
        let endpoint_url = self.belongs_to.read().unwrap().urls.api.clone() + "/mfa/finish";
        let chorus_request = ChorusRequest {
            request: Client::new().post(endpoint_url).json(&mfa_verify_schema),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);

        let mfa_token_schema = chorus_request
            .send_and_deserialize_response::<MfaTokenSchema>(self)
            .await?;

        self.mfa_token = Some(MfaToken {
            token: mfa_token_schema.token,
            expires_at: Utc::now() + Duration::from_secs(60 * 5),
        });

        Ok(())
    }
}
