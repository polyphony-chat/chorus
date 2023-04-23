pub mod schemas {
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::fmt;

    use crate::errors::FieldFormatError;

    /**
    A struct that represents a well-formed email address.
     */
    #[derive(Clone)]
    pub struct AuthEmail {
        pub email: String,
    }

    impl AuthEmail {
        /**
        Returns a new [`Result<AuthEmail, FieldFormatError>`].
        ## Arguments
        The email address you want to validate.
        ## Errors
        You will receive a [`FieldFormatError`], if:
        - The email address is not in a valid format.

         */
        pub fn new(email: String) -> Result<AuthEmail, FieldFormatError> {
            let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
            if !regex.is_match(email.clone().as_str()) {
                return Err(FieldFormatError::EmailError);
            }
            return Ok(AuthEmail { email });
        }
    }

    #[derive(Clone)]
    pub struct AuthUsername {
        pub username: String,
    }

    impl AuthUsername {
        pub fn new(username: String) -> Result<AuthUsername, FieldFormatError> {
            if username.len() < 2 || username.len() > 32 {
                return Err(FieldFormatError::UsernameError);
            } else {
                return Ok(AuthUsername { username });
            }
        }
    }

    #[derive(Clone)]
    pub struct AuthPassword {
        pub password: String,
    }

    impl AuthPassword {
        pub fn new(password: String) -> Result<AuthPassword, FieldFormatError> {
            if password.len() < 1 || password.len() > 72 {
                return Err(FieldFormatError::PasswordError);
            } else {
                return Ok(AuthPassword { password });
            }
        }
    }

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

    impl RegisterSchema {
        /**
        Returns a new [`Result<RegisterSchema, FieldFormatError>`].
        ## Arguments
        All but "String::username" and "bool::consent" are optional.

        ## Errors
        You will receive a [`FieldFormatError`], if:
        - The username is less than 2 or more than 32 characters in length
        - You supply a `password` which is less than 1 or more than 72 characters in length.

        These constraints have been defined [in the Spacebar-API](https://docs.spacebar.chat/routes/)
        */
        pub fn new(
            username: AuthUsername,
            password: Option<AuthPassword>,
            consent: bool,
            email: Option<AuthEmail>,
            fingerprint: Option<String>,
            invite: Option<String>,
            date_of_birth: Option<String>,
            gift_code_sku_id: Option<String>,
            captcha_key: Option<String>,
            promotional_email_opt_in: Option<bool>,
        ) -> Result<RegisterSchema, FieldFormatError> {
            let username = username.username;

            let email_addr;
            if email.is_some() {
                email_addr = Some(email.unwrap().email);
            } else {
                email_addr = None;
            }

            let has_password;
            if password.is_some() {
                has_password = Some(password.unwrap().password);
            } else {
                has_password = None;
            }

            if !consent {
                return Err(FieldFormatError::ConsentError);
            }

            return Ok(RegisterSchema {
                username,
                password: has_password,
                consent,
                email: email_addr,
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

    impl InstancePoliciesSchema {
        pub fn new(
            instance_name: String,
            instance_description: Option<String>,
            front_page: Option<String>,
            tos_page: Option<String>,
            correspondence_email: Option<String>,
            correspondence_user_id: Option<String>,
            image: Option<String>,
            instance_id: Option<String>,
        ) -> Self {
            InstancePoliciesSchema {
                instance_name,
                instance_description,
                front_page,
                tos_page,
                correspondence_email,
                correspondence_user_id,
                image,
                instance_id,
            }
        }
    }

    impl fmt::Display for InstancePoliciesSchema {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "InstancePoliciesSchema {{ instance_name: {}, instance_description: {}, front_page: {}, tos_page: {}, correspondence_email: {}, correspondence_user_id: {}, image: {}, instance_id: {} }}",
                self.instance_name,
                self.instance_description.clone().unwrap_or("None".to_string()),
                self.front_page.clone().unwrap_or("None".to_string()),
                self.tos_page.clone().unwrap_or("None".to_string()),
                self.correspondence_email.clone().unwrap_or("None".to_string()),
                self.correspondence_user_id.clone().unwrap_or("None".to_string()),
                self.image.clone().unwrap_or("None".to_string()),
                self.instance_id.clone().unwrap_or("None".to_string()),
            )
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ErrorResponse {
        pub code: i32,
        pub message: String,
        pub errors: IntermittentError,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct IntermittentError {
        #[serde(flatten)]
        pub errors: std::collections::HashMap<String, ErrorField>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct ErrorField {
        #[serde(default)]
        pub _errors: Vec<Error>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Error {
        pub message: String,
        pub code: String,
    }
}

// I know that some of these tests are... really really basic and unneccessary, but sometimes, I
// just feel like writing tests, so there you go :) -@bitfl0wer
#[cfg(test)]
mod schemas_tests {
    use super::schemas::*;
    use crate::errors::FieldFormatError;

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
            Err(FieldFormatError::PasswordError)
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
            Err(FieldFormatError::PasswordError)
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
            Err(FieldFormatError::UsernameError)
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
            Err(FieldFormatError::UsernameError)
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
            Err(FieldFormatError::ConsentError)
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
            Err(FieldFormatError::EmailError)
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
        assert_ne!(reg, Err(FieldFormatError::EmailError));
    }
}
