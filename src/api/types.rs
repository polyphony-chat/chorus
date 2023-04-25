use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{api::limits::Limits, URLBundle};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    token: String,
    settings: UserSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    afk_timeout: i32,
    allow_accessibility_detection: bool,
    animate_emoji: bool,
    animate_stickers: i32,
    contact_sync_enabled: bool,
    convert_emoticons: bool,
    custom_status: Option<String>,
    default_guilds_restricted: bool,
    detect_platform_accounts: bool,
    developer_mode: bool,
    disable_games_tab: bool,
    enable_tts_command: bool,
    explicit_content_filter: i32,
    friend_source_flags: FriendSourceFlags,
    friend_discovery_flags: Option<i32>,
    gateway_connected: bool,
    gif_auto_play: bool,
    guild_folders: Vec<GuildFolder>,
    guild_positions: Vec<i64>,
    inline_attachment_media: bool,
    inline_embed_media: bool,
    locale: String,
    message_display_compact: bool,
    native_phone_integration_enabled: bool,
    render_embeds: bool,
    render_reactions: bool,
    restricted_guilds: Vec<i64>,
    show_current_game: bool,
    status: String,
    stream_notifications_enabled: bool,
    theme: String,
    timezone_offset: i32,
    view_nsfw_guilds: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendSourceFlags {
    all: Option<bool>,
    mutual_friends: Option<bool>,
    mutual_guilds: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildFolder {
    id: String,
    guild_ids: Vec<i64>,
    name: String,
}

/**
Represents the result you get from GET: /api/instance/policies/.
*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InstancePolicies {
    instance_name: String,
    instance_description: Option<String>,
    front_page: Option<String>,
    tos_page: Option<String>,
    correspondence_email: Option<String>,
    correspondence_user_id: Option<String>,
    image: Option<String>,
    instance_id: Option<String>,
}

impl InstancePolicies {
    pub fn new(
        instance_name: String,
        instance_description: Option<String>,
        front_page: Option<String>,
        tos_page: Option<String>,
        correspondence_email: Option<String>,
        correspondence_user_id: Option<String>,
        image: Option<String>,
        instance_id: Option<String>,
    ) -> Self {
        InstancePolicies {
            instance_name,
            instance_description,
            front_page,
            tos_page,
            correspondence_email,
            correspondence_user_id,
            image,
            instance_id,
        }
    }
}

impl fmt::Display for InstancePolicies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
                f,
                "InstancePoliciesSchema {{ instance_name: {}, instance_description: {}, front_page: {}, tos_page: {}, correspondence_email: {}, correspondence_user_id: {}, image: {}, instance_id: {} }}",
                self.instance_name,
                self.instance_description.clone().unwrap_or("None".to_string()),
                self.front_page.clone().unwrap_or("None".to_string()),
                self.tos_page.clone().unwrap_or("None".to_string()),
                self.correspondence_email.clone().unwrap_or("None".to_string()),
                self.correspondence_user_id.clone().unwrap_or("None".to_string()),
                self.image.clone().unwrap_or("None".to_string()),
                self.instance_id.clone().unwrap_or("None".to_string()),
            )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
    pub errors: IntermittentError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntermittentError {
    #[serde(flatten)]
    pub errors: std::collections::HashMap<String, ErrorField>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ErrorField {
    #[serde(default)]
    pub _errors: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub message: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserObject {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Option<bool>,
    accent_color: Option<String>,
    locale: String,
    verified: Option<bool>,
    email: Option<String>,
    flags: i8,
    premium_type: Option<i8>,
    public_flags: Option<i8>,
}

#[derive(Debug)]
pub struct User {
    logged_in: bool,
    belongs_to: URLBundle,
    token: String,
    rate_limits: Limits,
    pub settings: UserSettings,
    pub object: UserObject,
}

impl User {
    pub fn is_logged_in(&self) -> bool {
        self.logged_in
    }

    pub fn belongs_to(&self) -> URLBundle {
        self.belongs_to.clone()
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_logged_in(&mut self, bool: bool) {
        self.logged_in = bool;
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn new(
        logged_in: bool,
        belongs_to: URLBundle,
        token: String,
        rate_limits: Limits,
        settings: UserSettings,
        object: UserObject,
    ) -> User {
        User {
            logged_in,
            belongs_to,
            token,
            rate_limits,
            settings,
            object,
        }
    }
}
