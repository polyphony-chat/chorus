// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{
    entities::{
        Application, Attachment, Channel, GuildMember, PublicUser, RoleSubscriptionData, Sticker,
        StickerItem, User,
    },
    utils::Snowflake,
    Shared,
};
use crate::{UInt32, UInt8};

use super::option_arc_rwlock_ptr_eq;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// Represents a message sent in a channel.
///
/// # Reference
/// See <https://docs.discord.food/resources/message#message-object>
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub author: Option<PublicUser>,
    pub content: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub tts: Option<bool>,
    pub mention_everyone: bool,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mentions: Option<Vec<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mention_roles: Option<Vec<Snowflake>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub mention_channels: Option<Vec<ChannelMention>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub attachments: Option<Vec<Attachment>>,
    #[cfg(feature = "sqlx")]
    pub embeds: sqlx::types::Json<Vec<Embed>>,
    #[cfg(not(feature = "sqlx"))]
    pub embeds: Option<Vec<Embed>>,
    #[cfg(feature = "sqlx")]
    pub reactions: Option<sqlx::types::Json<Vec<Reaction>>>,
    #[cfg(not(feature = "sqlx"))]
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<serde_json::Value>,
    pub pinned: bool,
    pub webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    #[cfg(feature = "sqlx")]
    pub activity: Option<sqlx::types::Json<MessageActivity>>,
    #[cfg(not(feature = "sqlx"))]
    pub activity: Option<MessageActivity>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub application: Option<Application>,
    pub application_id: Option<Snowflake>,
    #[cfg(feature = "sqlx")]
    pub message_reference: Option<sqlx::types::Json<MessageReference>>,
    #[cfg(not(feature = "sqlx"))]
    pub message_reference: Option<MessageReference>,
    pub flags: Option<MessageFlags>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub referenced_message: Option<Box<Message>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub interaction: Option<MessageInteraction>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub thread: Option<Channel>,
    #[cfg(feature = "sqlx")]
    pub components: Option<sqlx::types::Json<Vec<Component>>>,
    #[cfg(not(feature = "sqlx"))]
    pub components: Option<Vec<Component>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub sticker_items: Option<Vec<StickerItem>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub stickers: Option<Vec<Sticker>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub role_subscription_data: Option<RoleSubscriptionData>,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.channel_id == other.channel_id
            && self.author == other.author
            && self.content == other.content
            && self.timestamp == other.timestamp
            && self.edited_timestamp == other.edited_timestamp
            && self.tts == other.tts
            && self.mention_everyone == other.mention_everyone
            && self.mentions == other.mentions
            && self.mention_roles == other.mention_roles
            && self.mention_channels == other.mention_channels
            && self.attachments == other.attachments
            && self.embeds == other.embeds
            && self.embeds == other.embeds
            && self.reactions == other.reactions
            && self.reactions == other.reactions
            && self.nonce == other.nonce
            && self.pinned == other.pinned
            && self.webhook_id == other.webhook_id
            && self.message_type == other.message_type
            && self.activity == other.activity
            && self.activity == other.activity
            && self.application == other.application
            && self.application_id == other.application_id
            && self.message_reference == other.message_reference
            && self.message_reference == other.message_reference
            && self.flags == other.flags
            && self.referenced_message == other.referenced_message
            && self.interaction == other.interaction
            && self.thread == other.thread
            && self.components == other.components
            && self.components == other.components
            && self.sticker_items == other.sticker_items
            && self.stickers == other.stickers
            && self.role_subscription_data == other.role_subscription_data
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Ord, PartialOrd, Copy)]
/// # Reference
/// See <https://docs.discord.food/resources/message#message-reference-object>
pub struct MessageReference {
    #[serde(rename = "type")]
    #[serde(default)]
    pub reference_type: MessageReferenceType,
    pub message_id: Snowflake,
    pub channel_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub fail_if_not_exists: Option<bool>,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageReferenceType {
    /// A standard reference used by replies and system messages
    #[default]
    Default = 0,
    /// A reference used to point to a message at a point in time
    Forward = 1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageInteraction {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub interaction_type: UInt8,
    pub name: String,
    pub user: User,
    pub member: Option<Shared<GuildMember>>,
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl PartialEq for MessageInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.interaction_type == other.interaction_type
            && self.name == other.name
            && self.user == other.user
            && option_arc_rwlock_ptr_eq(&self.member, &other.member)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, Eq, PartialOrd, Ord, Hash)]
pub struct AllowedMention {
    parse: Vec<AllowedMentionType>,
    roles: Vec<Snowflake>,
    users: Vec<Snowflake>,
    replied_user: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AllowedMentionType {
    Roles,
    Users,
    Everyone,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    #[serde(rename = "type")]
    channel_type: i32,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    embed_type: Option<EmbedType>,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EmbedType {
    #[deprecated]
    ApplicationNews,
    Article,
    AutoModerationMessage,
    AutoModerationNotification,
    Gift,
    #[serde(rename = "gifv")]
    GifVideo,
    Image,
    Link,
    PostPreview,
    Rich,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub struct EmbedImage {
    url: String,
    proxy_url: String,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub struct EmbedThumbnail {
    url: String,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord, Hash)]
struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<i32>,
    width: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub struct EmbedAuthor {
    name: String,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// # Reference
/// See <https://docs.discord.food/resources/message#reaction-object>
pub struct Reaction {
    pub count: UInt32,
    pub burst_count: UInt32,
    #[serde(default)]
    pub me: bool,
    #[serde(default)]
    pub burst_me: bool,
    pub burst_colors: Vec<String>,
    pub emoji: PartialEmoji,
    #[cfg(feature = "sqlx")]
    #[serde(skip)]
    pub user_ids: Vec<Snowflake>,
}

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// # Reference
/// See <https://docs.discord.food/resources/message#message-activity-object>
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: i64,
    pub party_id: Option<String>,
}

#[derive(
    Debug, Default, PartialEq, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.food/resources/message#message-type>
pub enum MessageType {
    /// A default message
    #[default]
    Default = 0,
    /// A message sent when a user is added to a group DM or thread
    RecipientAdd = 1,
    /// A message sent when a user is removed from a group DM or thread
    RecipientRemove = 2,
    /// A message sent when a user creates a call in a private channel
    Call = 3,
    /// A message sent when a group DM or thread's name is changed
    ChannelNameChange = 4,
    /// A message sent when a group DM's icon is changed
    ChannelIconChange = 5,
    /// A message sent when a message is pinned in a channel
    ChannelPinnedMessage = 6,
    /// A message sent when a user joins a guild
    GuildMemberJoin = 7,
    /// A message sent when a user subscribes to (boosts) a guild
    UserPremiumGuildSubscription = 8,
    /// A message sent when a user subscribes to (boosts) a guild to tier 1
    UserPremiumGuildSubscriptionTier1 = 9,
    /// A message sent when a user subscribes to (boosts) a guild to tier 2
    UserPremiumGuildSubscriptionTier2 = 10,
    /// A message sent when a user subscribes to (boosts) a guild to tier 3
    UserPremiumGuildSubscriptionTier3 = 11,
    /// A message sent when a news channel is followed
    ChannelFollowAdd = 12,
    /// A message sent when a user starts streaming in a guild (deprecated)
    #[deprecated]
    GuildStream = 13,
    /// A message sent when a guild is disqualified from discovery
    GuildDiscoveryDisqualified = 14,
    /// A message sent when a guild requalifies for discovery
    GuildDiscoveryRequalified = 15,
    /// A message sent when a guild has failed discovery requirements for a week
    GuildDiscoveryGracePeriodInitial = 16,
    /// A message sent when a guild has failed discovery requirements for 3 weeks
    GuildDiscoveryGracePeriodFinal = 17,
    /// A message sent when a thread is created
    ThreadCreated = 18,
    /// A message sent when a user replies to a message
    Reply = 19,
    /// A message sent when a user uses a slash command
    #[serde(rename = "CHAT_INPUT_COMMAND")]
    ApplicationCommand = 20,
    /// A message sent when a thread starter message is added to a thread
    ThreadStarterMessage = 21,
    /// A message sent to remind users to invite friends to a guild
    GuildInviteReminder = 22,
    /// A message sent when a user uses a context menu command
    ContextMenuCommand = 23,
    /// A message sent when auto moderation takes an action
    AutoModerationAction = 24,
    /// A message sent when a user purchases or renews a role subscription
    RoleSubscriptionPurchase = 25,
    /// A message sent when a user is upsold to a premium interaction
    InteractionPremiumUpsell = 26,
    /// A message sent when a stage channel starts
    StageStart = 27,
    /// A message sent when a stage channel ends
    StageEnd = 28,
    /// A message sent when a user starts speaking in a stage channel
    StageSpeaker = 29,
    /// A message sent when a user raises their hand in a stage channel
    StageRaiseHand = 30,
    /// A message sent when a stage channel's topic is changed
    StageTopic = 31,
    /// A message sent when a user purchases an application premium subscription
    GuildApplicationPremiumSubscription = 32,
    /// A message sent when a user adds an application to group DM
    PrivateChannelIntegrationAdded = 33,
    /// A message sent when a user removed an application from a group DM
    PrivateChannelIntegrationRemoved = 34,
    /// A message sent when a user gifts a premium (Nitro) referral
    PremiumReferral = 35,
    /// A message sent when a user enabled lockdown for the guild
    GuildIncidentAlertModeEnabled = 36,
    /// A message sent when a user disables lockdown for the guild
    GuildIncidentAlertModeDisabled = 37,
    /// A message sent when a user reports a raid for the guild
    GuildIncidentReportRaid = 38,
    /// A message sent when a user reports a false alarm for the guild
    GuildIncidentReportFalseAlarm = 39,
    /// A message sent when no one sends a message in the current channel for 1 hour
    GuildDeadchatRevivePrompt = 40,
    /// A message sent when a user buys another user a gift
    CustomGift = 41,
    GuildGamingStatsPrompt = 42,
    /// A message sent when a user purchases a guild product
    PurchaseNotification = 44,
}

bitflags! {
    #[derive(Debug, Clone, Copy,  PartialEq, Eq, Hash, PartialOrd, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.food/resources/message#message-type>
    pub struct MessageFlags: u64 {
        /// This message has been published to subscribed channels (via Channel Following)
        const CROSSPOSTED = 1 << 0;
        ///	This message originated from a message in another channel (via Channel Following)
        const IS_CROSSPOST = 1 << 1;
        /// Embeds will not be included when serializing this message
        const SUPPRESS_EMBEDS = 1 << 2;
        /// The source message for this crosspost has been deleted (via Channel Following)
        const SOURCE_MESSAGE_DELETED = 1 << 3;
        /// This message came from the urgent message system
        const URGENT = 1 << 4;
        /// This message has an associated thread, with the same ID as the message
        const HAS_THREAD = 1 << 5;
        /// This message is only visible to the user who invoked the interaction
        const EPHEMERAL = 1 << 6;
        /// This message is an interaction response and the bot is "thinking"
        const LOADING = 1 << 7;
        /// Some roles were not mentioned and added to the thread
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;
        /// This message contains a link that impersonates Discord
        const SHOULD_SHOW_LINK_NOT_DISCORD_WARNING = 1 << 10;
        /// This message will not trigger push and desktop notifications
        const SUPPRESS_NOTIFICATIONS = 1 << 12;
        /// This message's audio attachments are rendered as voice messages
        const VOICE_MESSAGE = 1 << 13;
        /// This message has a forwarded message snapshot attached
        const HAS_SNAPSHOT = 1 << 14;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
// Note: this is likely used for unicode emojis
pub struct PartialEmoji {
    /// Note: if id is None, the name field
    /// is a unicode emoji (the only data given)
    #[serde(default)]
    pub id: Option<Snowflake>,

    pub name: String,

    #[serde(default)]
    pub animated: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(not(feature = "sqlx"), repr(u8))]
#[cfg_attr(feature = "sqlx", repr(i16))]
pub enum ReactionType {
    Normal = 0,
    Burst = 1, // The dreaded super reactions
}
