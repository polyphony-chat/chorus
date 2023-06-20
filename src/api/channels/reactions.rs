use reqwest::Client;

use crate::{
    api::{handle_request, handle_request_as_result},
    errors::ChorusLibError,
    instance::UserMeta,
    types,
};

/**
Useful metadata for working with [`types::Reaction`], bundled together nicely.
 */
pub struct ReactionMeta {
    pub message_id: types::Snowflake,
    pub channel_id: types::Snowflake,
}

impl ReactionMeta {
    /**
    Deletes all reactions for a message.
    This endpoint requires the `MANAGE_MESSAGES` permission to be present on the current user.

    # Arguments
    * `user` - A mutable reference to a [`UserMeta`] instance.

    # Returns
    A `Result` [`()`] [`crate::errors::ChorusLibError`] if something went wrong.
    Fires a `Message Reaction Remove All` Gateway event.

    # Reference
    See [https://discord.com/developers/docs/resources/channel#delete-all-reactions](https://discord.com/developers/docs/resources/channel#delete-all-reactions)
     */
    pub async fn delete_all(&self, user: &mut UserMeta) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }

    /**
    Gets a list of users that reacted with a specific emoji to a message.

    # Arguments
    * `emoji` - A string slice containing the emoji to search for. The emoji must be URL Encoded or
    the request will fail with 10014: Unknown Emoji. To use custom emoji, you must encode it in the
    format name:id with the emoji name and emoji id.
    * `user` - A mutable reference to a [`UserMeta`] instance.

    # Returns
    A Result that is [`Err(crate::errors::ChorusLibError)`] if something went wrong.

    # Reference
    See [https://discord.com/developers/docs/resources/channel#get-reactions](https://discord.com/developers/docs/resources/channel#get-reactions)
     */
    pub async fn get(&self, emoji: &str, user: &mut UserMeta) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let request = Client::new().get(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }

    /**
    Deletes all the reactions for a given `emoji` on a message. This endpoint requires the
    MANAGE_MESSAGES permission to be present on the current user.

    # Arguments
    * `emoji` - A string slice containing the emoji to delete. The `emoji` must be URL Encoded or
    the request will fail with 10014: Unknown Emoji. To use custom emoji, you must encode it in the
    format name:id with the emoji name and emoji id.
    * `user` - A mutable reference to a [`UserMeta`] instance.

    # Returns
    A Result that is [`Err(crate::errors::ChorusLibError)`] if something went wrong.
    Fires a `Message Reaction Remove Emoji` Gateway event.

    # Reference
    See [https://discord.com/developers/docs/resources/channel#delete-all-reactions-for-emoji](https://discord.com/developers/docs/resources/channel#delete-all-reactions-for-emoji)
     */
    pub async fn delete_emoji(
        &self,
        emoji: &str,
        user: &mut UserMeta,
    ) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }

    /**
    Create a reaction for the message.

    This endpoint requires the READ_MESSAGE_HISTORY permission
    to be present on the current user. Additionally, if nobody else has reacted to the message using
    this emoji, this endpoint requires the ADD_REACTIONS permission to be present on the current
    user.
     # Arguments
    * `emoji` - A string slice containing the emoji to delete. The `emoji` must be URL Encoded or
    the request will fail with 10014: Unknown Emoji. To use custom emoji, you must encode it in the
    format name:id with the emoji name and emoji id.
    * `user` - A mutable reference to a [`UserMeta`] instance.

    # Returns
    A `Result` containing [`()`] or a [`crate::errors::ChorusLibError`].

    # Reference
    See [https://discord.com/developers/docs/resources/channel#create-reaction](https://discord.com/developers/docs/resources/channel#create-reaction)
     */
    pub async fn create(&self, emoji: &str, user: &mut UserMeta) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me/",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let request = Client::new().put(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }

    /**
    Delete a reaction the current user has made for the message.

    # Arguments
    * `emoji` - A string slice containing the emoji to delete. The `emoji` must be URL Encoded or
    the request will fail with 10014: Unknown Emoji. To use custom emoji, you must encode it in the
    format name:id with the emoji name and emoji id.
    * `user` - A mutable reference to a [`UserMeta`] instance.

    # Returns
    A `Result` containing [`()`] or a [`crate::errors::ChorusLibError`].
    Fires a `Message Reaction Remove` Gateway event.

    # Reference
    See [https://discord.com/developers/docs/resources/channel#delete-own-reaction](https://discord.com/developers/docs/resources/channel#delete-own-reaction)
     */
    pub async fn remove(&self, emoji: &str, user: &mut UserMeta) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me/",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }

    /**
    Delete a user's reaction to a message.

    This endpoint requires the MANAGE_MESSAGES permission to be present on the current user.

    # Arguments
    * `user_id` - A string slice containing the ID of the user whose reaction is to be deleted.
    * `emoji` - A string slice containing the emoji to delete. The `emoji` must be URL Encoded or
    the request will fail with 10014: Unknown Emoji. To use custom emoji, you must encode it in the
    format name:id with the emoji name and emoji id.
    * `user` - A mutable reference to a [`UserMeta`] instance.

    # Returns
    A `Result` containing [`()`] or a [`crate::errors::ChorusLibError`].
    Fires a Message Reaction Remove Gateway event.

    # Reference
    See [https://discord.com/developers/docs/resources/channel#delete-own-reaction](https://discord.com/developers/docs/resources/channel#delete-own-reaction)
     */
    pub async fn delete_user(
        &self,
        user_id: &str,
        emoji: &str,
        user: &mut UserMeta,
    ) -> Result<(), ChorusLibError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/{}",
            user.belongs_to.borrow().urls.api,
            self.channel_id,
            self.message_id,
            emoji,
            user_id
        );
        let request = Client::new().delete(url).bearer_auth(user.token());
        handle_request_as_result(request, user, crate::api::limits::LimitType::Channel).await
    }
}
