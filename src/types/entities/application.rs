use crate::types::utils::Snowflake;
use crate::types::{Team, User};
use bitflags::{bitflags, Flags};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
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
    pub owner: User,
    pub flags: u64,
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
    pub install_params: Option<InstallParams>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub team: Option<Team>,
}

impl Application {
    pub fn flags(&self) -> ApplicationFlags {
        ApplicationFlags::from_bits(self.flags.to_owned()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    pub struct ApplicationFlags: u64 {
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        const GATEWAY_PRESENCE = 1 << 12;
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        const EMBEDDED = 1 << 17;
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub name: String,
    pub description: String,
    pub options: Vec<ApplicationCommandOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    pub r#type: ApplicationCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub choices: Vec<ApplicationCommandOptionChoice>,
    pub options: Vec<ApplicationCommandOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ApplicationCommandOptionType {
    #[serde(rename = "SUB_COMMAND")]
    SubCommand = 1,
    #[serde(rename = "SUB_COMMAND_GROUP")]
    SubCommandGroup = 2,
    #[serde(rename = "STRING")]
    String = 3,
    #[serde(rename = "INTEGER")]
    Integer = 4,
    #[serde(rename = "BOOLEAN")]
    Boolean = 5,
    #[serde(rename = "USER")]
    User = 6,
    #[serde(rename = "CHANNEL")]
    Channel = 7,
    #[serde(rename = "ROLE")]
    Role = 8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionData {
    pub id: Snowflake,
    pub name: String,
    pub options: Vec<ApplicationCommandInteractionDataOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    pub value: Value,
    pub options: Vec<ApplicationCommandInteractionDataOption>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
/// See https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-guild-application-command-permissions-structure
pub struct GuildApplicationCommandPermissions {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub guild_id: Snowflake,
    pub permissions: Vec<ApplicationCommandPermission>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
/// See https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permissions-structure
pub struct ApplicationCommandPermission {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub permission_type: ApplicationCommandPermissionType,
    /// true to allow, false, to disallow
    pub permission: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")] 
#[repr(u8)]
/// See https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permission-type
pub enum ApplicationCommandPermissionType {
    #[default]
    Role = 1,
    User = 2,
    Channel = 3
}