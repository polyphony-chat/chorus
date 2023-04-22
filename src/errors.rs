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
    pub InstanceServerError
    NoResponse = "Did not receive a response from the Server.",
    RequestErrorError{url:String, error:String} = "An error occured while trying to GET from {url}: {error}",
    ReceivedErrorCodeError{error_code:String} = "Received the following error code while requesting from the route: {error_code}",
    CantGetInfoError{error:String} = "Something seems to be wrong with the instance. Cannot get information about the instance: {error}",
    InvalidFormBodyError{error:String} = "The server responded with: {error}",
}
