// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use pubserve::Publisher;

use super::*;
use crate::types;

/// Subscribable events the [Gateway] emits.
///
/// Most of these are received via a websocket connection.
///
/// Receiving a [GatewayError] from `error` means the connection was closed.
#[derive(Default, Debug, Clone)]
pub struct Events {
    pub application: Application,
    pub auto_moderation: AutoModeration,
    pub session: Session,
    pub message: Message,
    pub user: User,
    pub relationship: Relationship,
    pub channel: Channel,
    pub thread: Thread,
    pub guild: Guild,
    pub invite: Invite,
    pub integration: Integration,
    pub interaction: Interaction,
    pub stage_instance: StageInstance,
    pub call: Call,
    pub voice: Voice,
    pub webhooks: Webhooks,
    pub mfa: Mfa,
    pub gateway_identify_payload: Publisher<types::GatewayIdentifyPayload>,
    pub gateway_resume: Publisher<types::GatewayResume>,
    pub error: Publisher<GatewayError>,
}

impl Events {
	/// Returns a new [Events] struct with no subscribed observers
	pub fn empty() -> Events {
		Events::default()
	}
}

#[derive(Default, Debug, Clone)]
pub struct Application {
    pub command_permissions_update: Publisher<types::ApplicationCommandPermissionsUpdate>,
}

#[derive(Default, Debug, Clone)]
pub struct AutoModeration {
    pub rule_create: Publisher<types::AutoModerationRuleCreate>,
    pub rule_update: Publisher<types::AutoModerationRuleUpdate>,
    pub rule_delete: Publisher<types::AutoModerationRuleDelete>,
    pub action_execution: Publisher<types::AutoModerationActionExecution>,
}

#[derive(Default, Debug, Clone)]
pub struct Session {
    pub ready: Publisher<types::GatewayReady>,
    pub ready_supplemental: Publisher<types::GatewayReadySupplemental>,
    pub replace: Publisher<types::SessionsReplace>,
    pub reconnect: Publisher<types::GatewayReconnect>,
    pub invalid: Publisher<types::GatewayInvalidSession>,
    pub resumed: Publisher<types::GatewayResumed>,
}

#[derive(Default, Debug, Clone)]
pub struct StageInstance {
    pub create: Publisher<types::StageInstanceCreate>,
    pub update: Publisher<types::StageInstanceUpdate>,
    pub delete: Publisher<types::StageInstanceDelete>,
}

#[derive(Default, Debug, Clone)]
pub struct Message {
    pub create: Publisher<types::MessageCreate>,
    pub update: Publisher<types::MessageUpdate>,
    pub delete: Publisher<types::MessageDelete>,
    pub delete_bulk: Publisher<types::MessageDeleteBulk>,
    pub reaction_add: Publisher<types::MessageReactionAdd>,
    pub reaction_remove: Publisher<types::MessageReactionRemove>,
    pub reaction_remove_all: Publisher<types::MessageReactionRemoveAll>,
    pub reaction_remove_emoji: Publisher<types::MessageReactionRemoveEmoji>,
    pub recent_mention_delete: Publisher<types::RecentMentionDelete>,
    pub ack: Publisher<types::MessageACK>,
    pub last_messages: Publisher<types::LastMessages>,
}

#[derive(Default, Debug, Clone)]
pub struct User {
    pub update: Publisher<types::UserUpdate>,
    pub connections_update: Publisher<types::UserConnectionsUpdate>,
    pub note_update: Publisher<types::UserNoteUpdate>,
    pub guild_settings_update: Publisher<types::UserGuildSettingsUpdate>,
    pub presence_update: Publisher<types::PresenceUpdate>,
    pub typing_start: Publisher<types::TypingStartEvent>,
}

#[derive(Default, Debug, Clone)]
pub struct Relationship {
    pub add: Publisher<types::RelationshipAdd>,
    pub remove: Publisher<types::RelationshipRemove>,
}

#[derive(Default, Debug, Clone)]
pub struct Channel {
    pub create: Publisher<types::ChannelCreate>,
    pub update: Publisher<types::ChannelUpdate>,
    pub unread_update: Publisher<types::ChannelUnreadUpdate>,
    pub delete: Publisher<types::ChannelDelete>,
    pub pins_update: Publisher<types::ChannelPinsUpdate>,
}

#[derive(Default, Debug, Clone)]
pub struct Thread {
    pub create: Publisher<types::ThreadCreate>,
    pub update: Publisher<types::ThreadUpdate>,
    pub delete: Publisher<types::ThreadDelete>,
    pub list_sync: Publisher<types::ThreadListSync>,
    pub member_update: Publisher<types::ThreadMemberUpdate>,
    pub members_update: Publisher<types::ThreadMembersUpdate>,
}

#[derive(Default, Debug, Clone)]
pub struct Guild {
    pub create: Publisher<types::GuildCreate>,
    pub update: Publisher<types::GuildUpdate>,
    pub delete: Publisher<types::GuildDelete>,
    pub audit_log_entry_create: Publisher<types::GuildAuditLogEntryCreate>,
    pub ban_add: Publisher<types::GuildBanAdd>,
    pub ban_remove: Publisher<types::GuildBanRemove>,
    pub emojis_update: Publisher<types::GuildEmojisUpdate>,
    pub stickers_update: Publisher<types::GuildStickersUpdate>,
    pub integrations_update: Publisher<types::GuildIntegrationsUpdate>,
    pub member_add: Publisher<types::GuildMemberAdd>,
    pub member_remove: Publisher<types::GuildMemberRemove>,
    pub member_update: Publisher<types::GuildMemberUpdate>,
    pub members_chunk: Publisher<types::GuildMembersChunk>,
    pub role_create: Publisher<types::GuildRoleCreate>,
    pub role_update: Publisher<types::GuildRoleUpdate>,
    pub role_delete: Publisher<types::GuildRoleDelete>,
    pub role_scheduled_event_create: Publisher<types::GuildScheduledEventCreate>,
    pub role_scheduled_event_update: Publisher<types::GuildScheduledEventUpdate>,
    pub role_scheduled_event_delete: Publisher<types::GuildScheduledEventDelete>,
    pub role_scheduled_event_user_add: Publisher<types::GuildScheduledEventUserAdd>,
    pub role_scheduled_event_user_remove: Publisher<types::GuildScheduledEventUserRemove>,
    pub passive_update_v1: Publisher<types::PassiveUpdateV1>,
}

#[derive(Default, Debug, Clone)]
pub struct Invite {
    pub create: Publisher<types::InviteCreate>,
    pub delete: Publisher<types::InviteDelete>,
}

#[derive(Default, Debug, Clone)]
pub struct Integration {
    pub create: Publisher<types::IntegrationCreate>,
    pub update: Publisher<types::IntegrationUpdate>,
    pub delete: Publisher<types::IntegrationDelete>,
}

#[derive(Default, Debug, Clone)]
pub struct Interaction {
    pub create: Publisher<types::InteractionCreate>,
}

#[derive(Default, Debug, Clone)]
pub struct Call {
    pub create: Publisher<types::CallCreate>,
    pub update: Publisher<types::CallUpdate>,
    pub delete: Publisher<types::CallDelete>,
}

#[derive(Default, Debug, Clone)]
pub struct Voice {
    pub state_update: Publisher<types::VoiceStateUpdate>,
    pub server_update: Publisher<types::VoiceServerUpdate>,
}

#[derive(Default, Debug, Clone)]
pub struct Webhooks {
    pub update: Publisher<types::WebhooksUpdate>,
}

#[derive(Default, Debug, Clone)]
pub struct Mfa {
    pub authenticator_create: Publisher<types::AuthenticatorCreate>,
    pub authenticator_update: Publisher<types::AuthenticatorUpdate>,
    pub authenticator_delete: Publisher<types::AuthenticatorDelete>,
}
