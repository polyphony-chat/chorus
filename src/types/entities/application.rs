// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::utils::Snowflake;
use crate::types::Shared;
use crate::types::{Team, User};

#[allow(unused_imports)]
use super::{arc_rwlock_ptr_eq, option_arc_rwlock_ptr_eq};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://discord.com/developers/docs/resources/application#application-resource>
pub struct Application {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    #[cfg(feature = "sqlx")]
    pub r#type: Option<sqlx::types::Json<Value>>,
    #[cfg(not(feature = "sqlx"))]
    pub r#type: Option<Value>,
    pub hook: bool,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub verify_key: String,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub owner: Shared<User>,
    pub flags: ApplicationFlags,
    #[cfg(feature = "sqlx")]
    pub redirect_uris: Option<sqlx::types::Json<Vec<String>>>,
    #[cfg(not(feature = "sqlx"))]
    pub redirect_uris: Option<Vec<String>>,
    pub rpc_application_state: i64,
    pub store_application_state: i64,
    pub verification_state: i64,
    pub interactions_endpoint_url: Option<String>,
    pub integration_public: bool,
    pub integration_require_code_grant: bool,
    pub discoverability_state: i64,
    pub discovery_eligibility_flags: i64,
    #[cfg(feature = "sqlx")]
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
    #[cfg(not(feature = "sqlx"))]
    pub tags: Option<Vec<String>>,
    pub cover_image: Option<String>,
    #[cfg(feature = "sqlx")]
    pub install_params: Option<sqlx::types::Json<InstallParams>>,
    #[cfg(not(feature = "sqlx"))]
    pub install_params: Option<Shared<InstallParams>>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub team: Option<Team>,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.icon == other.icon
            && self.description == other.description
            && self.summary == other.summary
            && self.r#type == other.r#type
            && self.hook == other.hook
            && self.bot_public == other.bot_public
            && self.bot_require_code_grant == other.bot_require_code_grant
            && self.verify_key == other.verify_key
            && arc_rwlock_ptr_eq(&self.owner, &other.owner)
            && self.flags == other.flags
            && self.redirect_uris == other.redirect_uris
            && self.rpc_application_state == other.rpc_application_state
            && self.store_application_state == other.store_application_state
            && self.verification_state == other.verification_state
            && self.interactions_endpoint_url == other.interactions_endpoint_url
            && self.integration_public == other.integration_public
            && self.integration_require_code_grant == other.integration_require_code_grant
            && self.discoverability_state == other.discoverability_state
            && self.discovery_eligibility_flags == other.discovery_eligibility_flags
            && self.tags == other.tags
            && self.cover_image == other.cover_image
            && compare_install_params(&self.install_params, &other.install_params)
            && self.terms_of_service_url == other.terms_of_service_url
            && self.privacy_policy_url == other.privacy_policy_url
            && self.team == other.team
    }
}

#[cfg(not(tarpaulin_include))]
impl PartialOrd for Application {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            // Note: this skips a few fields
            // probably? not a problem
            self.id
                .cmp(&other.id)
                .then(self.name.cmp(&other.name))
                .then(self.icon.cmp(&other.icon))
                .then(self.description.cmp(&other.description))
                .then(self.summary.cmp(&other.summary))
                .then(self.hook.cmp(&other.hook))
                .then(self.bot_public.cmp(&other.bot_public))
                .then(
                    self.bot_require_code_grant
                        .cmp(&other.bot_require_code_grant),
                )
                .then(self.verify_key.cmp(&other.verify_key))
                .then(self.redirect_uris.cmp(&other.redirect_uris))
                .then(self.rpc_application_state.cmp(&other.rpc_application_state))
                .then(
                    self.store_application_state
                        .cmp(&other.store_application_state),
                )
                .then(self.verification_state.cmp(&other.verification_state))
                .then(
                    self.interactions_endpoint_url
                        .cmp(&other.interactions_endpoint_url),
                )
                .then(self.integration_public.cmp(&other.integration_public))
                .then(
                    self.integration_require_code_grant
                        .cmp(&other.integration_require_code_grant),
                )
                .then(self.discoverability_state.cmp(&other.discoverability_state))
                .then(
                    self.discovery_eligibility_flags
                        .cmp(&other.discovery_eligibility_flags),
                )
                .then(self.tags.cmp(&other.tags))
                .then(self.cover_image.cmp(&other.cover_image))
                .then(self.terms_of_service_url.cmp(&other.terms_of_service_url))
                .then(self.privacy_policy_url.cmp(&other.privacy_policy_url)),
        )
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(feature = "sqlx")]
fn compare_install_params(
    a: &Option<sqlx::types::Json<InstallParams>>,
    b: &Option<sqlx::types::Json<InstallParams>>,
) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => match (a.encode_to_string(), b.encode_to_string()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        },
        (None, None) => true,
        _ => false,
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(not(feature = "sqlx"))]
fn compare_install_params(
    a: &Option<Shared<InstallParams>>,
    b: &Option<Shared<InstallParams>>,
) -> bool {
    option_arc_rwlock_ptr_eq(a, b)
}

impl Default for Application {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: "".to_string(),
            icon: None,
            description: None,
            summary: None,
            r#type: None,
            hook: true,
            bot_public: true,
            bot_require_code_grant: false,
            verify_key: "".to_string(),
            owner: Default::default(),
            flags: ApplicationFlags::empty(),
            redirect_uris: None,
            rpc_application_state: 0,
            store_application_state: 1,
            verification_state: 1,
            interactions_endpoint_url: None,
            integration_public: true,
            integration_require_code_grant: false,
            discoverability_state: 1,
            discovery_eligibility_flags: 2240,
            tags: None,
            cover_image: None,
            install_params: None,
            terms_of_service_url: None,
            privacy_policy_url: None,
            team: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// # Reference
/// See <https://discord.com/developers/docs/resources/application#install-params-object>
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/application#application-object-application-flags>
    pub struct ApplicationFlags: u64 {
        /// Indicates if an app uses the Auto Moderation API
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        /// Intent required for bots in 100 or more servers to receive presence_update events
        const GATEWAY_PRESENCE = 1 << 12;
        /// Intent required for bots in under 100 servers to receive presence_update events, found on the Bot page in your app's settings on discord.com
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        /// Intent required for bots in 100 or more servers to receive member-related events like guild_member_add.
        /// See the list of member-related events under GUILD_MEMBERS
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        /// Intent required for bots in under 100 servers to receive member-related events like guild_member_add, found on the Bot page in your app's settings on discord.com.
        /// See the list of member-related events under GUILD_MEMBERS
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        /// Indicates unusual growth of an app that prevents verification
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        /// Indicates if an app is embedded within the Discord client (currently unavailable publicly)
        const EMBEDDED = 1 << 17;
        /// Intent required for bots in 100 or more servers to receive message content
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        /// Intent required for bots in under 100 servers to receive message content, found on the Bot page in your app's settings on discord.com
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        /// Indicates if an app has registered slash commands
        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// # Reference
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-object>
pub struct ApplicationCommand {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub name: String,
    pub description: String,
    pub options: Vec<Shared<ApplicationCommandOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reference
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure>
pub struct ApplicationCommandOption {
    pub r#type: ApplicationCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub choices: Vec<ApplicationCommandOptionChoice>,
    pub options: Shared<Vec<ApplicationCommandOption>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
/// # Reference
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types>
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    /// Any integer between -2^53 and 2^53
    Integer = 4,
    Boolean = 5,
    User = 6,
    /// Includes all channel types + categories
    Channel = 7,
    Role = 8,
    /// Includes users and roles
    Mentionable = 9,
    /// Any double between -2^53 and 2^53
    Number = 10,
    Attachment = 11,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionData {
    pub id: Snowflake,
    pub name: String,
    pub options: Vec<Shared<ApplicationCommandInteractionDataOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    pub value: Value,
    pub options: Vec<Shared<ApplicationCommandInteractionDataOption>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-guild-application-command-permissions-structure>
pub struct GuildApplicationCommandPermissions {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub guild_id: Snowflake,
    pub permissions: Vec<Shared<ApplicationCommandPermission>>,
}

#[derive(
    Debug, Default, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash, PartialOrd, Ord,
)]
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permissions-structure>
pub struct ApplicationCommandPermission {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub permission_type: ApplicationCommandPermissionType,
    /// true to allow, false, to disallow
    pub permission: bool,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permission-type>
pub enum ApplicationCommandPermissionType {
    #[default]
    Role = 1,
    User = 2,
    Channel = 3,
}
