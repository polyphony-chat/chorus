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
    RequestFailed{url:String, error: String} = "An error occured while trying to GET from {url}: {error}",
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

custom_error! {
    #[derive(PartialEq, Eq)]
    pub ObserverError
    AlreadySubscribed = "Each event can only be subscribed to once."
}

custom_error! {
    /// For errors we receive from the gateway, see https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#gateway-close-event-codes;
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
