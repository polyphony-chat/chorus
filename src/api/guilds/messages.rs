// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::types::{Guild, Message, MessageSearchQuery, Snowflake};

impl Guild {
    /// Returns messages without the reactions key that match a search query in the guild.
    /// The messages that are direct results will have an extra hit key set to true.
    /// If operating on a guild channel, this endpoint requires the `READ_MESSAGE_HISTORY`
    /// permission to be present on the current user.
    ///
    /// If the guild/channel you are searching is not yet indexed, the endpoint will return a 202 accepted response.
    /// In this case, the method will return a [`ChorusError::InvalidResponse`] error.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/message#search-messages>
    pub async fn search_messages(
        guild_id: Snowflake,
        query: MessageSearchQuery,
        user: &mut ChorusUser,
    ) -> ChorusResult<Vec<Message>> {
        Message::search(
            crate::types::MessageSearchEndpoint::GuildChannel(guild_id),
            query,
            user,
        )
        .await
    }
}
