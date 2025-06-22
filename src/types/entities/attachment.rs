// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;
use crate::{UInt32, UInt64};

use super::{Application, User};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, PartialOrd)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://discord.com/developers/docs/resources/channel#attachment-object>
pub struct Attachment {
    pub id: Snowflake,
    /// The name of the file without the extension or title of the clip
    ///
    /// (max 1024 characters, automatically provided when the filename is normalized or randomly generated due to invalid characters)
    pub title: Option<String>,
    pub filename: String,
    /// Max 1024 characters
    pub description: Option<String>,
    /// The file's [media type](https://en.wikipedia.org/wiki/Media_type)
    pub content_type: Option<String>,
    /// The size of the file in bytes
    pub size: UInt64,
    /// Source URL of the file
    pub url: String,
    /// A proxied url of the file, only for attachments with a defined width and height
    pub proxy_url: String,
    pub height: Option<UInt64>,
    pub width: Option<UInt64>,
    /// The version of the explicit content scan filter this attachment was scanned with
    ///
    /// This field is missing if the attachment has not yet been scanned.
    ///
    /// If it set to 0, the
    /// attachment is not eligible for a scan.
    // TODO: Do we have an endpoint for this?
    pub content_scan_version: Option<UInt32>,
    /// The attachment placeholder protocol version (currently 1)
    pub placeholder_version: Option<UInt32>,
    /// A low-resolution thumbhash of the attachment to display before it is loaded
    ///
    /// See <https://github.com/evanw/thumbhash>
    pub placeholder: Option<String>,
    /// If set to true, the attachment will be automatically removed after a set period of time.
    ///
    /// Ephemeral attachments on messages are guaranteed to be available as long as the message itself exists.
    pub ephemeral: Option<bool>,
    /// The duration of the audio file (only for voice messages)
    pub duration_secs: Option<f32>,
    /// A Base64 encoded bytearray representing a sampled waveform (only for voice messages)
    ///
    /// # Notes
    /// Note that this is computed on the client side.
    /// This means it can be spoofed and isn't necessarily accurate.
    pub waveform: Option<String>,
    /// See [AttachmentFlags]
    pub flags: Option<AttachmentFlags>,

    pub clip_created_at: Option<chrono::DateTime<Utc>>,

    /// The participants in the clip (max 100)
    pub clip_participants: Option<Vec<User>>,

    /// The application the clip was taken in
    #[serde(rename = "application")]
    pub clip_application: Option<Application>,

    // FIXME: why is this here? this is not received in any API (Spacebar / Discord)
    #[serde(skip_serializing)]
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub content: Option<Vec<u8>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
/// Discord.com's send schema for file attachments
///
/// # Reference
/// See <https://docs.discord.food/resources/message#attachment-structure>
pub struct PartialDiscordFileAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    /// The name of the file without the extension or title of the clip
    ///
    /// (max 1024 characters, automatically provided when the filename is normalized or randomly generated due to invalid characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    pub filename: String,

    /// The name of the file pre-uploaded to Discord's GCP bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_filename: Option<String>,

    /// Max 1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // Note: this field is marked as receive-only, we don't send it in json
    #[serde(skip_serializing)]
    /// The file's [media type](https://en.wikipedia.org/wiki/Media_type)
    pub content_type: Option<String>,

    /// The size of the file in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,

    /// The duration of the audio file (only for voice messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f32>,

    /// A Base64 encoded bytearray representing a sampled waveform (only for voice messages)
    ///
    /// # Notes
    /// Note that this is computed on the client side.
    /// This means it can be spoofed and isn't necessarily accurate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,

    /// Whether the file being uploaded is a clipped recording of a stream.
    ///
    /// If true, `clip_created_at` and `clip_participant_ids` are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_clip: Option<bool>,

    /// Whether the file being uploaded is a thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_thumbnail: Option<bool>,

    /// Whether this attachment is a remixed version of another attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_remix: Option<bool>,

    /// Whether this attachment is a spoiler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_spoiler: Option<bool>,

    /// Required if `is_clip` is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_created_at: Option<chrono::DateTime<Utc>>,

    /// The IDs of the participants in the clip (max 100)
    ///
    /// Required if `is_clip` is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_participant_ids: Option<Vec<Snowflake>>,

    /// The ID of the application the clip was taken in
    #[serde(rename = "application_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_application_id: Option<Snowflake>,

    /// Note: this field is added as part of chorus' API, not mapped from a remote interface
    #[serde(skip_serializing)]
    pub content: Vec<u8>,
}

impl PartialDiscordFileAttachment {
    /// Clones every field except `content`, which is set to an empty [Vec]
    pub fn clone_metadata(&self) -> PartialDiscordFileAttachment {
        PartialDiscordFileAttachment {
            id: self.id,
            title: self.title.clone(),
            filename: self.filename.clone(),
            uploaded_filename: self.uploaded_filename.clone(),
            description: self.description.clone(),
            content_type: self.content_type.clone(),
            size: self.size.clone(),
            ephemeral: self.ephemeral,
            duration_secs: self.duration_secs,
            waveform: self.waveform.clone(),
            is_clip: self.is_clip,
            is_thumbnail: self.is_thumbnail,
            is_remix: self.is_remix,
            is_spoiler: self.is_spoiler,
            clip_created_at: self.clip_created_at,
            clip_participant_ids: self.clip_participant_ids.clone(),
            clip_application_id: self.clip_application_id,
            content: Vec::new(),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags, PartialOrd)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.food/resources/message#attachment-flags>
    pub struct AttachmentFlags: u64 {
        /// Attachment is a clipped recording of a stream
        ///
        /// Set on the client side with `is_clip`
        const IS_CLIP = 1 << 0;

        /// Attachment is a thumbnail
        ///
        /// Set on the client side with `is_thumbnail`
        const IS_THUMBNAIL = 1 << 1;

        /// Attachment is a remix of another attachment
        ///
        /// Set on the client side with `is_remix`
        const IS_REMIX = 1 << 2;

        /// Attachment is a spoiler
        ///
        /// Set on the client side with `is_spoiler`
        const IS_SPOILER = 1 << 3;

        /// Attachment was flagged as sensitive content
        const CONTAINS_EXPLICIT_MEDIA = 1 << 4;

        /// Attachment is an animated image
        const IS_ANIMATED = 1 << 5;
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
/// Discord.com's send schema for Google Cloud storage attachments
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/message#upload-attachment-structure>
pub struct CloudUploadAttachment {
    /// The ID of the attachment to reference in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    pub filename: String,
    /// The size of the file in bytes
    pub file_size: u64,

    /// Whether the file being uploaded is a clipped recording of a stream.
    ///
    /// If true, `clip_created_at` and `clip_participant_ids` are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_clip: Option<bool>,

    /// Required if `is_clip` is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_created_at: Option<chrono::DateTime<Utc>>,

    /// The IDs of the participants in the clip (max 100)
    ///
    /// Required if `is_clip` is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_participant_ids: Option<Vec<Snowflake>>,

    /// The ID of the application the clip was taken in
    #[serde(rename = "application_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip_application_id: Option<Snowflake>,

    /// The title of the clip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Note: this field is added as part of chorus' API, not mapped from a remote interface
    #[serde(skip_serializing)]
    pub content: Vec<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
/// Discord.com's receive schema for Google Cloud storage attachments
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/message#cloud-attachment-structure>
pub struct CloudAttachment {
    /// The ID of the attachment upload, if provided in the request
    pub id: Option<u64>,

    /// The URL to upload the file to
    pub upload_url: String,

    /// The name of the uploaded file
    pub upload_filename: String,
}
