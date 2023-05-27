use crate::errors::FieldFormatError;
use regex::Regex;
use serde::{Deserialize, Serialize};

/**
A struct that represents a well-formed email address.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
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
        if !regex.is_match(email.as_str()) {
            return Err(FieldFormatError::EmailError);
        }
        Ok(AuthEmail { email })
    }
}

/**
A struct that represents a well-formed username.
## Arguments
Please use new() to create a new instance of this struct.
## Errors
You will receive a [`FieldFormatError`], if:
- The username is not between 2 and 32 characters.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AuthUsername {
    pub username: String,
}

impl AuthUsername {
    /**
    Returns a new [`Result<AuthUsername, FieldFormatError>`].
    ## Arguments
    The username you want to validate.
    ## Errors
    You will receive a [`FieldFormatError`], if:
    - The username is not between 2 and 32 characters.
     */
    pub fn new(username: String) -> Result<AuthUsername, FieldFormatError> {
        if username.len() < 2 || username.len() > 32 {
            Err(FieldFormatError::UsernameError)
        } else {
            Ok(AuthUsername { username })
        }
    }
}

/**
A struct that represents a well-formed password.
## Arguments
Please use new() to create a new instance of this struct.
## Errors
You will receive a [`FieldFormatError`], if:
- The password is not between 1 and 72 characters.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AuthPassword {
    pub password: String,
}

impl AuthPassword {
    /**
    Returns a new [`Result<AuthPassword, FieldFormatError>`].
    ## Arguments
    The password you want to validate.
    ## Errors
    You will receive a [`FieldFormatError`], if:
    - The password is not between 1 and 72 characters.
     */
    pub fn new(password: String) -> Result<AuthPassword, FieldFormatError> {
        if password.is_empty() || password.len() > 72 {
            Err(FieldFormatError::PasswordError)
        } else {
            Ok(AuthPassword { password })
        }
    }
}

/**
A struct that represents a well-formed register request.
## Arguments
Please use new() to create a new instance of this struct.
## Errors
You will receive a [`FieldFormatError`], if:
- The username is not between 2 and 32 characters.
- The password is not between 1 and 72 characters.
 */

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
    ) -> Result<RegisterSchema, FieldFormatError> {
        let username = AuthUsername::new(username)?.username;

        let email = if let Some(email) = email {
            Some(AuthEmail::new(email)?.email)
        } else {
            None
        };

        let password = if let Some(password) = password {
            Some(AuthPassword::new(password)?.password)
        } else {
            None
        };

        if !consent {
            return Err(FieldFormatError::ConsentError);
        }

        Ok(RegisterSchema {
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
        })
    }
}

/**
A struct that represents a well-formed login request.
## Arguments
Please use new() to create a new instance of this struct.
## Errors
You will receive a [`FieldFormatError`], if:
- The username is not between 2 and 32 characters.
- The password is not between 1 and 72 characters.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct LoginSchema {
    pub login: String,
    pub password: Option<String>,
    pub undelete: Option<bool>,
    pub captcha_key: Option<String>,
    pub login_source: Option<String>,
    pub gift_code_sku_id: Option<String>,
}

impl LoginSchema {
    /**
    Returns a new [`Result<LoginSchema, FieldFormatError>`].
    ## Arguments
    login: The username you want to login with.
    password: The password you want to login with.
    undelete: Honestly no idea what this is for.
    captcha_key: The captcha key you want to login with.
    login_source: The login source.
    gift_code_sku_id: The gift code sku id.
    ## Errors
    You will receive a [`FieldFormatError`], if:
    - The username is less than 2 or more than 32 characters in length
     */
    pub fn new(
        login: String,
        password: Option<String>,
        undelete: Option<bool>,
        captcha_key: Option<String>,
        login_source: Option<String>,
        gift_code_sku_id: Option<String>,
    ) -> Result<LoginSchema, FieldFormatError> {
        Ok(LoginSchema {
            login,
            password,
            undelete,
            captcha_key,
            login_source,
            gift_code_sku_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TotpSchema {
    code: String,
    ticket: String,
    gift_code_sku_id: Option<String>,
    login_source: Option<String>,
}
