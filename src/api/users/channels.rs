// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::{
    errors::ChorusResult,
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{Channel, LimitType, PrivateChannelCreateSchema},
};

impl ChorusUser {
    /// Fetches a list of private channels the user is in.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/channel#get-private-channels>
    pub async fn get_private_channels(&mut self) -> ChorusResult<Vec<Channel>> {
        let url = format!(
            "{}/users/@me/channels",
            self.belongs_to.read().unwrap().urls.api
        );
        ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self)
        .send_and_deserialize_response::<Vec<Channel>>(self)
        .await
    }

    /// Creates a DM channel or group DM channel.
    ///
    /// One recipient creates or returns an existing DM channel,
    /// none or multiple recipients create a group DM channel.
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/channel#create-private-channel>
    pub async fn create_private_channel(
        &mut self,
        create_private_channel_schema: PrivateChannelCreateSchema,
    ) -> ChorusResult<Channel> {
        let url = format!(
            "{}/users/@me/channels",
            self.belongs_to.read().unwrap().urls.api
        );
        ChorusRequest {
            request: Client::new().post(url).json(&create_private_channel_schema),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self)
        .send_and_deserialize_response::<Channel>(self)
        .await
    }
}
