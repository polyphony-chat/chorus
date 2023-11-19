use super::*;
use crate::types;

#[derive(Default, Debug)]
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
    pub gateway_identify_payload: GatewayEvent<types::GatewayIdentifyPayload>,
    pub gateway_resume: GatewayEvent<types::GatewayResume>,
    pub error: GatewayEvent<GatewayError>,
}

#[derive(Default, Debug)]
pub struct Application {
    pub command_permissions_update: GatewayEvent<types::ApplicationCommandPermissionsUpdate>,
}

#[derive(Default, Debug)]
pub struct AutoModeration {
    pub rule_create: GatewayEvent<types::AutoModerationRuleCreate>,
    pub rule_update: GatewayEvent<types::AutoModerationRuleUpdate>,
    pub rule_delete: GatewayEvent<types::AutoModerationRuleDelete>,
    pub action_execution: GatewayEvent<types::AutoModerationActionExecution>,
}

#[derive(Default, Debug)]
pub struct Session {
    pub ready: GatewayEvent<types::GatewayReady>,
    pub ready_supplemental: GatewayEvent<types::GatewayReadySupplemental>,
    pub replace: GatewayEvent<types::SessionsReplace>,
}

#[derive(Default, Debug)]
pub struct StageInstance {
    pub create: GatewayEvent<types::StageInstanceCreate>,
    pub update: GatewayEvent<types::StageInstanceUpdate>,
    pub delete: GatewayEvent<types::StageInstanceDelete>,
}

#[derive(Default, Debug)]
pub struct Message {
    pub create: GatewayEvent<types::MessageCreate>,
    pub update: GatewayEvent<types::MessageUpdate>,
    pub delete: GatewayEvent<types::MessageDelete>,
    pub delete_bulk: GatewayEvent<types::MessageDeleteBulk>,
    pub reaction_add: GatewayEvent<types::MessageReactionAdd>,
    pub reaction_remove: GatewayEvent<types::MessageReactionRemove>,
    pub reaction_remove_all: GatewayEvent<types::MessageReactionRemoveAll>,
    pub reaction_remove_emoji: GatewayEvent<types::MessageReactionRemoveEmoji>,
    pub ack: GatewayEvent<types::MessageACK>,
}

#[derive(Default, Debug)]
pub struct User {
    pub update: GatewayEvent<types::UserUpdate>,
    pub guild_settings_update: GatewayEvent<types::UserGuildSettingsUpdate>,
    pub presence_update: GatewayEvent<types::PresenceUpdate>,
    pub typing_start: GatewayEvent<types::TypingStartEvent>,
}

#[derive(Default, Debug)]
pub struct Relationship {
    pub add: GatewayEvent<types::RelationshipAdd>,
    pub remove: GatewayEvent<types::RelationshipRemove>,
}

#[derive(Default, Debug)]
pub struct Channel {
    pub create: GatewayEvent<types::ChannelCreate>,
    pub update: GatewayEvent<types::ChannelUpdate>,
    pub unread_update: GatewayEvent<types::ChannelUnreadUpdate>,
    pub delete: GatewayEvent<types::ChannelDelete>,
    pub pins_update: GatewayEvent<types::ChannelPinsUpdate>,
}

#[derive(Default, Debug)]
pub struct Thread {
    pub create: GatewayEvent<types::ThreadCreate>,
    pub update: GatewayEvent<types::ThreadUpdate>,
    pub delete: GatewayEvent<types::ThreadDelete>,
    pub list_sync: GatewayEvent<types::ThreadListSync>,
    pub member_update: GatewayEvent<types::ThreadMemberUpdate>,
    pub members_update: GatewayEvent<types::ThreadMembersUpdate>,
}

#[derive(Default, Debug)]
pub struct Guild {
    pub create: GatewayEvent<types::GuildCreate>,
    pub update: GatewayEvent<types::GuildUpdate>,
    pub delete: GatewayEvent<types::GuildDelete>,
    pub audit_log_entry_create: GatewayEvent<types::GuildAuditLogEntryCreate>,
    pub ban_add: GatewayEvent<types::GuildBanAdd>,
    pub ban_remove: GatewayEvent<types::GuildBanRemove>,
    pub emojis_update: GatewayEvent<types::GuildEmojisUpdate>,
    pub stickers_update: GatewayEvent<types::GuildStickersUpdate>,
    pub integrations_update: GatewayEvent<types::GuildIntegrationsUpdate>,
    pub member_add: GatewayEvent<types::GuildMemberAdd>,
    pub member_remove: GatewayEvent<types::GuildMemberRemove>,
    pub member_update: GatewayEvent<types::GuildMemberUpdate>,
    pub members_chunk: GatewayEvent<types::GuildMembersChunk>,
    pub role_create: GatewayEvent<types::GuildRoleCreate>,
    pub role_update: GatewayEvent<types::GuildRoleUpdate>,
    pub role_delete: GatewayEvent<types::GuildRoleDelete>,
    pub role_scheduled_event_create: GatewayEvent<types::GuildScheduledEventCreate>,
    pub role_scheduled_event_update: GatewayEvent<types::GuildScheduledEventUpdate>,
    pub role_scheduled_event_delete: GatewayEvent<types::GuildScheduledEventDelete>,
    pub role_scheduled_event_user_add: GatewayEvent<types::GuildScheduledEventUserAdd>,
    pub role_scheduled_event_user_remove: GatewayEvent<types::GuildScheduledEventUserRemove>,
    pub passive_update_v1: GatewayEvent<types::PassiveUpdateV1>,
}

#[derive(Default, Debug)]
pub struct Invite {
    pub create: GatewayEvent<types::InviteCreate>,
    pub delete: GatewayEvent<types::InviteDelete>,
}

#[derive(Default, Debug)]
pub struct Integration {
    pub create: GatewayEvent<types::IntegrationCreate>,
    pub update: GatewayEvent<types::IntegrationUpdate>,
    pub delete: GatewayEvent<types::IntegrationDelete>,
}

#[derive(Default, Debug)]
pub struct Interaction {
    pub create: GatewayEvent<types::InteractionCreate>,
}

#[derive(Default, Debug)]
pub struct Call {
    pub create: GatewayEvent<types::CallCreate>,
    pub update: GatewayEvent<types::CallUpdate>,
    pub delete: GatewayEvent<types::CallDelete>,
}

#[derive(Default, Debug)]
pub struct Voice {
    pub state_update: GatewayEvent<types::VoiceStateUpdate>,
    pub server_update: GatewayEvent<types::VoiceServerUpdate>,
}

#[derive(Default, Debug)]
pub struct Webhooks {
    pub update: GatewayEvent<types::WebhooksUpdate>,
}
