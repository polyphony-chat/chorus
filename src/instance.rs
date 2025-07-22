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

use crate::errors::{ChorusError, ChorusResult};
use crate::gateway::{events::Events, Gateway, GatewayHandle, GatewayOptions};
use crate::ratelimiter::ChorusRequest;
use crate::types::types::subconfigs::limits::rates::RateLimits;
use crate::types::{
    ClientProperties, GatewayIdentifyPayload, GeneralConfiguration, Limit, LimitType,
    LimitsConfiguration, MfaToken, MfaTokenSchema, MfaVerifySchema, Shared, User, UserSettings,
};
use crate::UrlBundle;

/// A builder pattern type for [Instance]
#[derive(Debug, Clone, Default)]
pub struct InstanceBuilder {
    /// The provided root URL, if any
    ///
    /// Usually set with [InstanceBuilder::new]
    ///
    /// One of this field or `urls` is required
    pub root_url: Option<String>,

    /// The provided full URLs, if any
    ///
    /// Usually set with [InstanceBuilder::from_url_bundle]
    ///
    /// One of this field or `root_url` is required
    pub urls: Option<UrlBundle>,

    /// The custom provided [InstanceSoftware]
    ///
    /// See [InstanceBuilder::with_software]
    pub software: Option<InstanceSoftware>,

    /// The custom provided [GatewayOptions]
    ///
    /// See [InstanceBuilder::with_gateway_options]
    pub gateway_options: Option<GatewayOptions>,

    /// Custom provided [ClientProperties] (telemetry data), if any
    ///
    /// These will be used in the instance's requests, and will be inherited by new [ChorusUser]s.
    ///
    /// See [InstanceBuilder::with_client_properties]
    pub default_client_properties: Option<ClientProperties>,

    /// Whether or not to skip trying to fetch the instance's ratelimit configuration.
    ///
    /// `false` by default.
    ///
    /// See [InstanceBuilder::skip_fetching_ratelimits]
    pub should_skip_fetching_ratelimits: bool,

    /// Whether or not to skip trying to fetch the instance's general configuration (which contains
    /// general information about the instance - see [GeneralConfiguration]).
    ///
    /// `false` by default.
    ///
    /// See [InstanceBuilder::skip_fetching_general_info]
    pub should_skip_fetching_general_info: bool,

    /// The default gateway [`Events`] new gateway connections will inherit.
    ///
    /// This field can be used to subscribe to events that are received before we get access to the
    /// gateway handle object on new [ChorusUser]s created with [Instance::login_account],
    /// [Instance::login_with_token] and [Instance::register_account]
    ///
    /// You should subscribe your [`Error`](crate::errors::GatewayError) and [`Ready`](crate::types::GatewayReady) observers here, as well as any observers you want to receive from all connections.
    pub default_gateway_events: Events,
}

impl InstanceBuilder {
    /// Creates an [`InstanceBuilder`] by providing only the root url of the [`Instance`].
    ///
    /// Once you have set all the options you want, use [InstanceBuilder::build()].
    ///
    /// When the [`Instance`] is built, it will try to automatically discover the remaining urls.
    ///
    /// Note that some [`Instance`]s don't support this. If that is the case, you will need to use
    /// [InstanceBuilder::from_url_bundle] and provide the remaining urls manually.
    pub fn new(root_url: String) -> InstanceBuilder {
        InstanceBuilder {
            root_url: Some(root_url.to_string()),
            ..Default::default()
        }
    }

    /// Creates an [`InstanceBuilder`] by providing a [full set of URLs](UrlBundle).
    ///
    /// Once you have set all the options you want, use [InstanceBuilder::build()].
    ///
    /// This is equivalent to [InstanceBuilder::new] and should be used if that method cannot
    /// automatically find all the urls.
    pub fn from_url_bundle(urls: UrlBundle) -> InstanceBuilder {
        InstanceBuilder {
            urls: Some(urls),
            ..Default::default()
        }
    }

    /// Creates an [`InstanceBuilder`] by providing a [full set of URLs](UrlBundle).
    ///
    /// Once you have set all the options you want, use [InstanceBuilder::build()].
    ///
    /// This is equivalent to [InstanceBuilder::new] and should be used if that method cannot
    /// automatically find all the urls.
    ///
    /// Alias of [InstanceBuilder::from_url_bundle]
    pub fn from_urls(urls: UrlBundle) -> InstanceBuilder {
        Self::from_url_bundle(urls)
    }

    /// Manually specifies the type of software the [`Instance`] is running.
    ///
    /// This should only be used if you're 100% sure, as setting it wrongly can cause
    /// (de)serialization errors or undefined behaviours.
    ///
    /// Normally we'll ping a few endpoints to discover it automatically, but this
    /// can reveal things about the client you may not want to.
    ///
    /// (To also skip other optional requests that may reveal too much, see
    /// [Self::skip_optional_requests])
    ///
    /// See [`InstanceSoftware`] for possible values
    pub fn with_software(self, software: InstanceSoftware) -> InstanceBuilder {
        let mut s = self;
        s.software = Some(software);
        s
    }

    /// Manually sets the [`GatewayOptions`] the instance will use when spawning new connections
    /// (when logging in and registering new accounts).
    ///
    /// These options impact the low-level workings of the gateway, such as the encoding and
    /// compression method used.
    ///
    /// They are heavily dependent on what the instance supports and therefore the instance
    /// [software](InstanceSoftware).
    ///
    /// They are usually optimally set automatically, but setting them manually may help for compatibility or development purposes.
    ///
    /// See [`GatewayOptions`] for possible values
    pub fn with_gateway_options(self, options: GatewayOptions) -> InstanceBuilder {
        let mut s = self;
        s.gateway_options = Some(options);
        s
    }

    /// Manually sets the instance's [ClientProperties] (telemetry data)
    ///
    /// These will be used in the instance's requests, and will be inherited by new [ChorusUser]s.
    ///
    /// See [`ClientProperties`]
    pub fn with_client_properties(self, properties: ClientProperties) -> InstanceBuilder {
        let mut s = self;
        s.default_client_properties = Some(properties);
        s
    }

    /// Sets whether or not to skip trying to fetch the instance's ratelimit configuration.
    ///
    /// `false` by default.
    ///
    /// You may consider setting this to `true` if you know your instance does not
    /// have those endpoints and you want to avoid making the extra requests.
    pub fn skip_fetching_ratelimits(self, should_skip: bool) -> InstanceBuilder {
        let mut s = self;
        s.should_skip_fetching_ratelimits = should_skip;
        s
    }

    /// Sets whether or not to skip trying to fetch the instance's general configuration (which contains
    /// general information about the instance - see [GeneralConfiguration]).
    ///
    /// `false` by default.
    ///
    /// You may consider setting this to `true` if you know your instance does not
    /// have those endpoints and you want to avoid making the extra requests.
    pub fn skip_fetching_general_info(self, should_skip: bool) -> InstanceBuilder {
        let mut s = self;
        s.should_skip_fetching_general_info = should_skip;
        s
    }

    /// Sets whether or not to skip all optional requests when initalizing the instance.
    ///
    /// These requests are used to fetch info about the instance, such as its ratelimit
    /// configuration and its name, description, tos page, ... (if those are publically
    /// accessible).
    ///
    /// Note that even if this is set to `true`, certain requests may be performed to determine the
    /// instance's [software](InstanceSoftware). This can be skipped by setting in manually using
    /// [Self::with_software].
    ///
    /// This method sets both [Self::skip_fetching_ratelimits] and [Self::skip_fetching_general_info].
    ///
    /// `false` by default.
    ///
    /// You may consider setting this to `true` if you know your instance does not
    /// have those endpoints and you want to avoid making the extra requests.
    pub fn skip_optional_requests(self, should_skip: bool) -> InstanceBuilder {
        self.skip_fetching_ratelimits(should_skip)
            .skip_fetching_general_info(should_skip)
    }

    /// Tries to build an [Instance] from the data provided to the builder.
    ///
    /// Requires one of `root_url` ([InstanceBuilder::new]) or `urls`
    /// ([InstanceBuilder::from_urls]) to be provided.
    ///
    /// Note that it's recommended to store the resulting [Instance]
    /// in the [Shared] wrapper (using `.into_shared()`).
    pub async fn build(self) -> ChorusResult<Instance> {
        let urls;

        if let Some(url_bundle) = self.urls {
            urls = url_bundle;
        } else if let Some(root_url) = self.root_url {
            log::trace!("Discovering instance URLs from root URL..");
            urls = UrlBundle::from_root_url(&root_url).await?;
        } else {
            return ChorusResult::Err(ChorusError::InvalidArguments { error: "One of root_url or urls is required. See InstanceBuilder::new or InstanceBuilder::from_urls".to_string() });
        }

        // Create the object, so we can send ChorusRequests
        let mut instance = Instance {
            client: Client::new(),
            urls,
            default_gateway_events: self.default_gateway_events,
            default_client_properties: self.default_client_properties.unwrap_or_default(),

            // Will all be overwritten soon
            limits_information: None,
            instance_info: GeneralConfiguration::default(),
            gateway_options: GatewayOptions::default(),
            software: InstanceSoftware::Other,
        };

        if self.should_skip_fetching_ratelimits {
            log::trace!("Skipping instance ratelimit info fetch..");
        } else {
            instance.limits_information = match instance.is_limited().await? {
                Some(limits_configuration) => {
                    let limits =
                        ChorusRequest::limits_config_to_hashmap(&limits_configuration.rate);

                    Some(LimitsInformation {
                        ratelimits: limits,
                        configuration: limits_configuration.rate,
                    })
                }
                None => None,
            };
        }

        if self.should_skip_fetching_general_info {
            log::trace!("Skipping general instance info fetch..");
        } else {
            match instance.general_configuration_schema().await {
                Ok(info) => instance.instance_info = info,
                Err(e) => {
                    log::warn!("Could not get instance configuration schema: {e}");
                }
            };
        }

        if let Some(manual_software) = self.software {
            instance.software = manual_software;
            log::trace!("Instance software manually set to {:?}", instance.software);
        } else {
            instance.software = instance.detect_software().await;
            log::debug!(
                "Instance software automatically detected as {:?}",
                instance.software
            );
        }

        if let Some(manual_options) = self.gateway_options {
            instance.gateway_options = manual_options;
            log::trace!("Instance gateway options manually set..");
        } else {
            instance.gateway_options = GatewayOptions::for_instance_software(instance.software());
            log::trace!("Instance gateway options automatically set based off instance software.");
        }

        log::trace!("Instance successfully built!");

        Ok(instance)
    }
}

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

    #[serde(skip)]
    /// The default [ClientProperties] (telemetry data) that the instance
    /// uses for its requests and new [ChorusUser]s inherit.
    ///
    /// See [ClientProperties] for more info.
    pub default_client_properties: ClientProperties,
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

    /// Creates a new [`Instance`] from only the [relevant instance urls](UrlBundle).
    ///
    /// Equivalent to doing
    ///
    /// ```no_run
    /// # let urls = UrlBundle::new("", "", "", "");
    /// InstanceBuilder::from_url_bundle(urls).build().await
    /// ```
    ///
    /// If you need to set more options, use [InstanceBuilder].
    ///
    /// To create an [Instance] from one singular url, use [`Instance::new()`] or
    /// [InstanceBuilder::new].
    pub async fn from_url_bundle(urls: UrlBundle) -> ChorusResult<Instance> {
        InstanceBuilder::from_url_bundle(urls).build().await
    }

    /// Creates a new [`Instance`] by trying to get the [relevant instance urls](UrlBundle) from a root url.
    ///
    /// Equivalent to doing
    ///
    /// ```no_run
    /// # let root_url = "";
    /// InstanceBuilder::new(root_url).build().await
    /// ```
    ///
    /// If you need to set more options, use [InstanceBuilder].
    pub async fn new(root_url: &str) -> ChorusResult<Instance> {
        InstanceBuilder::new(root_url.to_string()).build().await
    }

    /// Tries to fetch the instance's ratelimits information
    ///
    /// Only supported on [InstanceSoftware::Symfonia] and [InstanceSoftware::SpacebarTypescript]
    pub async fn is_limited(&mut self) -> ChorusResult<Option<LimitsConfiguration>> {
        let request = ChorusRequest {
            request: Client::new()
                .get(format!("{}/policies/instance/limits", self.urls.api))
                .header(http::header::ACCEPT, "application/json"),
            limit_type: LimitType::Global,
        }
        .with_client_properties(&self.default_client_properties);

        match request
            .send_anonymous_and_deserialize_response::<LimitsConfiguration>(self)
            .await
        {
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

    /// Manually sets the default [`GatewayOptions`] the instance should use when spawning new connections.
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
    /// **Usage of this method is generally discouraged. This sets the software after
    /// the [`Instance`] has already been built assuming a different value and will (!)
    /// cause problems.**
    ///
    /// See [InstanceBuilder::with_software] for a safer way to do this.
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
    /// Assume it implements all core features of the Spacebar API.
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

        let instance_read = self.belongs_to.read().unwrap();
        let gateway_events = instance_read.default_gateway_events.clone();
        let client_properties = instance_read.default_client_properties.clone();
        drop(instance_read);

        *self.gateway.events.lock().await = gateway_events;
        self.client_properties = client_properties;

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
    /// This route is usually used in response to [ChorusError::MfaRequired].
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
