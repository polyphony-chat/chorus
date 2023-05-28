use crate::types::entities::{Guild, UnavailableGuild, PublicUser};
use crate::types::events::WebSocketEvent;
use crate::types::{AuditLogEntry, Emoji, GuildMember, GuildScheduledEvent, RoleObject, Sticker};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::PresenceUpdate;

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-create
/// This one is particularly painful, it can be a Guild object with an extra field or an unavailable guild object
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
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-add-guild-ban-add-event-fields
pub struct GuildBanAdd {
    pub guild_id: String,
    pub user: PublicUser,
}

impl WebSocketEvent for GuildBanAdd {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove
pub struct GuildBanRemove {
    pub guild_id: String,
    pub user: PublicUser,
}

impl WebSocketEvent for GuildBanRemove {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-update
pub struct GuildUpdate {
    #[serde(flatten)]
    pub guild: Guild,
}

impl WebSocketEvent for GuildUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-delete
pub struct GuildDelete {
    #[serde(flatten)]
    pub guild: UnavailableGuild,
}

impl WebSocketEvent for GuildDelete {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-audit-log-entry-create
pub struct GuildAuditLogEntryCreate {
    #[serde(flatten)]
    pub entry: AuditLogEntry,
}

impl WebSocketEvent for GuildAuditLogEntryCreate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-emojis-update
pub struct GuildEmojisUpdate {
    pub guild_id: String,
    pub emojis: Vec<Emoji>,
}

impl WebSocketEvent for GuildEmojisUpdate {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-stickers-update
pub struct GuildStickersUpdate {
    pub guild_id: String,
    pub stickers: Vec<Sticker>,
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
    pub user: PublicUser,
}

impl WebSocketEvent for GuildMemberRemove {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-member-update
pub struct GuildMemberUpdate {
    pub guild_id: String,
    pub roles: Vec<String>,
    pub user: PublicUser,
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
    pub nonce: Option<String>,
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
