pub mod schemas {
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, fmt};

    use crate::errors::FieldFormatError;

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
            if !regex.is_match(email.clone().as_str()) {
                return Err(FieldFormatError::EmailError);
            }
            return Ok(AuthEmail { email });
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
                return Err(FieldFormatError::UsernameError);
            } else {
                return Ok(AuthUsername { username });
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
            if password.len() < 1 || password.len() > 72 {
                return Err(FieldFormatError::PasswordError);
            } else {
                return Ok(AuthPassword { password });
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
            return Ok(LoginSchema {
                login,
                password,
                undelete,
                captcha_key,
                login_source,
                gift_code_sku_id,
            });
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginResult {
        token: String,
        settings: UserSettings,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserSettings {
        afk_timeout: i32,
        allow_accessibility_detection: bool,
        animate_emoji: bool,
        animate_stickers: i32,
        contact_sync_enabled: bool,
        convert_emoticons: bool,
        custom_status: Option<String>,
        default_guilds_restricted: bool,
        detect_platform_accounts: bool,
        developer_mode: bool,
        disable_games_tab: bool,
        enable_tts_command: bool,
        explicit_content_filter: i32,
        friend_source_flags: FriendSourceFlags,
        friend_discovery_flags: Option<i32>,
        gateway_connected: bool,
        gif_auto_play: bool,
        guild_folders: Vec<GuildFolder>,
        guild_positions: Vec<i64>,
        inline_attachment_media: bool,
        inline_embed_media: bool,
        locale: String,
        message_display_compact: bool,
        native_phone_integration_enabled: bool,
        render_embeds: bool,
        render_reactions: bool,
        restricted_guilds: Vec<i64>,
        show_current_game: bool,
        status: String,
        stream_notifications_enabled: bool,
        theme: String,
        timezone_offset: i32,
        view_nsfw_guilds: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FriendSourceFlags {
        all: Option<bool>,
        mutual_friends: Option<bool>,
        mutual_guilds: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GuildFolder {
        id: String,
        guild_ids: Vec<i64>,
        name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct TotpSchema {
        code: String,
        ticket: String,
        gift_code_sku_id: Option<String>,
        login_source: Option<String>,
    }

    /**
    Represents the result you get from GET: /api/instance/policies/.
    */
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
            AuthPassword::new("".to_string()),
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
            long_un = long_un + "a";
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
