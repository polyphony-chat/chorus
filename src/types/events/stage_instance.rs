use serde::{Deserialize, Serialize};

use crate::types::StageInstance;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#stage-instance-create
pub struct StageInstanceCreate {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#stage-instance-update
pub struct StageInstanceUpdate {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#stage-instance-delete
pub struct StageInstanceDelete {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}
