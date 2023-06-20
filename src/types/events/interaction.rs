use serde::{Deserialize, Serialize};

use crate::types::Interaction;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#interaction-create
pub struct InteractionCreate {
    #[serde(flatten)]
    pub interaction: Interaction,
}
