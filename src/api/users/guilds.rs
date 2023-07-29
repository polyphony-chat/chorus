use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::Snowflake;

impl UserMeta {
    /// Leaves a given guild.
    ///
    /// # Reference:
    /// Read <https://discord-userdoccers.vercel.app/resources/guild#leave-guild>
    // TODO: Docs: What is lurking here?
    pub async fn leave_guild(&mut self, guild_id: &Snowflake, lurking: bool) -> ChorusResult<()> {
        ChorusRequest {
            request: Client::new()
                .delete(format!(
                    "{}/users/@me/guilds/{}/",
                    self.belongs_to.borrow().urls.api,
                    guild_id
                ))
                .bearer_auth(self.token())
                .body(to_string(&lurking).unwrap()),
            limit_type: crate::api::LimitType::Guild(*guild_id),
        }
        .handle_request_as_result(self)
        .await
    }
}
