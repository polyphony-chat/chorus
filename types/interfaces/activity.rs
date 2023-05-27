use serde::{Deserialize, Serialize};
use crate::entities::Emoji;

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