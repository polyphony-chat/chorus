use custom_error::custom_error;

custom_error! {
    #[derive(PartialEq, Eq)]
    pub FieldFormatError
    PasswordError = "Password must be between 1 and 72 characters.",
    UsernameError = "Username must be between 2 and 32 characters.",
    ConsentError = "Consent must be 'true' to register.",
    EmailError = "The provided email address is in an invalid format.",
}

custom_error! {
    #[derive(PartialEq, Eq)]
    pub ChorusLibError
    NoResponse = "Did not receive a response from the Server.",
    RequestErrorError{url:String, error:String} = "An error occured while trying to GET from {url}: {error}",
    ReceivedErrorCodeError{error_code:String} = "Received the following error code while requesting from the route: {error_code}",
    CantGetInfoError{error:String} = "Something seems to be wrong with the instance. Cannot get information about the instance: {error}",
    InvalidFormBodyError{error_type: String, error:String} = "The server responded with: {error_type}: {error}",
    RateLimited{bucket:String} = "Ratelimited on Bucket {bucket}",
    MultipartCreationError{error: String} = "Got an error whilst creating the form: {error}",
    FormCreationError{error: String} = "Got an error whilst creating the form: {error}",
    TokenExpired = "Token expired, invalid or not found.",
    NoPermission = "You do not have the permissions needed to perform this action.",
    NotFound{error: String} = "The provided resource hasn't been found: {error}",
    PasswordRequiredError = "You need to provide your current password to authenticate for this action.",
    InvalidResponseError{error: String} = "The response is malformed and cannot be processed. Error: {error}",
    InvalidArgumentsError{error: String} = "Invalid arguments were provided. Error: {error}"
}

custom_error! {
    #[derive(PartialEq, Eq)]
    pub ObserverError
    AlreadySubscribedError = "Each event can only be subscribed to once."
}

custom_error! {
    /// For errors we receive from the gateway, see https://discord-userdoccers.vercel.app/topics/opcodes-and-status-codes#gateway-close-event-codes;
    ///
    /// Supposed to be sent as numbers, though they are sent as string most of the time?
    ///
    /// Also includes errors when initiating a connection and unexpected opcodes
    #[derive(PartialEq, Eq)]
    pub GatewayError
    // Errors we have received from the gateway
    UnknownError = "We're not sure what went wrong. Try reconnecting?",
    UnknownOpcodeError = "You sent an invalid Gateway opcode or an invalid payload for an opcode",
    DecodeError = "Gateway server couldn't decode payload",
    NotAuthenticatedError = "You sent a payload prior to identifying",
    AuthenticationFailedError = "The account token sent with your identify payload is invalid",
    AlreadyAuthenticatedError = "You've already identified, no need to reauthenticate",
    InvalidSequenceNumberError = "The sequence number sent when resuming the session was invalid. Reconnect and start a new session",
    RateLimitedError = "You are being rate limited!",
    SessionTimedOutError = "Your session timed out. Reconnect and start a new one",
    InvalidShardError = "You sent us an invalid shard when identifying",
    ShardingRequiredError = "The session would have handled too many guilds - you are required to shard your connection in order to connect",
    InvalidAPIVersionError = "You sent an invalid Gateway version",
    InvalidIntentsError = "You sent an invalid intent",
    DisallowedIntentsError = "You sent a disallowed intent. You may have tried to specify an intent that you have not enabled or are not approved for",

    // Errors when initiating a gateway connection
    CannotConnectError{error: String} = "Cannot connect due to a tungstenite error: {error}",
    NonHelloOnInitiateError{opcode: u8} = "Received non hello on initial gateway connection ({opcode}), something is definitely wrong",

    // Other misc errors
    UnexpectedOpcodeReceivedError{opcode: u8} = "Received an opcode we weren't expecting to receive: {opcode}",
}
