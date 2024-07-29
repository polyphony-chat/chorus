// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::types::{Snowflake, ThemeColors};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// A schema used to modify a user.
///
/// See <https://docs.discord.sex/resources/user#json-params>
pub struct UserModifySchema {
    /// The user's new username (2-32 characters)
    ///
    /// Requires that `current_password` is set.
    pub username: Option<String>,
    // TODO: Maybe add a special discriminator type?
    /// Requires that `current_password` is set.
    pub discriminator: Option<String>,
    /// The user's display name (1-32 characters)
    ///
    /// # Note
    ///
    /// This is not yet implemented on Spacebar
    pub global_name: Option<String>,
    // TODO: Add a CDN data type
    pub avatar: Option<String>,
    /// Note: This is not yet implemented on Spacebar
    pub avatar_decoration_id: Option<Snowflake>,
    /// Note: This is not yet implemented on Spacebar
    pub avatar_decoration_sku_id: Option<Snowflake>,
    /// The user's email address; if changing from a verified email, email_token must be provided
    ///
    /// Requires that `current_password` is set.
    // TODO: Is ^ up to date? One would think this may not be the case, since email_token exists
    pub email: Option<String>,
    /// The user's email token from their previous email, required if a new email is set.
    ///
    /// See <https://docs.discord.sex/resources/user#modify-user-email> and <https://docs.discord.sex/resources/user#verify-user-email-change>
    /// for changing the user's email.
    ///
    /// # Note
    ///
    /// This is not yet implemented on Spacebar
    pub email_token: Option<String>,
    /// The user's pronouns (max 40 characters)
    ///
    /// # Note
    ///
    /// This is not yet implemented on Spacebar
    pub pronouns: Option<String>,
    /// The user's banner.
    ///
    /// Can only be changed for premium users
    pub banner: Option<String>,
    /// The user's bio (max 190 characters)
    pub bio: Option<String>,
    /// The user's accent color, as a hex integer
    pub accent_color: Option<u64>,
    /// The user's [UserFlags].
    ///
    /// Only [UserFlags::PREMIUM_PROMO_DISMISSED], [UserFlags::HAS_UNREAD_URGENT_MESSAGES]
    /// and DISABLE_PREMIUM can be set.
    ///
    /// # Note
    ///
    /// This is not yet implemented on Spacebar
    pub flags: Option<u64>,
    /// The user's date of birth, can only be set once
    ///
    /// Requires that `current_password` is set.
    pub date_of_birth: Option<NaiveDate>,
    /// The user's current password (if the account does not have a password, this sets it)
    ///
    /// Required for updating `username`, `discriminator`, `email`, `date_of_birth` and
    /// `new_password`
    #[serde(rename = "password")]
    pub current_password: Option<String>,
    /// The user's new password (8-72 characters)
    ///
    /// Requires that `current_password` is set.
    ///
    /// Regenerates the user's token
    pub new_password: Option<String>,
    /// Spacebar only field, potentially same as `email_token`
    pub code: Option<String>,
}

/// A schema used to create a private channel.
///
/// # Attributes:
/// - recipients: The users to include in the private channel
/// - access_tokens: The access tokens of users that have granted your app the `gdm.join` scope. Only usable for OAuth2 requests (which can only create group DMs).
/// - nicks: A mapping of user IDs to their respective nicknames. Only usable for OAuth2 requests (which can only create group DMs).
///
/// # Reference:
/// Read: <https://discord-userdoccers.vercel.app/resources/channel#json-params>
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PrivateChannelCreateSchema {
    pub recipients: Option<Vec<Snowflake>>,
    pub access_tokens: Option<Vec<String>>,
    pub nicks: Option<HashMap<Snowflake, String>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema used to modify the current user's profile.
///
/// Similar to [crate::types::UserProfileMetadata]
///
/// See <https://docs.discord.sex/resources/user#modify-user-profile>
pub struct UserModifyProfileSchema {
    // Note: one of these causes a 500 if it is sent
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's new pronouns (max 40 characters)
    pub pronouns: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's new bio (max 190 characters)
    pub bio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: Add banner -- do we have an image data struct
    /// The user's new accent color encoded as an i32 representation of a hex color code
    pub accent_color: Option<i32>,

    // Note: without the skip serializing this currently (2024/07/28) causes a 500!
    //
    // Which in turns locks the user's account, requiring phone number verification
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's new [ThemeColors]
    pub theme_colors: Option<ThemeColors>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's new profile popup animation particle type
    pub popout_animation_particle_type: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's new profile emoji id
    pub emoji_id: Option<Snowflake>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user's new profile ffect id
    pub profile_effect_id: Option<Snowflake>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema used to delete or disable the current user's profile.
///
/// See <https://docs.discord.sex/resources/user#disable-user> and
/// <https://docs.discord.sex/resources/user#delete-user>
pub struct DeleteDisableUserSchema {
    /// The user's current password, if any
    pub password: Option<String>,
}
