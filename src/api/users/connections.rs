use reqwest::Client;

use crate::{
    errors::ChorusResult,
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{AuthorizeConnectionReturn, AuthorizeConnectionSchema, ConnectionType, LimitType},
};

impl ChorusUser {
    /// Fetches a url that can be used for authorizing a new connection.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#authorize-user-connection>
    ///
    /// Note: it doesn't seem to be actually unauthenticated
    pub async fn authorize_connection(
        &mut self,
        connection_type: ConnectionType,
        query_parameters: AuthorizeConnectionSchema,
    ) -> ChorusResult<String> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new()
            .get(format!(
                "{}/connections/{}/authorize",
                self.belongs_to.read().unwrap().urls.api,
                connection_type_string
            ))
            // Note: ommiting this header causes a 401 Unauthorized,
            // even though discord.sex mentions it as unauthenticated
            .header("Authorization", self.token())
            .query(&query_parameters);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request
            .deserialize_response::<AuthorizeConnectionReturn>(self)
            .await
            .map(|response| response.url)
    }
}
