// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        CloudAttachment, CloudUploadAttachment, CreateCloudAttachmentURLsReturn, LimitType,
        PartialDiscordFileAttachment, Snowflake,
    },
};

impl From<PartialDiscordFileAttachment> for CloudUploadAttachment {
    fn from(value: PartialDiscordFileAttachment) -> Self {
        Self::from_attachment(value)
    }
}

impl CloudUploadAttachment {
    /// Creates self from a [PartialDiscordFileAttachment]
    pub fn from_attachment(attachment: PartialDiscordFileAttachment) -> Self {
        Self {
            id: attachment.id,
            filename: attachment.filename,
            file_size: attachment.size.unwrap_or(attachment.content.len() as u64),
            is_clip: attachment.is_clip,
            clip_created_at: attachment.clip_created_at,
            clip_participant_ids: attachment.clip_participant_ids,
            clip_application_id: attachment.clip_application_id,
            title: attachment.title,
            content: attachment.content,
        }
    }

    /// Creates attachment URLs to upload the intended attachments directly to a Google Cloud (or
    /// similar) storage bucket.
    ///
    /// The [CloudUploadAttachment] objects can have empty `content` fields.
    ///
    /// Requires the same permissions as uploading an attachment inline with a message.
    ///
    /// To automatically upload to the URLs as well, see
    /// [CloudUploadAttachment::upload_to_storage_bucket].
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/message#create-attachments>
    pub async fn create_cloud_attachment_urls(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        attachments: &Vec<CloudUploadAttachment>,
    ) -> ChorusResult<Vec<CloudAttachment>> {
        // Note: this is done so we can easily have the attachments in a borrow
        let attachments_as_json = serde_json::to_string(attachments).unwrap();

        let request = ChorusRequest {
            limit_type: LimitType::Channel(channel_id),
            request: Client::new()
                .post(format!(
                    "{}/channels/{}/attachments",
                    &user.belongs_to.read().unwrap().urls.api,
                    channel_id
                ))
                .header("Content-Type", "application/json")
                .body(format!("{{\"files\": {}}}", attachments_as_json)),
        }
        .with_headers_for(user);

        request
            .send_and_deserialize_response::<CreateCloudAttachmentURLsReturn>(user)
            .await
            .map(|x| x.attachments)
    }

    /// Uploads attachments directly to a Google Cloud (or similar) storage bucket.
    ///
    /// Requires the same permissions as uploading an attachment inline with a message.
    ///
    /// Uses the lower level [CloudUploadAttachment::create_cloud_attachment_urls] API and uploads to
    /// those URLs.
    ///
    /// For a higher level API, see [PartialDiscordFileAttachment::bulk_upload_to_storage_bucket].
    ///
    /// # Reference
    /// See <https://discord-userdoccers.vercel.app/resources/message#create-attachments>
    pub async fn upload_to_storage_bucket(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        attachments: Vec<CloudUploadAttachment>,
    ) -> ChorusResult<Vec<CloudAttachment>> {
        let urls = Self::create_cloud_attachment_urls(user, channel_id, &attachments).await?;

        if urls.len() != attachments.len() {
            log::warn!(
                "Received back {} upload URLs for {} attachments!",
                urls.len(),
                attachments.len()
            );
            log::warn!("This instance may be malicious or broken");
            return ChorusResult::Err(ChorusError::InvalidResponse {
                error: "Received different number of upload URLs than we have attachments"
                    .to_string(),
                http_status: reqwest::StatusCode::OK,
            });
        }

        // Note: we want to keep track of the index and also move attachments
        let mut i = 0;
        for attachment in attachments {
            let url = urls.get(i).unwrap();

            if url.id != attachment.id {
                log::warn!(
                    "Attachment URL ID does not match attachment ID! {:?} vs {:?}",
                    url.id,
                    attachment.id
                );
                return ChorusResult::Err(ChorusError::InvalidResponse {
                    error: "Attachment URL ID does not match attachment ID".to_string(),
                    http_status: reqwest::StatusCode::OK,
                });
            }

            let request = Client::new().put(&url.upload_url).body(attachment.content);
            match request.send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        log::warn!(
                            "Received status {} when trying to upload to a cloud storage bucket",
                            resp.status()
                        );
                        return ChorusResult::Err(ChorusError::ReceivedError {
                            error: crate::errors::ApiError {
                                json_error: crate::errors::JsonError {
                                    code: resp.status().as_u16() as u32,
                                    message: None,
                                    errors: None,
                                },
                                http_status: resp.status(),
                            },
                            response_text: resp.text().await.unwrap_or_default(),
                        });
                    }

                    log::trace!(
                        "Successfully uploaded attachment {} to cloud storage as {}!",
                        attachment.filename,
                        url.upload_filename
                    );
                }
                Err(e) => {
                    return ChorusResult::Err(ChorusError::RequestFailed {
                        url: url.upload_url.clone(),
                        error: format!("{}", e),
                    })
                }
            }

            i += 1;
        }

        Ok(urls)
    }
}

impl PartialDiscordFileAttachment {
    /// Uploads a list of [PartialDiscordFileAttachment]s to a Google Cloud (or similar) storage bucket.
    ///
    /// Returns the same list of attachments with `content` set to an empty [Vec] and
    /// `uploaded_filename` properly set.
    ///
    /// Uses the lower level [CloudUploadAttachment] API.
    ///
    /// For a higher level API, see [Message::send](crate::types::Message::send).
    pub async fn bulk_upload_to_storage_bucket(
        user: &mut ChorusUser,
        channel_id: Snowflake,
        attachments: Vec<PartialDiscordFileAttachment>,
    ) -> ChorusResult<Vec<PartialDiscordFileAttachment>> {
        let mut attachments_metadata: Vec<PartialDiscordFileAttachment> =
            attachments.iter().map(|x| x.clone_metadata()).collect();
        let cloud_attachments: Vec<CloudUploadAttachment> =
            attachments.into_iter().map(|x| x.into()).collect();

        let uploaded_urls =
            CloudUploadAttachment::upload_to_storage_bucket(user, channel_id, cloud_attachments)
                .await?;

        // Note: we already checked they fit 1 to 1 one level lower
        let mut i = 0;
        for uploaded_url in uploaded_urls {
            let attachment_meta = attachments_metadata.get_mut(i).unwrap();
            attachment_meta.uploaded_filename = Some(uploaded_url.upload_filename);

            i += 1;
        }

        Ok(attachments_metadata)
    }

    /// Uploads a single [PartialDiscordFileAttachment] to a Google Cloud (or similar) storage bucket.
    ///
    /// Uses [PartialDiscordFileAttachment::bulk_upload_to_storage_bucket] with only one element.
    ///
    /// **If you have multiple attachments, you should always prefer bulk_upload over this method.**
    ///
    /// Returns the same attachment with `content` set to an empty [Vec] and
    /// `uploaded_filename` properly set.
    ///
    /// For a higher level API, see [Message::send](crate::types::Message::send).
    pub async fn upload_single_to_storage_bucket(
        self,
        user: &mut ChorusUser,
        channel_id: Snowflake,
    ) -> ChorusResult<PartialDiscordFileAttachment> {
        let uploaded = Self::bulk_upload_to_storage_bucket(user, channel_id, vec![self]).await?;

        Ok(uploaded.into_iter().next().unwrap())
    }
}
