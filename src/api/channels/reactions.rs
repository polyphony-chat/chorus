use reqwest::Client;

use crate::{
    api::LimitType,
    errors::ChorusResult,
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{self, PublicUser, Snowflake},
};

/// Useful metadata for working with [`types::Reaction`], bundled together nicely.
pub struct ReactionMeta {
    pub message_id: types::Snowflake,
    pub channel_id: types::Snowflake,
}

impl ReactionMeta {
    /// Deletes all reactions for a message.
    ///
    /// This endpoint requires the [`MANAGE_MESSAGES`](crate::types::PermissionFlags::MANAGE_MESSAGES) permission.
    ///
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/channel#delete-all-reactions>
    pub async fn delete_all(&self, user: &mut UserMeta) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Gets a list of users that reacted with a specific emoji to a message.
    ///
    /// The emoji must be URL Encoded or the request will fail with 10014: Unknown Emoji.
    /// To use custom emoji, the format of the emoji string must be name:id.
    ///
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/channel#get-reactions>
    pub async fn get(&self, emoji: &str, user: &mut UserMeta) -> ChorusResult<Vec<PublicUser>> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.channel_id),
        };
        chorus_request
            .deserialize_response::<Vec<PublicUser>>(user)
            .await
    }

    /// Deletes all the reactions for a given emoji on a message.
    ///
    /// This endpoint requires the [`MANAGE_MESSAGES`](crate::types::PermissionFlags::MANAGE_MESSAGES) permission.
    ///
    /// The emoji must be URL Encoded or the request will fail with 10014: Unknown Emoji.
    /// To use custom emoji, the format of the emoji string must be name:id.
    ///
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/channel#delete-all-reactions-for-emoji>
    pub async fn delete_emoji(&self, emoji: &str, user: &mut UserMeta) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Create a reaction on a message.
    ///
    /// This endpoint requires the [`READ_MESSAGE_HISTORY`](crate::types::PermissionFlags::READ_MESSAGE_HISTORY) permission.
    ///
    /// Additionally, if nobody else has reacted to the message using this emoji,
    /// this endpoint requires the [`ADD_REACTIONS`](crate::types::PermissionFlags::ADD_REACTIONS) permission.
    ///
    /// The emoji must be URL Encoded or the request will fail with 10014: Unknown Emoji.
    /// To use custom emoji, the format of the emoji string must be `name:id`.
    ///
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/channel#create-reaction>
    pub async fn create(&self, emoji: &str, user: &mut UserMeta) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let chorus_request = ChorusRequest {
            request: Client::new().put(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Deletes a reaction the current user has made to the message.
    ///
    /// The reaction emoji must be URL Encoded or the request will fail with 10014: Unknown Emoji.
    /// To use custom emoji, the format of the emoji string must be name:id.
    ///
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/channel#delete-own-reaction>
    pub async fn remove(&self, emoji: &str, user: &mut UserMeta) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }

    /// Deletes a user's reaction to a message.
    ///
    /// This endpoint requires the [`MANAGE_MESSAGES`](crate::types::PermissionFlags::MANAGE_MESSAGES) permission.
    ///
    /// The reaction emoji must be URL Encoded or the request will fail with 10014: Unknown Emoji.
    /// To use custom emoji, the format of the emoji string must be name:id.
    ///
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/channel#delete-user-reaction>
    pub async fn delete_user(
        &self,
        user_id: Snowflake,
        emoji: &str,
        user: &mut UserMeta,
    ) -> ChorusResult<()> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/{}",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji,
            user_id
        );
        let chorus_request = ChorusRequest {
            request: Client::new().delete(url).bearer_auth(user.token()),
            limit_type: LimitType::Channel(self.channel_id),
        };
        chorus_request.handle_request_as_result(user).await
    }
}
