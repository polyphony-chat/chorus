// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Contains all the errors that can be returned by the library.
use custom_error::custom_error;

use crate::types::WebSocketEvent;

custom_error! {
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub RegistrationError
    Consent = "Consent must be 'true' to register.",
}

pub type ChorusResult<T> = std::result::Result<T, ChorusError>;

custom_error! {
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub ChorusError
    /// Server did not respond.
    NoResponse = "Did not receive a response from the Server.",
    /// Reqwest returned an Error instead of a Response object.
    RequestFailed{url:String, error: String} = "An error occurred while trying to GET from {url}: {error}",
    /// Response received, however, it was not of the successful responses type. Used when no other, special case applies.
    ReceivedErrorCode{error_code: u16, error: String} = "Received the following error code while requesting from the route: {error_code}",
    /// Used when there is likely something wrong with the instance, the request was directed to.
    CantGetInformation{error:String} = "Something seems to be wrong with the instance. Cannot get information about the instance: {error}",
    /// The requests form body was malformed/invalid.
    InvalidFormBody{error_type: String, error:String} = "The server responded with: {error_type}: {error}",
    /// The request has not been processed by the server due to a relevant rate limit bucket being exhausted.
    RateLimited{bucket:String} = "Ratelimited on Bucket {bucket}",
    /// The multipart form could not be created.
    MultipartCreation{error: String} = "Got an error whilst creating the form: {error}",
    /// The regular form could not be created.
    FormCreation{error: String} = "Got an error whilst creating the form: {error}",
    /// The token is invalid.
    TokenExpired = "Token expired, invalid or not found.",
    /// No permission
    NoPermission = "You do not have the permissions needed to perform this action.",
    /// Resource not found
    NotFound{error: String} = "The provided resource hasn't been found: {error}",
    /// Used when you, for example, try to change your spacebar account password without providing your old password for verification.
    PasswordRequired = "You need to provide your current password to authenticate for this action.",
    /// Malformed or unexpected response.
    InvalidResponse{error: String} = "The response is malformed and cannot be processed. Error: {error}",
    /// Invalid, insufficient or too many arguments provided.
    InvalidArguments{error: String} = "Invalid arguments were provided. Error: {error}"
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
    #[derive(PartialEq, Eq)]
    pub ObserverError
    AlreadySubscribed = "Each event can only be subscribed to once."
}

custom_error! {
    /// For errors we receive from the gateway, see <https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#gateway-close-event-codes>;
    ///
    /// Supposed to be sent as numbers, though they are sent as string most of the time?
    ///
    /// Also includes errors when initiating a connection and unexpected opcodes
    #[derive(PartialEq, Eq, Default, Clone)]
    pub GatewayError
    // Errors we have received from the gateway
    #[default]
    Unknown = "We're not sure what went wrong. Try reconnecting?",
    UnknownOpcode = "You sent an invalid Gateway opcode or an invalid payload for an opcode",
    Decode = "Gateway server couldn't decode payload",
    NotAuthenticated = "You sent a payload prior to identifying",
    AuthenticationFailed = "The account token sent with your identify payload is invalid",
    AlreadyAuthenticated = "You've already identified, no need to reauthenticate",
    InvalidSequenceNumber = "The sequence number sent when resuming the session was invalid. Reconnect and start a new session",
    RateLimited = "You are being rate limited!",
    SessionTimedOut = "Your session timed out. Reconnect and start a new one",
    InvalidShard = "You sent us an invalid shard when identifying",
    ShardingRequired = "The session would have handled too many guilds - you are required to shard your connection in order to connect",
    InvalidAPIVersion = "You sent an invalid Gateway version",
    InvalidIntents = "You sent an invalid intent",
    DisallowedIntents = "You sent a disallowed intent. You may have tried to specify an intent that you have not enabled or are not approved for",

    // Errors when initiating a gateway connection
    CannotConnect{error: String} = "Cannot connect due to a tungstenite error: {error}",
    NonHelloOnInitiate{opcode: u8} = "Received non hello on initial gateway connection ({opcode}), something is definitely wrong",

    // Other misc errors
    UnexpectedOpcodeReceived{opcode: u8} = "Received an opcode we weren't expecting to receive: {opcode}",
}

impl WebSocketEvent for GatewayError {}

custom_error! {
    /// Voice Gateway errors
    ///
    /// Similar to [GatewayError].
    ///
    /// See <https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-close-event-codes>;
    #[derive(Clone, Default, PartialEq, Eq)]
    pub VoiceGatewayError
    // Errors we receive
    #[default]
    UnknownOpcode = "You sent an invalid opcode",
    FailedToDecodePayload = "You sent an invalid payload in your identifying to the (Voice) Gateway",
    NotAuthenticated = "You sent a payload before identifying with the (Voice) Gateway",
    AuthenticationFailed = "The token you sent in your identify payload is incorrect",
    AlreadyAuthenticated = "You sent more than one identify payload",
    SessionNoLongerValid = "Your session is no longer valid",
    SessionTimeout = "Your session has timed out",
    ServerNotFound = "We can't find the server you're trying to connect to",
    UnknownProtocol = "We didn't recognize the protocol you sent",
    Disconnected = "Channel was deleted, you were kicked, voice server changed, or the main gateway session was dropped. Should not reconnect.",
    VoiceServerCrashed = "The server crashed, try resuming",
    UnknownEncryptionMode = "Server failed to decrypt data",

    // Errors when initiating a gateway connection
    CannotConnect{error: String} = "Cannot connect due to a tungstenite error: {error}",
    NonHelloOnInitiate{opcode: u8} = "Received non hello on initial gateway connection ({opcode}), something is definitely wrong",

    // Other misc errors
    UnexpectedOpcodeReceived{opcode: u8} = "Received an opcode we weren't expecting to receive: {opcode}",
}

impl WebSocketEvent for VoiceGatewayError {}

custom_error! {
    /// Voice UDP errors.
    #[derive(Clone, PartialEq, Eq)]
    pub VoiceUdpError

    // General errors
    BrokenSocket{error: String} = "Could not write / read from UDP socket: {error}",
    NoData = "We have not set received the necessary data to perform this operation.",

    // Encryption errors
    EncryptionModeNotImplemented{encryption_mode: String} = "Voice encryption mode {encryption_mode} is not yet implemented.",
    NoKey = "Tried to encrypt / decrypt rtp data, but no key has been received yet",
    FailedEncryption = "Tried to encrypt rtp data, but failed. Most likely this is an issue chorus' nonce generation. Please open an issue on the chorus github: https://github.com/polyphony-chat/chorus/issues/new",
    FailedDecryption = "Tried to decrypt rtp data, but failed. Most likely this is an issue chorus' nonce generation. Please open an issue on the chorus github: https://github.com/polyphony-chat/chorus/issues/new",
    FailedNonceGeneration{error: String} = "Tried to generate nonce, but failed due to error: {error}.",

    // Errors when initiating a socket connection
    CannotBind{error: String} = "Cannot bind socket due to a UDP error: {error}",
    CannotConnect{error: String} = "Cannot connect due to a UDP error: {error}",
}

impl WebSocketEvent for VoiceUdpError {}
