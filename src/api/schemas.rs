pub mod schemas {
    use std::fmt;

    use regex::Regex;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub struct RegisterSchema {
        username: String,
        password: Option<String>,
        consent: bool,
        email: Option<String>,
        fingerprint: Option<String>,
        invite: Option<String>,
        date_of_birth: Option<String>,
        gift_code_sku_id: Option<String>,
        captcha_key: Option<String>,
        promotional_email_opt_in: Option<bool>,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct RegisterSchemaError {
        pub message: String,
    }

    impl RegisterSchemaError {
        fn new(message: String) -> Self {
            RegisterSchemaError { message }
        }
    }

    impl fmt::Display for RegisterSchemaError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for RegisterSchemaError {}

    impl RegisterSchema {
        /**
        Returns a new [`Result<RegisterSchema, RegisterSchemaError>`].
        ## Arguments
        All but "String::username" and "bool::consent" are optional.

        ## Errors
        You will receive a [`RegisterSchemaError`], if:
        - The username is less than 2 or more than 32 characters in length
        - You supply a `password` which is less than 1 or more than 72 characters in length.

        These constraints have been defined [in the Spacebar-API](https://docs.spacebar.chat/routes/)
        */
        pub fn new(
            username: String,
            password: Option<String>,
            consent: bool,
            email: Option<String>,
            fingerprint: Option<String>,
            invite: Option<String>,
            date_of_birth: Option<String>,
            gift_code_sku_id: Option<String>,
            captcha_key: Option<String>,
            promotional_email_opt_in: Option<bool>,
        ) -> Result<RegisterSchema, RegisterSchemaError> {
            if username.len() < 2 || username.len() > 32 {
                return Err(RegisterSchemaError::new(
                    "Username must be between 2 and 32 characters".to_string(),
                ));
            }
            if password.is_some()
                && (password.as_ref().unwrap().len() < 1 || password.as_ref().unwrap().len() > 72)
            {
                return Err(RegisterSchemaError {
                    message: "Password must be between 1 and 72 characters.".to_string(),
                });
            }
            if !consent {
                return Err(RegisterSchemaError {
                    message: "Consent must be 'true' to register.".to_string(),
                });
            }

            let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
            if email.clone().is_some() && !regex.is_match(email.clone().unwrap().as_str()) {
                return Err(RegisterSchemaError {
                    message: "The provided email address is in an invalid format.".to_string(),
                });
            }

            return Ok(RegisterSchema {
                username,
                password,
                consent,
                email,
                fingerprint,
                invite,
                date_of_birth,
                gift_code_sku_id,
                captcha_key,
                promotional_email_opt_in,
            });
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub struct LoginSchema {
        login: String,
        password: String,
        undelete: Option<bool>,
        captcha_key: Option<String>,
        login_source: Option<String>,
        gift_code_sku_id: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct TotpSchema {
        code: String,
        ticket: String,
        gift_code_sku_id: Option<String>,
        login_source: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct InstancePoliciesSchema {
        instance_name: String,
        instance_description: Option<String>,
        front_page: Option<String>,
        tos_page: Option<String>,
        correspondence_email: Option<String>,
        correspondence_user_id: Option<String>,
        image: Option<String>,
        instance_id: Option<String>,
    }
}

// I know that some of these tests are... really really basic and unneccessary, but sometimes, I
// just feel like writing tests, so there you go :) -@bitfl0wer
#[cfg(test)]
mod schemas_tests {
    use super::schemas::*;

    #[test]
    fn password_too_short() {
        assert_eq!(
            RegisterSchema::new(
                "Test".to_string(),
                Some("".to_string()),
                true,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Err(RegisterSchemaError {
                message: "Password must be between 1 and 72 characters.".to_string()
            })
        );
    }

    #[test]
    fn password_too_long() {
        let mut long_pw = String::new();
        for _ in 0..73 {
            long_pw = long_pw + "a";
        }
        assert_eq!(
            RegisterSchema::new(
                "Test".to_string(),
                Some(long_pw),
                true,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Err(RegisterSchemaError {
                message: "Password must be between 1 and 72 characters.".to_string()
            })
        );
    }

    #[test]
    fn username_too_short() {
        assert_eq!(
            RegisterSchema::new(
                "T".to_string(),
                None,
                true,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Err(RegisterSchemaError {
                message: "Username must be between 2 and 32 characters".to_string()
            })
        );
    }

    #[test]
    fn username_too_long() {
        let mut long_un = String::new();
        for _ in 0..33 {
            long_un = long_un + "a";
        }
        assert_eq!(
            RegisterSchema::new(long_un, None, true, None, None, None, None, None, None, None,),
            Err(RegisterSchemaError {
                message: "Username must be between 2 and 32 characters".to_string()
            })
        );
    }

    #[test]
    fn consent_false() {
        assert_eq!(
            RegisterSchema::new(
                "Test".to_string(),
                None,
                false,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Err(RegisterSchemaError {
                message: "Consent must be 'true' to register.".to_string()
            })
        );
    }

    #[test]
    fn invalid_email() {
        assert_eq!(
            RegisterSchema::new(
                "Test".to_string(),
                None,
                true,
                Some("p@p.p".to_string()),
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            Err(RegisterSchemaError {
                message: "The provided email address is in an invalid format.".to_string()
            })
        )
    }

    #[test]
    fn valid_email() {
        let reg = RegisterSchema::new(
            "Test".to_string(),
            None,
            true,
            Some("me@mail.xy".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_ne!(
            reg,
            Err(RegisterSchemaError {
                message: "The provided email address is in an invalid format.".to_string()
            })
        );
    }
}
