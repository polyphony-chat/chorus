// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Contains all the errors that can be returned by the library.
use custom_error::custom_error;

use crate::types::WebSocketEvent;
use chorus_macros::WebSocketEvent;

custom_error! {
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub RegistrationError
    Consent = "consent must be 'true' to register",
}

pub type ChorusResult<T> = std::result::Result<T, ChorusError>;

custom_error! {
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub ChorusError
    /// Server did not respond.
    NoResponse = "server did not respond",
    /// Reqwest returned an Error instead of a Response object.
    RequestFailed{url:String, error: String} = "an error occurred while trying to GET from {url}: {error}",
    /// Response received, however, it was not of the successful responses type. Used when no other, special case applies.
    ReceivedErrorCode{error_code: u16, error: String} = "received error code while requesting from the route: {error_code}",
    /// Used when there is likely something wrong with the instance, the request was directed to.
    CantGetInformation{error:String} = "cannot get information about the instance: {error}, something is likely wrong with the instance",
    /// The requests form body was malformed/invalid.
    InvalidFormBody{error_type: String, error:String} = "the server responded with: {error_type}: {error}",
    /// The request has not been processed by the server due to a relevant rate limit bucket being exhausted.
    RateLimited{bucket:String} = "ratelimited on bucket {bucket}",
    /// The multipart form could not be created.
    MultipartCreation{error: String} = "got an error whilst creating the form: {error}",
    /// The regular form could not be created.
    FormCreation{error: String} = "got an error whilst creating the form: {error}",
    /// The token is invalid.
    TokenExpired = "token expired, invalid or not found",
    /// No permission
    NoPermission = "you lack the permissions needed to perform this action",
    /// Resource not found
    NotFound{error: String} = "the provided resource wasn't found: {error}",
    /// Used when you, for example, try to change your spacebar account password without providing your old password for verification.
    // RAGC: could this be worded a bit better to be more concise?
    PasswordRequired = "you need to provide your current password to authenticate for this action",
    /// Malformed or unexpected response.
    InvalidResponse{error: String} = "the response is malformed and cannot be processed: {error}",
    /// Invalid, insufficient or too many arguments provided.
    InvalidArguments{error: String} = "invalid arguments were provided: {error}"
}

impl From<reqwest::Error> for ChorusError {
    fn from(value: reqwest::Error) -> Self {
        ChorusError::RequestFailed {
            url: match value.url() {
                Some(url) => url.to_string(),
                None => "None".to_string(),
            },
            error: value.to_string(),
        }
    }
}

custom_error! {
    /// For errors we receive from the gateway, see <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#gateway-close-event-codes>;
    ///
    /// Supposed to be sent as numbers, though they are sent as string most of the time?
    ///
    /// Also includes errors when initiating a connection and unexpected opcodes
    #[derive(PartialEq, Eq, Default, Clone, WebSocketEvent)]
    pub GatewayError
    // Errors we have received from the gateway
    #[default]
    /// We're not sure what went wrong. Try reconnecting?
    Unknown = "unknown error occurred, try reconnecting",
    /// You sent an invalid opcode or an invalid payload for an opcode
    UnknownOpcode = "client sent invalid opcode or invalid payload for opcode",
    /// Gateway server couldn't decode payload
    Decode = "gateway server failed to decode payload",
    /// You sent a payload prior to identifying
    NotAuthenticated = "client sent payload before identifying",
    /// The account token sent with your identify payload is invalid
    AuthenticationFailed = "account token in identify is invalid",
    /// You've already identified, no need to reauthenticate
    AlreadyAuthenticated = "client sent more than one identify payload",
    /// The sequence number sent when resuming the session was invalid. Reconnect and start a new session
    InvalidSequenceNumber = "sequence number when resuming session was invalid.",
    /// You're being rate limited
    RateLimited = "you are being rate limited",
    /// Your session timed out. Reconnect and start a new one
    SessionTimedOut = "session timed out",
    /// You sent an invalid shard when identifying
    InvalidShard = "invalid shard in identify",
    /// The session would have handled too many guilds - you are required to shard your connection in order to connect
    ShardingRequired = "sharding is required to connect",
    /// You sent an invalid Gateway version
    InvalidAPIVersion = "client sent invalid gateway version",
    /// You sent an invalid intent
    InvalidIntents = "invalid intent",
    /// You sent a disallowed intent.
    ///
    /// You may have tried to specify an intent that you have not enabled or are not approved for
    DisallowedIntents = "disallowed (not enabled / approved) intent",

    // Errors when initiating a gateway connection
    CannotConnect{error: String} = "encountered a tungstenite error: {error}",
    NonHelloOnInitiate{opcode: u8} = "received non hello on initializing connection: {opcode}",

    // Other misc errors
    UnexpectedOpcodeReceived{opcode: u8} = "unexpected opcode received: {opcode}",
}

custom_error! {
    /// Voice Gateway errors
    ///
    /// Similar to [GatewayError].
    ///
    /// See <https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-close-event-codes>;
    #[derive(Clone, Default, PartialEq, Eq, WebSocketEvent)]
    pub VoiceGatewayError
    // Errors we receive
    #[default]
    /// You sent an invalid opcode
    UnknownOpcode = "client sent invalid opcode",
    /// You sent an invalid payload in your identifying to the (Voice) Gateway
    FailedToDecodePayload = "server failed to decode payload while identifying",
    /// You sent a payload before identifying with the (Voice) Gateway
    NotAuthenticated = "client sent payload before identifying",
    /// The token you sent in your identify payload is incorrect
    AuthenticationFailed = "account token in identify is invalid",
    /// You sent more than one identify payload
    AlreadyAuthenticated = "client sent more than one identify payload",
    /// Your session is no longer valid
    SessionNoLongerValid = "session no longer valid",
    /// Your session has timed out
    SessionTimeout = "session timed out",
    /// Can't find the desired server to connect to
    ServerNotFound = "desired server not found",
    /// The server didn't recognize the protocol you sent
    UnknownProtocol = "unrecognized or unknown protocol",
    /// Channel was deleted, you were kicked, voice server changed, or the main gateway session
    /// closed.
    ///
    /// Should not attempt to reconnect.
    Disconnected = "disconnected from voice",
    /// The server crashed, try resuming
    VoiceServerCrashed = "the voice server crashed",
    /// Server failed to decrypt data
    UnknownEncryptionMode = "server failed to decrypt / unknown encryption mode",

    // Errors when initiating a gateway connection
    CannotConnect{error: String} = "encountered a tungstenite error: {error}",
    NonHelloOnInitiate{opcode: u8} = "received non hello on initializing connection: {opcode}",

    // Other misc errors
    UnexpectedOpcodeReceived{opcode: u8} = "unexpected opcode received: {opcode}",
}

custom_error! {
    /// Voice UDP errors.
    #[derive(Clone, PartialEq, Eq, WebSocketEvent)]
    pub VoiceUdpError

    // General errors
    BrokenSocket{error: String} = "Could not write / read from UDP socket: {error}",
    /// We have not yet received the necessary data to perform this operation.
    NoData = "required data not yet received",

    // Encryption errors
    EncryptionModeNotImplemented{encryption_mode: String} = "voice encryption mode {encryption_mode} is not yet implemented",
    NoKey = "could not encrypt / decrypt data: no key received yet",
    FailedEncryption = "failed to encrypt data (most likely this is an issue in chorus' nonce generation, please open an issue)",
    FailedDecryption = "failed to decrypt data (most likely this is an issue in chorus' nonce generation, please open an issue)",
    FailedNonceGeneration{error: String} = "failed to generate nonce: {error}.",

    // Errors when initiating a socket connection
    CannotBind{error: String} = "failed to bind UDP socket: {error}",
    CannotConnect{error: String} = "failed to open UDP connection: {error}",
}

