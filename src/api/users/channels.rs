// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::ChorusResult,
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{Channel, LimitType, PrivateChannelCreateSchema},
};

impl ChorusUser {
    /// Creates a DM channel or group DM channel.
    ///
    /// One recipient creates or returns an existing DM channel,
    /// none or multiple recipients create a group DM channel.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/channel#create-private-channel>
    pub async fn create_private_channel(
        &mut self,
        create_private_channel_schema: PrivateChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        let url = format!(
            "{}/users/@me/channels",
            self.belongs_to.read().unwrap().urls.api
        );
        ChorusRequest {
            request: Client::new()
                .post(url)
                .header("Authorization", self.token())
                .header("Content-Type", "application/json")
                .body(to_string(&create_private_channel_schema).unwrap()),
            limit_type: LimitType::Global,
        }
        .deserialize_response::<Channel>(self)
        .await
    }
}
