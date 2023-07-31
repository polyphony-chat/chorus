use reqwest::Client;
use serde_json::to_string;

use crate::{
    api::LimitType,
    errors::ChorusResult,
    instance::UserMeta,
    ratelimiter::ChorusRequest,
    types::{Channel, PrivateChannelCreateSchema},
};

impl UserMeta {
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
        let url = format!("{}/users/@me/channels", self.belongs_to.borrow().urls.api);
        ChorusRequest {
            request: Client::new()
                .post(url)
                .bearer_auth(self.token())
                .body(to_string(&create_private_channel_schema).unwrap()),
            limit_type: LimitType::Global,
        }
        .deserialize_response::<Channel>(self)
        .await
    }
}
