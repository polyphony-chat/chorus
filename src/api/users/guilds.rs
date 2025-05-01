// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use reqwest::Client;
use serde_json::to_string;

use crate::errors::ChorusResult;
use crate::instance::ChorusUser;
use crate::ratelimiter::ChorusRequest;
use crate::types::{GetUserGuildsSchema, Guild, GuildLeaveSchema, LimitType, Snowflake};

impl ChorusUser {
    /// Leaves a given guild.
    ///
    /// Fires a [crate::types::GuildDelete] and [crate::types::GuildMemberRemove] event
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/guild#leave-guild>
    // TODO: Docs: What is "lurking" here?
    // It is documented as "Whether the user is lurking in the guild",
    // but that says nothing about what this field actually does / means
    pub async fn leave_guild(
        &mut self,
        guild_id: &Snowflake,
        lurking: Option<bool>,
    ) -> ChorusResult<()> {
        ChorusRequest {
            request: Client::new()
                .delete(format!(
                    "{}/users/@me/guilds/{}",
                    self.belongs_to.read().unwrap().urls.api,
                    guild_id
                ))
                .json(&GuildLeaveSchema { lurking }),
            limit_type: LimitType::Guild(*guild_id),
        }
        .send_and_handle_as_result(self)
        .await
    }

    /// Returns a list of user guild objects representing the guilds the current user is a member of.
    ///
    /// This endpoint returns 200 guilds by default (which is the maximum a user account can join)
    ///
    /// All parameters are optional
    ///
    /// # Reference:
    /// See <https://docs.discord.food/resources/guild#get-user-guilds>
    pub async fn get_guilds(
        &mut self,
        query: Option<GetUserGuildsSchema>,
    ) -> ChorusResult<Vec<Guild>> {
        let query_parameters = {
            if let Some(query_some) = query {
                query_some.to_query()
            } else {
                Vec::new()
            }
        };

        let url = format!(
            "{}/users/@me/guilds",
            self.belongs_to.read().unwrap().urls.api,
        );
        let chorus_request = ChorusRequest {
            request: Client::new().get(url).query(&query_parameters),

            limit_type: LimitType::Global,
        }
        .with_headers_for(self);

        chorus_request
            .send_and_deserialize_response::<Vec<Guild>>(self)
            .await
    }

    /// Returns a list of partial guild objects representing non-previewable guilds the user has
    /// pending join requests for.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/guild#get-join-request-guilds>
    pub async fn get_join_request_guilds(&mut self) -> ChorusResult<Vec<Guild>> {
        let url = format!(
            "{}/users/@me/join-request-guilds",
            self.belongs_to.read().unwrap().urls.api,
        );

        let chorus_request = ChorusRequest {
            request: Client::new().get(url),
            limit_type: LimitType::Global,
        }
        .with_headers_for(self);

        chorus_request
            .send_and_deserialize_response::<Vec<Guild>>(self)
            .await
    }
}
