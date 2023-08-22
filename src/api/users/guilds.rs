use reqwest::Client;
use serde_json::to_string;

use crate::api::LimitType;
use crate::errors::ChorusResult;
use crate::instance::UserMeta;
use crate::ratelimiter::ChorusRequest;
use crate::types::{GetUserGuildSchema, Guild, Snowflake};

impl UserMeta {
    /// Leaves a given guild.
    ///
    /// # Reference:
    /// See <https://discord-userdoccers.vercel.app/resources/guild#leave-guild>
    // TODO: Docs: What is "lurking" here?
    // It is documented as "Whether the user is lurking in the guild",
    // but that says nothing about what this field actually does / means
    pub async fn leave_guild(&mut self, guild_id: &Snowflake, lurking: bool) -> ChorusResult<()> {
        ChorusRequest {
            request: Client::new()
                .delete(format!(
                    "{}/users/@me/guilds/{}",
                    self.belongs_to.borrow().urls.api,
                    guild_id
                ))
                .header("Authorization", self.token())
                .header("Content-Type", "application/json")
                .body(to_string(&lurking).unwrap()),
            limit_type: crate::api::LimitType::Guild(*guild_id),
        }
        .handle_request_as_result(self)
        .await
    }

    /// Returns a list of user guild objects representing the guilds the current user is a member of.
    /// This endpoint returns 200 guilds by default
    ///
    /// # Reference:
    /// See: <https://discord-userdoccers.vercel.app/resources/guild#get-user-guilds>
    pub async fn get_guilds(
        &mut self,
        query: Option<GetUserGuildSchema>,
    ) -> ChorusResult<Vec<Guild>> {
        let url = format!("{}/users/@me/guilds", self.belongs_to.borrow().urls.api,);
        let chorus_request = ChorusRequest {
            request: Client::new()
                .get(url)
                .header("Authorization", self.token())
                .header("Content-Type", "application/json")
                .body(to_string(&query).unwrap()),

            limit_type: LimitType::Global,
        };
        chorus_request
            .deserialize_response::<Vec<Guild>>(self)
            .await
    }
}
