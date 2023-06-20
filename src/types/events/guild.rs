use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::entities::{Guild, PublicUser, UnavailableGuild};
use crate::types::{AuditLogEntry, Emoji, GuildMember, GuildScheduledEvent, RoleObject, Sticker};

use super::PresenceUpdate;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-create;
/// Received to give data about a guild;
// This one is particularly painful, it can be a Guild object with an extra field or an unavailable guild object
pub struct GuildCreate {
    #[serde(flatten)]
    pub d: GuildCreateDataOption,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-add-guild-ban-add-event-fields;
/// Received to give info about a user being banned from a guild;
pub struct GuildBanAdd {
    pub guild_id: String,
    pub user: PublicUser,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove;
/// Received to give info about a user being unbanned from a guild;
pub struct GuildBanRemove {
    pub guild_id: String,
    pub user: PublicUser,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-update;
/// Received to give info about a guild being updated;
pub struct GuildUpdate {
    #[serde(flatten)]
    pub guild: Guild,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-delete;
/// Received to tell the client about a guild being deleted;
pub struct GuildDelete {
    #[serde(flatten)]
    pub guild: UnavailableGuild,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-audit-log-entry-create;
/// Received to the client about an audit log entry being added;
pub struct GuildAuditLogEntryCreate {
    #[serde(flatten)]
    pub entry: AuditLogEntry,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-emojis-update;
/// Received to tell the client about a change to a guild's emoji list;
pub struct GuildEmojisUpdate {
    pub guild_id: String,
    pub emojis: Vec<Emoji>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-stickers-update;
/// Received to tell the client about a change to a guild's sticker list;
pub struct GuildStickersUpdate {
    pub guild_id: String,
    pub stickers: Vec<Sticker>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-integrations-update
pub struct GuildIntegrationsUpdate {
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-member-add;
/// Received to tell the client about a user joining a guild;
pub struct GuildMemberAdd {
    #[serde(flatten)]
    pub member: GuildMember,
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-member-remove;
/// Received to tell the client about a user leaving a guild;
pub struct GuildMemberRemove {
    pub guild_id: String,
    pub user: PublicUser,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-role-create
pub struct GuildRoleCreate {
    pub guild_id: String,
    pub role: RoleObject,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-role-update
pub struct GuildRoleUpdate {
    pub guild_id: String,
    pub role: RoleObject,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-role-delete
pub struct GuildRoleDelete {
    pub guild_id: String,
    pub role_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-create
pub struct GuildScheduledEventCreate {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-update
pub struct GuildScheduledEventUpdate {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-delete
pub struct GuildScheduledEventDelete {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-add
pub struct GuildScheduledEventUserAdd {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-remove
pub struct GuildScheduledEventUserRemove {
    pub guild_scheduled_event_id: String,
    pub user_id: String,
    pub guild_id: String,
}
