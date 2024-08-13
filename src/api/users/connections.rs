use futures_util::FutureExt;
use reqwest::Client;

use crate::{
    errors::ChorusResult,
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        AuthorizeConnectionReturn, AuthorizeConnectionSchema, Connection, ConnectionSubreddit,
        ConnectionType, CreateConnectionCallbackSchema, CreateContactSyncConnectionSchema,
        GetConnectionAccessTokenReturn, LimitType, ModifyConnectionSchema,
    },
};

impl ChorusUser {
    /// Fetches a url that can be used for authorizing a new connection.
    ///
    /// The user should then visit the url and authenticate to create the connection.
    ///
    /// # Notes
    /// This route seems to be preferred by the official infrastructure (client) to
    /// [Self::create_connection_callback].
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

    /// Creates a new connection for the current user.
    ///
    /// # Notes
    /// The official infrastructure (client) prefers the route
    /// [Self::authorize_connection] to this one.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#create-user-connection-callback>
    // TODO: When is this called? When should it be used over authorize_connection?
    pub async fn create_connection_callback(
        &mut self,
        connection_type: ConnectionType,
        json_schema: CreateConnectionCallbackSchema,
    ) -> ChorusResult<Connection> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new()
            .post(format!(
                "{}/connections/{}/callback",
                self.belongs_to.read().unwrap().urls.api,
                connection_type_string
            ))
            .header("Authorization", self.token())
            .json(&json_schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Creates a new contact sync connection for the current user.
    ///
    /// # Notes
    /// To create normal connection types, see [Self::authorize_connection] and
    /// [Self::create_connection_callback]
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#create-contact-sync-connection>
    pub async fn create_contact_sync_connection(
        &mut self,
        connection_account_id: &String,
        json_schema: CreateContactSyncConnectionSchema,
    ) -> ChorusResult<Connection> {
        let request = Client::new()
            .put(format!(
                "{}/users/@me/connections/contacts/{}",
                self.belongs_to.read().unwrap().urls.api,
                connection_account_id
            ))
            .header("Authorization", self.token())
            .json(&json_schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    // TODO: Add create_domain_connection (<https://docs.discord.sex/resources/user#create-domain-connection>)
    // It requires changing how chorus handles errors to support properly

    /// Fetches the current user's [Connection]s
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-connections>
    pub async fn get_connections(&mut self) -> ChorusResult<Vec<Connection>> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/connections",
                self.belongs_to.read().unwrap().urls.api,
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Refreshes a local user's [Connection].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#refresh-user-connection>
    pub async fn refresh_connection(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
    ) -> ChorusResult<()> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new()
            .post(format!(
                "{}/users/@me/connections/{}/{}/refresh",
                self.belongs_to.read().unwrap().urls.api,
                connection_type_string,
                connection_account_id
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.handle_request_as_result(self).await
    }

    /// Changes settings on a local user's [Connection].
    ///
    /// # Notes
    /// Not all connection types support all parameters.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#modify-user-connection>
    pub async fn modify_connection(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
        json_schema: ModifyConnectionSchema,
    ) -> ChorusResult<Connection> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new()
            .patch(format!(
                "{}/users/@me/connections/{}/{}",
                self.belongs_to.read().unwrap().urls.api,
                connection_type_string,
                connection_account_id
            ))
            .header("Authorization", self.token())
            .json(&json_schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }

    /// Deletes a local user's [Connection].
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#delete-user-connection>
    pub async fn delete_connection(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
    ) -> ChorusResult<()> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new()
            .delete(format!(
                "{}/users/@me/connections/{}/{}",
                self.belongs_to.read().unwrap().urls.api,
                connection_type_string,
                connection_account_id
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.handle_request_as_result(self).await
    }

    /// Returns a new access token for the given connection.
    ///
    /// Only available for [ConnectionType::Twitch], [ConnectionType::YouTube] and [ConnectionType::Spotify] connections.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-connection-access-token>
    pub async fn get_connection_access_token(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
    ) -> ChorusResult<String> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new()
            .get(format!(
                "{}/users/@me/connections/{}/{}/access-token",
                self.belongs_to.read().unwrap().urls.api,
                connection_type_string,
                connection_account_id
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request
            .deserialize_response::<GetConnectionAccessTokenReturn>(self)
            .await
            .map(|res| res.access_token)
    }

    /// Fetches a list of [subreddits](crate::types::ConnectionSubreddit)
    /// the connected account moderates.
    ///
    /// Only available for [ConnectionType::Reddit] connections.
    ///
    /// # Reference
    /// See <https://docs.discord.sex/resources/user#get-user-connection-subreddits>
    pub async fn get_connection_subreddits(
        &mut self,
        connection_account_id: &String,
    ) -> ChorusResult<Vec<ConnectionSubreddit>> {
        let request = Client::new()
            .get(format!(
                "{}/users/@me/connections/reddit/{}/subreddits",
                self.belongs_to.read().unwrap().urls.api,
                connection_account_id
            ))
            .header("Authorization", self.token());

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        };

        chorus_request.deserialize_response(self).await
    }
}
