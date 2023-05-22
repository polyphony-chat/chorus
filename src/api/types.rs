/*
To learn more about the types implemented here, please visit
https://discord.com/developers/docs .
I do not feel like re-documenting all of this, as everything is already perfectly explained there.
*/

use std::{cell::RefCell, rc::Rc, collections::HashMap};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_aux::{field_attributes::deserialize_option_number_from_string, prelude::{deserialize_string_from_number, deserialize_number_from_string}};

use crate::{api::limits::Limits, instance::Instance};

pub trait WebSocketEvent {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub token: String,
    pub settings: UserSettings,
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
    //view_nsfw_guilds: bool,
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UnavailableGuild {
    id: String,
    unavailable: Option<bool>,
}

/// See https://discord.com/developers/docs/resources/guild
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Guild {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<UserObject>,
    pub owner_id: Option<String>,
    pub permissions: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: Option<u8>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub widget_channel: Option<Channel>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    pub roles: Vec<RoleObject>,
    pub emojis: Vec<Emoji>,
    pub features: Option<Vec<String>>,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: Option<u8>,
    pub rules_channel_id: Option<String>,
    pub rules_channel: Option<String>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: Option<u8>,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: Option<String>,
    pub public_updates_channel_id: Option<String>,
    pub public_updates_channel: Option<Channel>,
    pub max_video_channel_users: Option<u8>,
    pub max_stage_video_channel_users: Option<u8>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub member_count: Option<u64>,
    pub presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub nsfw_level: Option<u8>,
    pub nsfw: Option<bool>,
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: Option<bool>,
    pub joined_at: String,
    pub afk_channel: Option<Channel>,
    pub bans: Option<Vec<GuildBan>>,
    pub primary_category_id: Option<String>,
    pub large: Option<bool>,
    pub channels: Option<Vec<Channel>>,
    pub template_id: Option<String>,
    pub template: Option<GuildTemplate>,
    pub invites: Option<Vec<GuildInvite>>,
    pub voice_states: Option<Vec<VoiceState>>,
    pub webhooks: Option<Vec<Webhook>>,
    pub mfa_level: Option<u8>,
    pub region: Option<String>,
    pub unavailable: Option<bool>,
    pub parent: Option<String>,
}
/// See https://docs.spacebar.chat/routes/#get-/guilds/-guild_id-/bans/-user-
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildBan {
    pub id: String,
    pub user_id: String,
    pub guild_id: String,
    pub executor_id: String,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WelcomeScreenObject {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WelcomeScreenChannel {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object
pub struct GuildScheduledEvent {
    pub id: String,
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub creator_id: Option<String>,
    pub name: String,
    pub description: String,
    pub scheduled_start_time: DateTime<Utc>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    pub status: GuildScheduledEventStatus,
    pub entity_type: GuildScheduledEventEntityType,
    pub entity_id: Option<String>,
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,
    pub creator: Option<UserObject>,
    pub user_count: Option<u64>,
    pub image: Option<String>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
/// See https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-privacy-level
pub enum GuildScheduledEventPrivacyLevel {
    #[default]
    GuildOnly = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
/// See https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-status
pub enum GuildScheduledEventStatus {
    #[default]
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone)]
#[repr(u8)]
/// See https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-types
pub enum GuildScheduledEventEntityType {
    #[default]
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-entity-metadata
pub struct GuildScheduledEventEntityMetadata {
    pub location: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object
pub struct AuditLogEntry {
    pub target_id: Option<String>,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: Option<String>,
    pub id: String,
    // to:do implement an enum for these types
    pub action_type: u8,
    // to:do add better options type
    pub options: Option<serde_json::Value>,
    pub reason: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/resources/audit-log#audit-log-change-object
pub struct AuditLogChange {
    pub new_value: Option<serde_json::Value>,
    pub old_value: Option<serde_json::Value>,
    pub key: String
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/topics/permissions#role-object
pub struct RoleObject {
    pub id: String,
    pub name: String,
    pub color: f64,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: u16,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
/// See https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
pub struct RoleTags {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub bot_id: Option<usize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub integration_id: Option<usize>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub subscription_listing_id: Option<usize>,
    // These use the bad bool format, "Tags with type null represent booleans. They will be present and set to null if they are "true", and will be not present if they are "false"."
    // premium_subscriber: bool,
    // available_for_purchase: bool,
    // guild_connections: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct UserObject {
    pub id: String,
    username: Option<String>,
    discriminator: Option<String>,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    accent_color: Option<String>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    /// This field comes as either a string or a number as a string
    /// So we need to account for that
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    flags: Option<i32>,
    premium_since: Option<String>,
    premium_type: Option<i8>,
    pronouns: Option<String>,
    public_flags: Option<i32>,
    banner: Option<String>,
    bio: Option<String>,
    theme_colors: Option<Vec<i32>>,
    phone: Option<String>,
    nsfw_allowed: Option<bool>,
    premium: Option<bool>,
    purchased_flags: Option<i32>,
    premium_usage_flags: Option<i32>,
    disabled: Option<bool>,
}

#[derive(Debug)]
pub struct User {
    pub belongs_to: Rc<RefCell<Instance>>,
    pub token: String,
    pub limits: Limits,
    pub settings: UserSettings,
    pub object: Option<UserObject>,
}

impl User {
    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn new(
        belongs_to: Rc<RefCell<Instance>>,
        token: String,
        limits: Limits,
        settings: UserSettings,
        object: Option<UserObject>,
    ) -> User {
        User {
            belongs_to,
            token,
            limits,
            settings,
            object,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    id: String,
    pub channel_id: String,
    author: UserObject,
    content: String,
    timestamp: String,
    edited_timestamp: Option<String>,
    tts: bool,
    mention_everyone: bool,
    mentions: Option<Vec<UserObject>>,
    mention_roles: Vec<String>,
    mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<DiscordFileAttachment>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<serde_json::Value>,
    pinned: bool,
    webhook_id: Option<String>,
    #[serde(rename = "type")]
    message_type: i32,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<String>,
    message_reference: Option<MessageReference>,
    flags: Option<i32>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
    components: Option<Vec<Component>>,
    sticker_items: Option<Vec<StickerItem>>,
    stickers: Option<Vec<Sticker>>,
    position: Option<i32>,
    role_subscription_data: Option<RoleSubscriptionData>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#message-create
pub struct MessageCreate {
    #[serde(flatten)]
    message: Message,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Option<Vec<MessageCreateUser>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#message-create-message-create-extra-fields
pub struct MessageCreateUser {
    pub id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    accent_color: Option<String>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    premium_since: Option<String>,
    premium_type: Option<i8>,
    pronouns: Option<String>,
    public_flags: Option<i32>,
    banner: Option<String>,
    bio: Option<String>,
    theme_colors: Option<Vec<i32>>,
    phone: Option<String>,
    nsfw_allowed: Option<bool>,
    premium: Option<bool>,
    purchased_flags: Option<i32>,
    premium_usage_flags: Option<i32>,
    disabled: Option<bool>,
    member: GuildMember
}

impl WebSocketEvent for MessageCreate {}

#[derive(Debug, Serialize, Deserialize, Default)]
struct PartialMessage {
    id: Option<String>,
    channel_id: Option<String>,
    author: Option<UserObject>,
    content: Option<String>,
    timestamp: Option<String>,
    edited_timestamp: Option<String>,
    tts: Option<bool>,
    mention_everyone: Option<bool>,
    mentions: Option<Vec<UserObject>>,
    mention_roles: Option<Vec<String>>,
    mention_channels: Option<Vec<ChannelMention>>,
    attachments: Option<Vec<DiscordFileAttachment>>,
    embeds: Option<Vec<Embed>>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<serde_json::Value>,
    pinned: Option<bool>,
    webhook_id: Option<String>,
    #[serde(rename = "type")]
    message_type: Option<i32>,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<String>,
    message_reference: Option<MessageReference>,
    flags: Option<i32>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<MessageInteraction>,
    thread: Option<Channel>,
    components: Option<Vec<Component>>,
    sticker_items: Option<Vec<StickerItem>>,
    stickers: Option<Vec<Sticker>>,
    position: Option<i32>,
    role_subscription_data: Option<RoleSubscriptionData>,
    guild_id: Option<String>,
    member: Option<GuildMember>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageUpdate {
    #[serde(flatten)]
    message: PartialMessage,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Option<Vec<(UserObject, GuildMember)>>, // Not sure if this is correct: https://discord.com/developers/docs/topics/gateway-events#message-create
}

impl WebSocketEvent for MessageUpdate {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageDelete {
    id: String,
    channel_id: String,
    guild_id: Option<String>,
}

impl WebSocketEvent for MessageDelete {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageDeleteBulk {
    ids: Vec<String>,
    channel_id: String,
    guild_id: Option<String>,
}

impl WebSocketEvent for MessageDeleteBulk {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionAdd {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    emoji: Emoji,
}

impl WebSocketEvent for MessageReactionAdd {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionRemove {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}

impl WebSocketEvent for MessageReactionRemove {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionRemoveAll {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
}

impl WebSocketEvent for MessageReactionRemoveAll {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageReactionRemoveEmoji {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}

impl WebSocketEvent for MessageReactionRemoveEmoji {}

#[derive(Debug, Serialize, Deserialize)]
struct ChannelMention {
    id: String,
    guild_id: String,
    #[serde(rename = "type")]
    channel_type: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
/**
Represents an Embed. [See the Discord Documentation](https://discord.com/developers/docs/resources/channel#embed-object).
 */
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<i32>,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedImage {
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedThumbnail {
    url: String,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]

struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub count: i32,
    pub me: bool,
    pub emoji: Emoji,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Emoji {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u64>,
    pub name: Option<String>,
    pub roles: Option<Vec<u64>>,
    pub user: Option<UserObject>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: i64,
    pub party_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<UserObject>,
    pub summary: String,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<String>,
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<InstallParams>,
    pub custom_install_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Team {
    pub icon: Option<String>,
    pub id: u64,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamMember {
    pub membership_state: u8,
    pub permissions: Vec<String>,
    pub team_id: u64,
    pub user: UserObject,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MembershipState {
    Invited = 1,
    Accepted = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageReference {
    pub message_id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageInteraction {
    pub id: u64,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub name: String,
    pub user: UserObject,
    pub member: Option<GuildMember>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GuildMember {
    pub user: Option<UserObject>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: Option<i32>,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: i32,
    pub guild_id: Option<String>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<UserObject>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<i32>,
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    pub available_tags: Option<Vec<Tag>>,
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<i32>,
    pub default_forum_layout: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub overwrite_type: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub allow: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub deny: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i32,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct ThreadMember {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub join_timestamp: Option<String>,
    pub flags: Option<u64>,
    pub member: Option<GuildMember>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/resources/guild#integration-object-integration-structure
pub struct Integration {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub integration_type: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enabled_emoticons: Option<bool>,
    pub expire_behaviour: Option<u8>,
    pub expire_grace_period: Option<u16>,
    pub user: Option<UserObject>,
    pub account: IntegrationAccount,
    pub synced_at: Option<DateTime<Utc>>,
    pub subscriber_count: Option<f64>,
    pub revoked: Option<bool>,
    pub application: Option<Application>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/resources/guild#integration-account-object-integration-account-structure
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/resources/stage-instance#stage-instance-object
pub struct StageInstance {
    pub id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub topic: String,
    pub privacy_level: u8,
    pub discoverable_disabled: bool,
    pub guild_scheduled_event_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultReaction {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Component {
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    TextInput = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerItem {
    pub id: u64,
    pub name: String,
    pub format_type: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sticker {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub pack_id: Option<u64>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: Option<String>,
    #[serde(rename = "type")]
    pub sticker_type: u8,
    pub format_type: u8,
    pub available: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub guild_id: Option<u64>,
    pub user: Option<UserObject>,
    pub sort_value: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: u64,
    pub tier_name: String,
    pub total_months_subscribed: u32,
    pub is_renewal: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TypingStartEvent {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub user_id: String,
    pub timestamp: i64,
    pub member: Option<GuildMember>,
}

impl WebSocketEvent for TypingStartEvent {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayIdentifyPayload {
    pub token: String,
    pub properties: GatewayIdentifyConnectionProps,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<i16>, //default: 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<(i32, i32)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceUpdate>,
    pub capabilities: i32,
}

impl GatewayIdentifyPayload {
    /// Creates an identify payload with the same default capabilities as the official client
    pub fn default_w_client_capabilities() -> Self {
        let mut def = Self::default();
        def.capabilities = 8189; // Default capabilities for a client
        def
    }

    /// Creates an identify payload with all possible capabilities
    pub fn default_w_all_capabilities() -> Self {
        let mut def = Self::default();
        def.capabilities = i32::MAX; // Since discord uses bitwise for capabilities, this has almost every bit as 1, so all capabilities
        def
    }
}

impl WebSocketEvent for GatewayIdentifyPayload {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayIdentifyConnectionProps {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields
pub struct PresenceUpdate {
    pub user: UserObject,
    pub guild_id: Option<String>,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatusObject,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#client-status-object
pub struct ClientStatusObject {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}

impl WebSocketEvent for PresenceUpdate {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: i32,
    url: Option<String>,
    created_at: i64,
    timestamps: Option<ActivityTimestamps>,
    application_id: Option<String>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<ActivityParty>,
    assets: Option<ActivityAssets>,
    secrets: Option<ActivitySecrets>,
    instance: Option<bool>,
    flags: Option<i32>,
    buttons: Option<Vec<ActivityButton>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ActivityTimestamps {
    start: Option<i64>,
    end: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ActivityParty {
    id: Option<String>,
    size: Option<Vec<(i32, i32)>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    #[serde(rename = "match")]
    match_string: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ActivityButton {
    label: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GatewayResume {
    pub token: String,
    pub session_id: String,
    pub seq: String,
}

impl WebSocketEvent for GatewayResume {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Sort of documented, though most fields are left out
/// For a full example see https://gist.github.com/kozabrada123/a347002b1fb8825a5727e40746d4e199
/// to:do add all undocumented fields
pub struct GatewayReady {
    pub analytics_token: Option<String>,
    pub auth_session_id_hash: Option<String>,
    pub country_code: Option<String>,

    pub v: u8,
    pub user: UserObject,
    /// For bots these are [UnavailableGuild]s, for users they are [Guild]
    pub guilds: Vec<Guild>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub sessions: Option<Vec<Session>>,
    pub session_id: String,
    pub session_type: Option<String>,
    pub resume_gateway_url: Option<String>,
    pub shard: Option<(u64, u64)>,
}

impl WebSocketEvent for GatewayReady {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Sent after the READY event when a client is a user
/// {"t":"READY_SUPPLEMENTAL","s":2,"op":0,"d":{"merged_presences":{"guilds":[[{"user_id":"463640391196082177","status":"online","game":null,"client_status":{"web":"online"},"activities":[]}]],"friends":[{"user_id":"463640391196082177","status":"online","last_modified":1684053508443,"client_status":{"web":"online"},"activities":[]}]},"merged_members":[[{"user_id":"463640391196082177","roles":[],"premium_since":null,"pending":false,"nick":"pog","mute":false,"joined_at":"2021-05-30T15:24:08.763000+00:00","flags":0,"deaf":false,"communication_disabled_until":null,"avatar":null}]],"lazy_private_channels":[],"guilds":[{"voice_states":[],"id":"848582562217590824","embedded_activities":[]}],"disclose":["pomelo"]}}
pub struct GatewayReadySupplemental {
    pub merged_presences: MergedPresences,
    pub merged_members: Vec<Vec<GuildMember>>,
    // ?
    pub lazy_private_channels: Vec<serde_json::Value>,
    pub guilds: Vec<SupplimentalGuild>,
    // ? pomelo
    pub disclose: Vec<String>,
}

impl WebSocketEvent for GatewayReadySupplemental {}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MergedPresences {
    pub guilds: Vec<Vec<MergedPresenceGuild>>,
    pub friends: Vec<MergedPresenceFriend>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MergedPresenceFriend {
    pub user_id: String,
    pub status: String,
    /// Looks like ms??
    pub last_modified: u128,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MergedPresenceGuild {
    pub user_id: String,
    pub status: String,
    // ?
    pub game: Option<serde_json::Value>,
    pub client_status: ClientStatusObject,
    pub activities: Vec<Activity>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SupplimentalGuild {
    pub voice_states: Vec<VoiceState>,
    pub id: String,
    pub embedded_activities: Vec<serde_json::Value>
}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#request-guild-members-request-guild-members-structure
pub struct GatewayRequestGuildMembers {
    pub guild_id: String,
    pub query: Option<String>,
    pub limit: u64,
    pub presences: Option<bool>,
    pub user_ids: Option<String>,
    pub nonce: Option<String>,
}

impl WebSocketEvent for GatewayRequestGuildMembers {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Seems like it sends active session info to users on connect
/// [{"activities":[],"client_info":{"client":"web","os":"other","version":0},"session_id":"ab5941b50d818b1f8d93b4b1b581b192","status":"online"}]
pub struct SessionsReplace {
    pub sessions: Vec<Session>
}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Session info for the current user
pub struct Session {
    pub activities: Vec<Activity>,
    pub client_info: ClientInfo,
    pub session_id: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Another Client info object
/// {"client":"web","os":"other","version":0}
// Note: I don't think this one exists yet? Though I might've made a mistake and this might be a duplicate 
pub struct ClientInfo {
    pub client: String,
    pub os: String,
    pub version: u8
}

impl WebSocketEvent for SessionsReplace {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#update-voice-state-gateway-voice-state-update-structure
pub struct GatewayVoiceStateUpdate {
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

impl WebSocketEvent for GatewayVoiceStateUpdate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#webhooks-update
pub struct WebhooksUpdate {
    pub guild_id: String,
    pub channel_id: String,
}

impl WebSocketEvent for WebhooksUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHello {
    pub op: i32,
    pub d: HelloData,
}

impl WebSocketEvent for GatewayHello {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HelloData {
    pub heartbeat_interval: u128,
}

impl WebSocketEvent for HelloData {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHeartbeat {
    pub op: u8,
    pub d: Option<u64>,
}

impl WebSocketEvent for GatewayHeartbeat {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayHeartbeatAck {
    pub op: i32,
}

impl WebSocketEvent for GatewayHeartbeatAck {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-pins-update
pub struct ChannelPinsUpdate {
    pub guild_id: Option<String>,
    pub channel_id: String,
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}

impl WebSocketEvent for ChannelPinsUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-add-guild-ban-add-event-fields
pub struct GuildBanAdd {
    pub guild_id: String,
    pub user: UserObject,
}

impl WebSocketEvent for GuildBanAdd {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove
pub struct GuildBanRemove {
    pub guild_id: String,
    pub user: UserObject,
}

impl WebSocketEvent for GuildBanRemove {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#user-update
pub struct UserUpdate {
    #[serde(flatten)]
    pub user: UserObject,
}

impl WebSocketEvent for UserUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-create
pub struct ChannelCreate {
    #[serde(flatten)]
    pub channel: Channel,
}

impl WebSocketEvent for ChannelCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-update
pub struct ChannelUpdate {
    #[serde(flatten)]
    pub channel: Channel,
}

impl WebSocketEvent for ChannelUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// Officially undocumented.
/// Sends updates to client about a new message with its id
/// {"channel_unread_updates": [{"id": "816412869766938648", "last_message_id": "1085892012085104680"}}
pub struct ChannelUnreadUpdate {
    pub channel_unread_updates: Vec<ChannelUnreadUpdateObject>,
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
/// Contains very few fields from [Channel]
/// See also [ChannelUnreadUpdates]
pub struct ChannelUnreadUpdateObject {
    pub id: String,
    pub last_message_id: String,
    pub last_pin_timestamp: Option<String>
}

impl WebSocketEvent for ChannelUnreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-delete
pub struct ChannelDelete {
    #[serde(flatten)]
    pub channel: Channel,
}

impl WebSocketEvent for ChannelDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-create
pub struct ThreadCreate {
    #[serde(flatten)]
    pub thread: Channel,
}

impl WebSocketEvent for ThreadCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-update
pub struct ThreadUpdate {
    #[serde(flatten)]
    pub thread: Channel,
}

impl WebSocketEvent for ThreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-delete
pub struct ThreadDelete {
    #[serde(flatten)]
    pub thread: Channel,
}

impl WebSocketEvent for ThreadDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-list-sync
pub struct ThreadListSync {
    pub guild_id: String,
    pub channel_ids: Option<Vec<String>>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>,
}

impl WebSocketEvent for ThreadListSync {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-member-update
/// The inner payload is a thread member object with an extra field.
pub struct ThreadMemberUpdate {
    #[serde(flatten)]
    pub member: ThreadMember,
    pub guild_id: String,
}

impl WebSocketEvent for ThreadMemberUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-members-update
pub struct ThreadMembersUpdate {
    pub id: String,
    pub guild_id: String,
    /// Capped at 50
    pub member_count: u8,
    pub added_members: Option<Vec<ThreadMember>>,
    pub removed_members: Option<Vec<String>>,
}

impl WebSocketEvent for ThreadMembersUpdate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-create
/// This one is particularly painful, it can be a Guild object with extra field or an unavailbile guild object
pub struct GuildCreate {
    #[serde(flatten)]
    pub d: GuildCreateDataOption,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GuildCreateDataOption {
    UnavailableGuild(UnavailableGuild),
    Guild(Guild),
}

impl Default for GuildCreateDataOption {
    fn default() -> Self {
        GuildCreateDataOption::UnavailableGuild(UnavailableGuild::default())
    }
}
impl WebSocketEvent for GuildCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-update
pub struct GuildUpdate {
    #[serde(flatten)]
    pub guild: Guild
}

impl WebSocketEvent for GuildUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-delete
pub struct GuildDelete {
    #[serde(flatten)]
    pub guild: UnavailableGuild
}

impl WebSocketEvent for GuildDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-audit-log-entry-create
pub struct GuildAuditLogEntryCreate {
    #[serde(flatten)]
    pub entry: AuditLogEntry
}

impl WebSocketEvent for GuildAuditLogEntryCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-emojis-update
pub struct GuildEmojisUpdate {
    pub guild_id: String,
    pub emojis: Vec<Emoji>
}

impl WebSocketEvent for GuildEmojisUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-stickers-update
pub struct GuildStickersUpdate {
    pub guild_id: String,
    pub stickers: Vec<Sticker>
}

impl WebSocketEvent for GuildStickersUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-integrations-update
pub struct GuildIntegrationsUpdate {
    pub guild_id: String,
}

impl WebSocketEvent for GuildIntegrationsUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-member-add
pub struct GuildMemberAdd {
    #[serde(flatten)]
    pub member: GuildMember,
    pub guild_id: String,
}

impl WebSocketEvent for GuildMemberAdd {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-member-remove
pub struct GuildMemberRemove {
    pub guild_id: String,
    pub user: UserObject,
}

impl WebSocketEvent for GuildMemberRemove {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-member-update
pub struct GuildMemberUpdate {
    pub guild_id: String,
    pub roles: Vec<String>,
    pub user: UserObject,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<DateTime<Utc>>,
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub communication_disabled_until: Option<DateTime<Utc>>,
}

impl WebSocketEvent for GuildMemberUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-members-chunk
pub struct GuildMembersChunk {
    pub guild_id: String,
    pub members: Vec<GuildMember>,
    pub chunk_index: u16,
    pub chunk_count: u16,
    pub not_found: Option<Vec<String>>,
    pub presences: Option<PresenceUpdate>,
    pub nonce: Option<String>
}

impl WebSocketEvent for GuildMembersChunk {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-role-create
pub struct GuildRoleCreate {
    pub guild_id: String,
    pub role: RoleObject,
}

impl WebSocketEvent for GuildRoleCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-role-update
pub struct GuildRoleUpdate {
    pub guild_id: String,
    pub role: RoleObject,
}

impl WebSocketEvent for GuildRoleUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-role-delete
pub struct GuildRoleDelete {
    pub guild_id: String,
    pub role_id: String,
}

impl WebSocketEvent for GuildRoleDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-create
pub struct GuildScheduledEventCreate {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

impl WebSocketEvent for GuildScheduledEventCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-update
pub struct GuildScheduledEventUpdate {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

impl WebSocketEvent for GuildScheduledEventUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-delete
pub struct GuildScheduledEventDelete {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

impl WebSocketEvent for GuildScheduledEventDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-add
pub struct GuildScheduledEventUserAdd {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}

impl WebSocketEvent for GuildScheduledEventUserAdd {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-remove
pub struct GuildScheduledEventUserRemove {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}

impl WebSocketEvent for GuildScheduledEventUserRemove {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// 
/// Seems to be passively set to update the client on guild details (though, why not just send the update events?)
pub struct PassiveUpdateV1 {
    pub voice_states: Vec<VoiceState>,
    pub members: Vec<GuildMember>,
    pub guild_id: String,
    pub channels: Vec<ChannelUnreadUpdateObject>,
}

impl WebSocketEvent for PassiveUpdateV1 {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#integration-create
pub struct IntegrationCreate {
    #[serde(flatten)]
    pub integration: Integration,
    pub guild_id: String,
}

impl WebSocketEvent for IntegrationCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#integration-update
pub struct IntegrationUpdate {
    #[serde(flatten)]
    pub integration: Integration,
    pub guild_id: String,
}

impl WebSocketEvent for IntegrationUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#integration-delete
pub struct IntegrationDelete {
    pub id: String,
    pub guild_id: String,
    pub application_id: Option<String>,
}

impl WebSocketEvent for IntegrationDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#invite-create
pub struct InviteCreate {
    #[serde(flatten)]
    pub invite: GuildInvite
}

impl WebSocketEvent for InviteCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#invite-delete
pub struct InviteDelete {
    pub channel_id: String,
    pub guild_id: Option<String>,
    pub code: String,
}

impl WebSocketEvent for InviteDelete {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Is sent to a client by the server to signify a new being created
/// {"t":"CALL_CREATE","s":2,"op":0,"d":{"voice_states":[],"ringing":[],"region":"milan","message_id":"1107187514906775613","embedded_activities":[],"channel_id":"837609115475771392"}}
pub struct CallCreate {
    pub voice_states: Vec<VoiceState>,
    /// Seems like a vec of channel ids
    pub ringing: Vec<String>,
    pub region: String, // milan
    pub message_id: String,
    /// What is this?
    pub embedded_activities: Vec<serde_json::Value>,
    pub channel_id: String,
}
impl WebSocketEvent for CallCreate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Updates the status of calls
/// {"t":"CALL_UPDATE","s":5,"op":0,"d":{"ringing":["837606544539254834"],"region":"milan","message_id":"1107191540234846308","guild_id":null,"channel_id":"837609115475771392"}}
pub struct CallUpdate {
    /// Seems like a vec of channel ids
    pub ringing: Vec<String>,
    pub region: String, // milan
    pub message_id: String,
    pub guild_id: Option<String>,
    pub channel_id: String,
}
impl WebSocketEvent for CallUpdate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// Deletes a ringing call
/// {"t":"CALL_DELETE","s":8,"op":0,"d":{"channel_id":"837609115475771392"}}
pub struct CallDelete {
    pub channel_id: String,
}
impl WebSocketEvent for CallDelete {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// See https://unofficial-discord-docs.vercel.app/gateway/op13
/// {"op":13,"d":{"channel_id":"837609115475771392"}}
pub struct CallSync {
    pub channel_id: String,
}
impl WebSocketEvent for CallSync {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// 
/// Sent to the server to signify lazy loading of a guild;
/// Sent by the official client when switching to a guild or channel;
/// After this, you should recieve message updates
/// 
/// See https://luna.gitlab.io/discord-unofficial-docs/lazy_guilds.html#op-14-lazy-request
/// 
/// {"op":14,"d":{"guild_id":"848582562217590824","typing":true,"activities":true,"threads":true}}
pub struct LazyRequest {
    pub guild_id: String,
    pub typing: bool,
    pub activities: bool,
    pub threads: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<HashMap<String, Vec<Vec<u64>>>>
}
impl WebSocketEvent for LazyRequest {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// Officially Undocumented
/// 
/// Not documented anywhere unofficially
/// 
/// Apparently "Message ACK refers to marking a message as read for Discord's API." (https://github.com/Rapptz/discord.py/issues/1851)
/// I suspect this is sent and recieved from the gateway to let clients on other devices know the user has read a message
/// 
/// {"t":"MESSAGE_ACK","s":3,"op":0,"d":{"version":52,"message_id":"1107236673638633472","last_viewed":null,"flags":null,"channel_id":"967363950217936897"}}
pub struct MessageACK {
    /// ?
    pub version: u16,
    pub message_id: String,
    pub last_viewed: Option<DateTime<Utc>>,
    /// What flags?
    pub flags: Option<serde_json::Value>,
    pub channel_id: String,
}
impl WebSocketEvent for MessageACK {}

#[derive(Debug, Default, Serialize, Clone)]
/// The payload used for sending events to the gateway
/// 
/// Similar to [GatewayReceivePayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to send the event name
pub struct GatewaySendPayload {
    pub op: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
}

impl WebSocketEvent for GatewaySendPayload {}

#[derive(Debug, Default, Deserialize, Clone)]
/// The payload used for receiving events from the gateway
/// 
/// Similar to [GatewaySendPayload], except we send a [Value] for d whilst we receive a [serde_json::value::RawValue]
/// Also, we never need to sent the event name

pub struct GatewayReceivePayload<'a> {
    pub op: u8,
    #[serde(borrow)]
    pub d: Option<&'a serde_json::value::RawValue>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

impl<'a> WebSocketEvent for GatewayReceivePayload<'a> {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordFileAttachment {
    pub id: i16,
    pub filename: String,
    description: Option<String>,
    content_type: Option<String>,
    size: i64,
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
    ephemeral: Option<bool>,
    duration_secs: Option<f32>,
    waveform: Option<String>,
    #[serde(skip_serializing)]
    content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PartialDiscordFileAttachment {
    pub id: Option<i16>,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<i64>,
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f32>,
    pub waveform: Option<String>,
    #[serde(skip_serializing)]
    pub content: Vec<u8>,
}

impl PartialDiscordFileAttachment {
    /**
    Moves `self.content` out of `self` and returns it.
    # Returns
    Vec<u8>
     */
    pub fn move_content(self) -> (Vec<u8>, PartialDiscordFileAttachment) {
        let content = self.content;
        let updated_struct = PartialDiscordFileAttachment {
            id: self.id,
            filename: self.filename,
            description: self.description,
            content_type: self.content_type,
            size: self.size,
            url: self.url,
            proxy_url: self.proxy_url,
            height: self.height,
            width: self.width,
            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform,
            content: Vec::new(),
        };
        (content, updated_struct)
    }

    pub fn move_filename(self) -> (String, PartialDiscordFileAttachment) {
        let filename = self.filename;
        let updated_struct = PartialDiscordFileAttachment {
            id: self.id,
            filename: String::new(),
            description: self.description,
            content_type: self.content_type,
            size: self.size,
            url: self.url,
            proxy_url: self.proxy_url,
            height: self.height,
            width: self.width,

            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform,
            content: self.content,
        };
        (filename, updated_struct)
    }

    pub fn move_content_type(self) -> (Option<String>, PartialDiscordFileAttachment) {
        let content_type = self.content_type;
        let updated_struct = PartialDiscordFileAttachment {
            id: self.id,
            filename: self.filename,
            description: self.description,
            content_type: None,
            size: self.size,
            url: self.url,
            proxy_url: self.proxy_url,
            height: self.height,
            width: self.width,
            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform,
            content: self.content,
        };
        (content_type, updated_struct)
    }

    pub fn set_id(&mut self, id: i16) {
        self.id = Some(id);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedMention {
    parse: Vec<AllowedMentionType>,
    roles: Vec<String>,
    users: Vec<String>,
    replied_user: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedMentionType {
    Roles,
    Users,
    Everyone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

/// See https://docs.spacebar.chat/routes/#cmp--schemas-template
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildTemplate {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub usage_count: Option<u64>,
    pub creator_id: String,
    pub creator: UserObject,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source_guild_id: String,
    pub source_guild: Vec<Guild>, // Unsure how a {recursive: Guild} looks like, might be a Vec?
    pub serialized_source_guild: Vec<Guild>,
    id: String,
}

/// See https://docs.spacebar.chat/routes/#cmp--schemas-invite
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildInvite {
    pub code: String,
    pub temporary: Option<bool>,
    pub uses: Option<i32>,
    pub max_uses: Option<i32>,
    pub max_age: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub guild_id: String,
    pub guild: Option<Guild>,
    pub channel_id: String,
    pub channel: Option<Channel>,
    pub inviter_id: Option<String>,
    pub inviter: Option<UserObject>,
    pub target_user_id: Option<String>,
    pub target_user: Option<String>,
    pub target_user_type: Option<i32>,
    pub vanity_url: Option<bool>,
}

/// See https://docs.spacebar.chat/routes/#cmp--schemas-voicestate
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VoiceState {
    pub guild_id: Option<String>,
    pub guild: Option<Guild>,
    pub channel_id: String,
    pub channel: Option<Channel>,
    pub user_id: String,
    pub user: Option<UserObject>,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub token: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
    pub id: Option<String>,
}

/// See https://docs.spacebar.chat/routes/#cmp--schemas-webhook
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Webhook {
    #[serde(rename = "type")]
    pub webhook_type: i32,
    pub name: String,
    pub avatar: String,
    pub token: String,
    pub guild_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<Guild>,
    pub channel_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    pub application_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserObject>,
    pub source_guild_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guild: Option<Guild>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildCreateResponse {
    pub id: String,
}
