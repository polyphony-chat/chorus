pub mod register {
    use std::fmt;

    use reqwest::Client;

    use crate::{api::schemas::schemas::RegisterSchema, instance::Instance};

    impl Instance {
        pub fn register(&mut self, register_schema: &RegisterSchema) {
            let limited_requester = &self.requester;
            let client = Client::new();
            let endpoint_url = &self.urls.get_api() + "/"
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct RegisterError {
        pub message: String,
    }

    impl RegisterError {
        fn new(message: String) -> Self {
            RegisterError { message }
        }
    }

    impl fmt::Display for RegisterError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for RegisterError {}
}
