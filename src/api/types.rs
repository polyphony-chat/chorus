/*
To learn more about the types implemented here, please visit
https://discord.com/developers/docs .
I do not feel like re-documenting all of this, as everything is already perfectly explained there.
*/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_value;

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
    unavailable: bool
}

/// See https://discord.com/developers/docs/resources/guild
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u8,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: u8,
    pub default_message_notifications: u8,
    pub explicit_content_filter: u8,
    pub roles: Vec<RoleObject>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub mfa_level: u8,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: u8,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: u8,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<u8>,
    pub max_stage_video_channel_users: Option<u8>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub nsfw_level: u8,
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: bool
}

/// See https://discord.com/developers/docs/topics/gateway-events#guild-create-guild-create-extra-fields
/// This is like [Guild], expect it has extra fields
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GuildCreateGuild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u8,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: u8,
    pub default_message_notifications: u8,
    pub explicit_content_filter: u8,
    pub roles: Vec<RoleObject>,
    pub emojis: Vec<Emoji>,
    pub features: Vec<String>,
    pub mfa_level: u8,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: u8,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: u8,
    pub premium_subscription_count: Option<u64>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<u8>,
    pub max_stage_video_channel_users: Option<u8>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub welcome_screen: Option<WelcomeScreenObject>,
    pub nsfw_level: u8,
    pub stickers: Option<Vec<Sticker>>,
    pub premium_progress_bar_enabled: bool,
    // ------ Extra Fields ------
    pub joined_at: DateTime<Utc>,
    pub large: bool,
    pub unavailable: Option<bool>,
    pub member_count: u64,
    // to:do implement voice states
    //pub voice_states: Vec<VoiceState>,
    pub members: Vec<GuildMember>,
    pub channels: Vec<Channel>,
    pub threads: Vec<Channel>,
    pub presences: Vec<PresenceUpdate>,
    // to:do add stage instances
    //pub stage_instances: Vec<StageInstance>,
    // to:do add guild schedules events
    //pub guild_scheduled_events: Vec<GuildScheduledEvent>
}

impl GuildCreateGuild {
    /// Converts self to a [Guild], discarding the extra fields
    pub fn to_guild(&self) -> Guild {
        let as_value = serde_json::to_value(&self).unwrap();
        return from_value(as_value).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WelcomeScreenObject {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WelcomeScreenChannel {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>
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
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    // to:do add role tags https://discord.com/developers/docs/topics/permissions#role-object-role-tags-structure
    //pub tags: Option<RoleTags>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct UserObject {
    pub id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: bool,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    accent_color: Option<String>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: String,
    premium_since: Option<String>,
    premium_type: i8,
    pronouns: Option<String>,
    public_flags: Option<i8>,
    banner: Option<String>,
    bio: String,
    theme_colors: Option<Vec<i32>>,
    phone: Option<String>,
    nsfw_allowed: bool,
    premium: bool,
    purchased_flags: i32,
    premium_usage_flags: i32,
    disabled: Option<bool>,
}

#[derive(Debug)]
pub struct User<'a> {
    pub belongs_to: &'a mut Instance,
    pub token: String,
    pub limits: Limits,
    pub settings: UserSettings,
    pub object: Option<UserObject>,
}

impl<'a> User<'a> {
    pub fn belongs_to(&mut self) -> &mut Instance {
        self.belongs_to
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn new(
        belongs_to: &'a mut Instance,
        token: String,
        limits: Limits,
        settings: UserSettings,
        object: Option<UserObject>,
    ) -> User<'a> {
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
    mentions: Vec<UserObject>,
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
pub struct MessageCreate {
    #[serde(flatten)]
    message: Message,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Vec<(UserObject, GuildMember)>, // Not sure if this is correct: https://discord.com/developers/docs/topics/gateway-events#message-create
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
    mentions: Vec<(UserObject, GuildMember)>, // Not sure if this is correct: https://discord.com/developers/docs/topics/gateway-events#message-create
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GuildMember {
    pub user: Option<UserObject>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: i32,
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
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionOverwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub overwrite_type: u8,
    pub allow: String,
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
    pub scopes: Option<Vec<String>>
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/resources/guild#integration-account-object-integration-account-structure
pub struct IntegrationAccount {
    pub id: String,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DefaultReaction {
    pub emoji_id: Option<String>,
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
    pub compress: Option<bool>,
    pub large_threshold: Option<i16>, //default: 50
    pub shard: Option<Vec<(i32, i32)>>,
    pub presence: Option<PresenceUpdate>,
    pub intents: i32,
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
    pub guild_id: String,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatusObject
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#client-status-object
pub struct ClientStatusObject {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>
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
pub struct GatewayReady {
    pub v: u8,
    pub user: UserObject,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    pub resume_gateway_url: Option<String>,
    pub shard: Option<(u64, u64)>,
}

impl WebSocketEvent for GatewayReady {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#request-guild-members-request-guild-members-structure
pub struct GatewayRequestGuildMembers {
    pub guild_id: String,
    pub query: Option<String>,
    pub limit: u64,
    pub presence: Option<bool>,
    pub user_ids: Option<String>,
    pub nonce: Option<String>,
}

impl WebSocketEvent for GatewayRequestGuildMembers {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#update-voice-state-gateway-voice-state-update-structure
pub struct GatewayVoiceStateUpdate {
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
}

impl WebSocketEvent for GatewayVoiceStateUpdate {}

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
    pub last_pin_timestamp: Option<DateTime<Utc>>
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
/// Not directly serialized, as the inner payload is the user object
pub struct UserUpdate {
    pub user: UserObject,
}

impl WebSocketEvent for UserUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-create
/// Not directly serialized, as the inner payload is a channel object
pub struct ChannelCreate {
    pub channel: Channel,
}

impl WebSocketEvent for ChannelCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-update
/// Not directly serialized, as the inner payload is a channel object
pub struct ChannelUpdate {
    pub channel: Channel,
}

impl WebSocketEvent for ChannelUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#channel-delete
/// Not directly serialized, as the inner payload is a channel object
pub struct ChannelDelete {
    pub channel: Channel,
}

impl WebSocketEvent for ChannelDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-create
/// Not directly serialized, as the inner payload is a channel object
pub struct ThreadCreate {
    pub thread: Channel,
}

impl WebSocketEvent for ThreadCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-update
/// Not directly serialized, as the inner payload is a channel object
pub struct ThreadUpdate {
    pub thread: Channel,
}

impl WebSocketEvent for ThreadUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-delete
/// Not directly serialized, as the inner payload is a channel object
pub struct ThreadDelete {
    pub thread: Channel,
}

impl WebSocketEvent for ThreadDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-list-sync
pub struct ThreadListSync {
    pub guild_id: String,
    pub channel_ids: Option<Vec<String>>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>
}

impl WebSocketEvent for ThreadListSync {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#thread-member-update
/// The inner payload is a thread member object with an extra field.
/// The extra field is a bit painful, because we can't just serialize a thread member object
pub struct ThreadMemberUpdate {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub join_timestamp: Option<String>,
    pub flags: Option<u64>,
    pub member: Option<GuildMember>,
    pub guild_id: String,
}

impl ThreadMemberUpdate {
    /// Convert self to a thread member, losing the added guild_id field
    pub fn to_thread_member(&self) -> ThreadMember {
        ThreadMember { id: self.id, user_id: self.user_id, join_timestamp: self.join_timestamp.clone(), flags: self.flags, member: self.member.clone() }
    }
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
    pub removed_members: Option<Vec<String>>
}

impl WebSocketEvent for ThreadMembersUpdate {}

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-create
/// This one is particularly painful, it can be a Guild object with extra field or an unavailbile guild object
pub struct GuildCreate {
    pub d: GuildCreateDataOption
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GuildCreateDataOption {
    UnavailableGuild(UnavailableGuild),
    Guild(Guild)
}

impl Default for GuildCreateDataOption {
    fn default() -> Self {
        GuildCreateDataOption::UnavailableGuild(UnavailableGuild::default())
    }
}
impl WebSocketEvent for GuildCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Option<serde_json::Value>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

impl WebSocketEvent for GatewayPayload {}

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
