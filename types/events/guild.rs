use serde::{Deserialize, Serialize};
use crate::entities::{Guild, UnavailableGuild, User};
use crate::events::WebSocketEvent;

#[derive(Debug, Deserialize, Serialize, Default)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-create
/// This one is particularly painful, it can be a Guild object with extra field or an unavailbile guild object
pub struct GuildCreate {
    pub d: GuildCreateDataOption,
}

#[derive(Debug, Deserialize, Serialize)]
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
    pub user: User,
}

impl WebSocketEvent for GuildBanAdd {}

#[derive(Debug, Default, Deserialize, Serialize)]
/// See https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove
pub struct GuildBanRemove {
    pub guild_id: String,
    pub user: User,
}

impl WebSocketEvent for GuildBanRemove {}