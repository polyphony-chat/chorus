// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use futures_util::FutureExt;
use reqwest::Client;

use crate::{
    errors::{ChorusError, ChorusResult},
    instance::ChorusUser,
    ratelimiter::ChorusRequest,
    types::{
        AuthorizeConnectionReturn, AuthorizeConnectionSchema, Connection, ConnectionSubreddit,
        ConnectionType, CreateConnectionCallbackSchema, CreateContactSyncConnectionSchema,
        CreateDomainConnectionError, CreateDomainConnectionReturn, GetConnectionAccessTokenReturn,
        LimitType, ModifyConnectionSchema,
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
    /// See <https://docs.discord.food/resources/user#authorize-user-connection>
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
            .query(&query_parameters);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);
        // Note: ommiting authorization causes a 401 Unauthorized,
        // even though discord.food mentions it as unauthenticated

        chorus_request
            .send_and_deserialize_response::<AuthorizeConnectionReturn>(self)
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
    /// See <https://docs.discord.food/resources/user#create-user-connection-callback>
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
            .json(&json_schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Creates a new contact sync connection for the current user.
    ///
    /// # Notes
    /// To create normal connection types, see [Self::authorize_connection] and
    /// [Self::create_connection_callback]
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#create-contact-sync-connection>
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
            .json(&json_schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Creates a new domain connection for the current user.
    ///
    /// This route has two possible successful return values:
    /// [CreateDomainConnectionReturn::Ok] and [CreateDomainConnectionReturn::ProofNeeded]
    ///
    /// To properly handle both, please see their respective documentation pages.
    ///
    /// # Notes
    /// To create normal connection types, see [Self::authorize_connection] and
    /// [Self::create_connection_callback]
    ///
    /// As of 2024/08/21, Spacebar does not yet implement this endpoint.
    ///
    /// # Examples
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// # use chorus::{instance::ChorusUser, types::CreateDomainConnectionReturn};
    /// # mod tests::common;
    /// # let mut bundle = tests::common::setup().await;
    /// let domain = "example.com".to_string();
    ///
    /// let user: ChorusUser; // Get this by registering / logging in
    /// # let user = bundle.user;
    ///
    /// let result = user.create_domain_connection(&domain).await;
    ///
    /// if let Ok(returned) = result {
    ///     match returned {
    ///         CreateDomainConnectionReturn::ProofNeeded(proof) => {
    ///             println!("Additional proof needed!");
    ///             println!("Either:");
    ///             println!("");
    ///             println!("- create a DNS TXT record with the name _discord.{domain} and content {proof}");
    ///             println!("or");
    ///             println!("- create a file at https://{domain}/.well-known/discord with the content {proof}");
    ///             // Once the user has added the proof, retry calling the endpoint
    ///         }
    ///         CreateDomainConnectionReturn::Ok(connection) => {
    ///             println!("Successfulyl created connection! {:?}", connection);
    ///         }
    ///     }
    /// } else {
    ///     println!("Failed to create connection: {:?}", result);
    /// }
    /// # tests::common::teardown(bundle).await;
    /// # })
    /// ```
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#create-domain-connection>
    pub async fn create_domain_connection(
        &mut self,
        domain: &String,
    ) -> ChorusResult<CreateDomainConnectionReturn> {
        let request = Client::new().post(format!(
            "{}/users/@me/connections/domain/{}",
            self.belongs_to.read().unwrap().urls.api,
            domain
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        let result = chorus_request.send(self).await;

        match result {
            Ok(response) => {
                let connection: Connection = ChorusRequest::deserialize_response(response).await?;

                Ok(CreateDomainConnectionReturn::Ok(connection))
            }
            Err(e) => {
                match e {
                    ChorusError::ReceivedError {
                        ref error,
                        ref response_text,
                    } => {
                        // TODO: maybe there is a JSON code for this?
                        if error.http_status.as_u16() == 400 {
                            let try_deserialize: Result<
                                CreateDomainConnectionError,
                                serde_json::Error,
                            > = serde_json::from_str(response_text);

                            if let Ok(deserialized) = try_deserialize {
                                return Ok(CreateDomainConnectionReturn::ProofNeeded(
                                    deserialized.proof,
                                ));
                            }
                        }

                        Err(e)
                    }
                    _ => Err(e),
                }
            }
        }
    }

    /// Fetches the current user's [Connection]s
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#get-user-connections>
    pub async fn get_connections(&mut self) -> ChorusResult<Vec<Connection>> {
        let request = Client::new().get(format!(
            "{}/users/@me/connections",
            self.belongs_to.read().unwrap().urls.api,
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Refreshes a local user's [Connection].
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#refresh-user-connection>
    pub async fn refresh_connection(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
    ) -> ChorusResult<()> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new().post(format!(
            "{}/users/@me/connections/{}/{}/refresh",
            self.belongs_to.read().unwrap().urls.api,
            connection_type_string,
            connection_account_id
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_handle_as_result(self).await
    }

    /// Changes settings on a local user's [Connection].
    ///
    /// # Notes
    /// Not all connection types support all parameters.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#modify-user-connection>
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
            .json(&json_schema);

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }

    /// Deletes a local user's [Connection].
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#delete-user-connection>
    pub async fn delete_connection(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
    ) -> ChorusResult<()> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new().delete(format!(
            "{}/users/@me/connections/{}/{}",
            self.belongs_to.read().unwrap().urls.api,
            connection_type_string,
            connection_account_id
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_handle_as_result(self).await
    }

    /// Returns a new access token for the given connection.
    ///
    /// Only available for [ConnectionType::Twitch], [ConnectionType::YouTube] and [ConnectionType::Spotify] connections.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#get-user-connection-access-token>
    pub async fn get_connection_access_token(
        &mut self,
        connection_type: ConnectionType,
        connection_account_id: &String,
    ) -> ChorusResult<String> {
        let connection_type_string = serde_json::to_string(&connection_type)
            .expect("Failed to serialize connection type!")
            .replace('"', "");

        let request = Client::new().get(format!(
            "{}/users/@me/connections/{}/{}/access-token",
            self.belongs_to.read().unwrap().urls.api,
            connection_type_string,
            connection_account_id
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request
            .send_and_deserialize_response::<GetConnectionAccessTokenReturn>(self)
            .await
            .map(|res| res.access_token)
    }

    /// Fetches a list of [subreddits](crate::types::ConnectionSubreddit)
    /// the connected account moderates.
    ///
    /// Only available for [ConnectionType::Reddit] connections.
    ///
    /// # Reference
    /// See <https://docs.discord.food/resources/user#get-user-connection-subreddits>
    pub async fn get_connection_subreddits(
        &mut self,
        connection_account_id: &String,
    ) -> ChorusResult<Vec<ConnectionSubreddit>> {
        let request = Client::new().get(format!(
            "{}/users/@me/connections/reddit/{}/subreddits",
            self.belongs_to.read().unwrap().urls.api,
            connection_account_id
        ));

        let chorus_request = ChorusRequest {
            request,
            limit_type: LimitType::default(),
        }
        .with_headers_for(self);

        chorus_request.send_and_deserialize_response(self).await
    }
}
