use std::{collections::HashMap};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::errors::FieldFormatError;

use super::{Embed};

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

        Ok(RegisterSchema {
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
    login: String,
    password: String,
    undelete: Option<bool>,
    captcha_key: Option<String>,
    login_source: Option<String>,
    gift_code_sku_id: Option<String>,
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
        login: AuthUsername,
        password: String,
        undelete: Option<bool>,
        captcha_key: Option<String>,
        login_source: Option<String>,
        gift_code_sku_id: Option<String>,
    ) -> Result<LoginSchema, FieldFormatError> {
        let login = login.username;
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageSendSchema {
    #[serde(rename = "type")]
    message_type: Option<i32>,
    content: Option<String>,
    nonce: Option<String>,
    tts: Option<bool>,
    embeds: Option<Vec<super::Embed>>,
    allowed_mentions: Option<super::AllowedMention>,
    message_reference: Option<super::MessageReference>,
    components: Option<Vec<super::Component>>,
    sticker_ids: Option<Vec<String>>,
    #[serde(flatten)]
    files: Option<HashMap<String, Vec<u8>>>,
    attachments: Option<Vec<super::PartialDiscordFileAttachment>>,
}

// make a new() method for MessageSendSchema
impl MessageSendSchema {
    pub fn new(
        message_type: Option<i32>,
        content: Option<String>,
        nonce: Option<String>,
        tts: Option<bool>,
        embeds: Option<Vec<Embed>>,
        allowed_mentions: Option<super::AllowedMention>,
        message_reference: Option<super::MessageReference>,
        components: Option<Vec<super::Component>>,
        sticker_ids: Option<Vec<String>>,
        files: Option<HashMap<String, Vec<u8>>>,
        attachments: Option<Vec<super::PartialDiscordFileAttachment>>,
    ) -> MessageSendSchema {
        MessageSendSchema {
            message_type,
            content,
            nonce,
            tts,
            embeds,
            allowed_mentions,
            message_reference,
            components,
            sticker_ids,
            files,
            attachments,
        }
    }
}

// I know that some of these tests are... really really basic and unneccessary, but sometimes, I
// just feel like writing tests, so there you go :) -@bitfl0wer
#[cfg(test)]
mod schemas_tests {
    use super::*;
    use crate::errors::FieldFormatError;

    #[test]
    fn password_too_short() {
        assert_eq!(
            AuthPassword::new("".to_string()),
            Err(FieldFormatError::PasswordError)
        );
    }

    #[test]
    fn password_too_long() {
        let mut long_pw = String::new();
        for _ in 0..73 {
            long_pw += "a";
        }
        assert_eq!(
            AuthPassword::new(long_pw),
            Err(FieldFormatError::PasswordError)
        );
    }

    #[test]
    fn username_too_short() {
        assert_eq!(
            AuthUsername::new("T".to_string()),
            Err(FieldFormatError::UsernameError)
        );
    }

    #[test]
    fn username_too_long() {
        let mut long_un = String::new();
        for _ in 0..33 {
            long_un += "a";
        }
        assert_eq!(
            AuthUsername::new(long_un),
            Err(FieldFormatError::UsernameError)
        );
    }

    #[test]
    fn consent_false() {
        assert_eq!(
            RegisterSchema::new(
                AuthUsername::new("Test".to_string()).unwrap(),
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
            AuthEmail::new("p@p.p".to_string()),
            Err(FieldFormatError::EmailError)
        )
    }

    #[test]
    fn valid_email() {
        let reg = RegisterSchema::new(
            AuthUsername::new("Testy".to_string()).unwrap(),
            None,
            true,
            Some(AuthEmail::new("me@mail.de".to_string()).unwrap()),
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
