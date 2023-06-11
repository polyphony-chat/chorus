use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Attachment {
    pub id: Snowflake,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f32>,
    pub waveform: Option<String>,
    #[serde(skip_serializing)]
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PartialDiscordFileAttachment {
    pub id: Option<i16>,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<i64>,
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f32>,
    pub waveform: Option<String>,
    #[serde(skip_serializing)]
    pub content: Vec<u8>,
}

impl PartialDiscordFileAttachment {
    /**
    Moves `self.content` out of `self` and returns it.
    # Returns
    Vec<u8>
     */
    pub fn move_content(self) -> (Vec<u8>, PartialDiscordFileAttachment) {
        let content = self.content;
        let updated_struct = PartialDiscordFileAttachment {
            id: self.id,
            filename: self.filename,
            description: self.description,
            content_type: self.content_type,
            size: self.size,
            url: self.url,
            proxy_url: self.proxy_url,
            height: self.height,
            width: self.width,
            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform,
            content: Vec::new(),
        };
        (content, updated_struct)
    }

    pub fn move_filename(self) -> (String, PartialDiscordFileAttachment) {
        let filename = self.filename;
        let updated_struct = PartialDiscordFileAttachment {
            id: self.id,
            filename: String::new(),
            description: self.description,
            content_type: self.content_type,
            size: self.size,
            url: self.url,
            proxy_url: self.proxy_url,
            height: self.height,
            width: self.width,

            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform,
            content: self.content,
        };
        (filename, updated_struct)
    }

    pub fn move_content_type(self) -> (Option<String>, PartialDiscordFileAttachment) {
        let content_type = self.content_type;
        let updated_struct = PartialDiscordFileAttachment {
            id: self.id,
            filename: self.filename,
            description: self.description,
            content_type: None,
            size: self.size,
            url: self.url,
            proxy_url: self.proxy_url,
            height: self.height,
            width: self.width,
            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform,
            content: self.content,
        };
        (content_type, updated_struct)
    }

    pub fn set_id(&mut self, id: i16) {
        self.id = Some(id);
    }
}
