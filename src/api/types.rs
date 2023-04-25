/*
To learn more about the types implemented here, please visit
https://discord.com/developers/docs .
I do not feel like re-documenting all of this, as everything is already perfectly explained there.
 */

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

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    channel_id: String,
    author: UserObject,
    content: String,
    timestamp: String,
    edited_timestamp: Option<String>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<UserObject>,
    mention_roles: Vec<String>,
    mention_channels: Option<Vec<ChannelMention>>,
    attachments: Vec<Attachment>,
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

#[derive(Debug, Serialize, Deserialize)]
struct MessageCreate {
    #[serde(flatten)]
    message: Message,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Vec<(UserObject, GuildMember)>, // Not sure if this is correct: https://discord.com/developers/docs/topics/gateway-events#message-create
}

#[derive(Debug, Serialize, Deserialize)]
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
    attachments: Option<Vec<Attachment>>,
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

#[derive(Debug, Serialize, Deserialize)]
struct MessageUpdate {
    #[serde(flatten)]
    message: PartialMessage,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    mentions: Vec<(UserObject, GuildMember)>, // Not sure if this is correct: https://discord.com/developers/docs/topics/gateway-events#message-create
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageDelete {
    id: String,
    channel_id: String,
    guild_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageDeleteBulk {
    ids: Vec<String>,
    channel_id: String,
    guild_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageReactionAdd {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    member: Option<GuildMember>,
    emoji: Emoji,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageReactionRemove {
    user_id: String,
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageReactionRemoveAll {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageReactionRemoveEmoji {
    channel_id: String,
    message_id: String,
    guild_id: Option<String>,
    emoji: Emoji,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChannelMention {
    id: String,
    guild_id: String,
    #[serde(rename = "type")]
    channel_type: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attachment {
    id: String,
    filename: String,
    description: Option<String>,
    content_type: Option<String>,
    size: i64,
    url: String,
    proxy_url: String,
    height: Option<String>,
    width: Option<String>,
    ephemeral: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
/**
Represents an Embed. [See the Discord Documentation](https://discord.com/developers/docs/resources/channel#embed-object).
 */
struct Embed {
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
struct Reaction {
    count: i32,
    me: bool,
    emoji: Emoji,
}

#[derive(Debug, Deserialize, Serialize)]
struct Emoji {
    id: Option<u64>,
    name: Option<String>,
    roles: Option<Vec<u64>>,
    user: Option<UserObject>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageActivity {
    #[serde(rename = "type")]
    activity_type: i64,
    party_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Application {
    id: String,
    name: String,
    icon: Option<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: bool,
    bot_require_code_grant: bool,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: Option<UserObject>,
    summary: String,
    verify_key: String,
    team: Option<Team>,
    guild_id: Option<String>,
    primary_sku_id: Option<String>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: Option<i32>,
    tags: Option<Vec<String>>,
    install_params: Option<InstallParams>,
    custom_install_url: Option<String>,
    role_connections_verification_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Team {
    icon: Option<String>,
    id: u64,
    members: Vec<TeamMember>,
    name: String,
    owner_user_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct TeamMember {
    membership_state: u8,
    permissions: Vec<String>,
    team_id: u64,
    user: UserObject,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum MembershipState {
    Invited = 1,
    Accepted = 2,
}

#[derive(Debug, Serialize, Deserialize)]
struct InstallParams {
    scopes: Vec<String>,
    permissions: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageReference {
    message_id: Option<String>,
    channel_id: Option<String>,
    guild_id: Option<String>,
    fail_if_not_exists: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageInteraction {
    id: u64,
    #[serde(rename = "type")]
    interaction_type: u8,
    name: String,
    user: UserObject,
    member: Option<GuildMember>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GuildMember {
    user: Option<UserObject>,
    nick: Option<String>,
    avatar: Option<String>,
    roles: Vec<String>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,
    flags: i32,
    pending: Option<bool>,
    permissions: Option<String>,
    communication_disabled_until: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    id: String,
    #[serde(rename = "type")]
    channel_type: i32,
    guild_id: Option<String>,
    position: Option<i32>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<String>,
    bitrate: Option<i32>,
    user_limit: Option<i32>,
    rate_limit_per_user: Option<i32>,
    recipients: Option<Vec<UserObject>>,
    icon: Option<String>,
    owner_id: Option<String>,
    application_id: Option<String>,
    parent_id: Option<String>,
    last_pin_timestamp: Option<String>,
    rtc_region: Option<String>,
    video_quality_mode: Option<i32>,
    message_count: Option<i32>,
    member_count: Option<i32>,
    thread_metadata: Option<ThreadMetadata>,
    member: Option<ThreadMember>,
    default_auto_archive_duration: Option<i32>,
    permissions: Option<String>,
    flags: Option<i32>,
    total_message_sent: Option<i32>,
    available_tags: Option<Vec<Tag>>,
    applied_tags: Option<Vec<String>>,
    default_reaction_emoji: Option<DefaultReaction>,
    default_thread_rate_limit_per_user: Option<i32>,
    default_sort_order: Option<i32>,
    default_forum_layout: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Tag {
    id: u64,
    name: String,
    moderated: bool,
    emoji_id: Option<u64>,
    emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    id: String,
    #[serde(rename = "type")]
    overwrite_type: u8,
    allow: String,
    deny: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ThreadMetadata {
    archived: bool,
    auto_archive_duration: i32,
    archive_timestamp: String,
    locked: bool,
    invitable: Option<bool>,
    create_timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ThreadMember {
    id: Option<u64>,
    user_id: Option<u64>,
    join_timestamp: Option<String>,
    flags: Option<u64>,
    member: Option<GuildMember>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DefaultReaction {
    emoji_id: Option<String>,
    emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Component {
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
struct StickerItem {
    id: u64,
    name: String,
    format_type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sticker {
    id: u64,
    pack_id: Option<u64>,
    name: String,
    description: Option<String>,
    tags: String,
    asset: Option<String>,
    #[serde(rename = "type")]
    sticker_type: u8,
    format_type: u8,
    available: Option<bool>,
    guild_id: Option<u64>,
    user: Option<UserObject>,
    sort_value: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoleSubscriptionData {
    role_subscription_listing_id: u64,
    tier_name: String,
    total_months_subscribed: u32,
    is_renewal: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct TypingStartEvent {
    channel_id: String,
    guild_id: Option<String>,
    user_id: String,
    timestamp: i64,
    member: Option<GuildMember>,
}
