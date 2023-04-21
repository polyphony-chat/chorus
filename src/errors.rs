use custom_error::custom_error;

custom_error! {
    #[derive(PartialEq, Eq)]
    pub InstancePoliciesError
        RequestErrorError{url:String, error:String} = "An error occured while trying to GET from {url}: {error}",
        ReceivedErrorCodeError{error_code:String} = "Received the following error code while requesting from the route: {error_code}"
}

custom_error! {
    #[derive(PartialEq, Eq)]
    pub RegisterSchemaError
    PasswordError = "Password must be between 1 and 72 characters.",
    UsernameError = "Username must be between 2 and 32 characters.",
    ConsentError = "Consent must be 'true' to register.",
    EmailError = "The provided email address is in an invalid format."
}
