// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::types::{
    GuildAffinity, HarvestBackendType, Snowflake, ThemeColors, TwoWayLinkType, UserAffinity,
};

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
    /// See:
    ///
    /// - the endpoints <https://docs.discord.sex/resources/user#modify-user-email> and <https://docs.discord.sex/resources/user#verify-user-email-change>
    ///
    /// - the relevant methods [`ChorusUser::initiate_email_change`](crate::instance::ChorusUser::initiate_email_change) and [`ChorusUser::verify_email_change`](crate::instance::ChorusUser::verify_email_change)
    ///
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

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// A schema used for [ChorusUser::verify_email_change](crate::instance::ChorusUser::verify_email_change)
///
/// See <https://docs.discord.sex/resources/user#verify-user-email-change>
pub struct VerifyUserEmailChangeSchema {
    /// The verification code sent to the user's email
    pub code: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// The return type of [ChorusUser::verify_email_change](crate::instance::ChorusUser::verify_email_change)
///
/// See <https://docs.discord.sex/resources/user#verify-user-email-change>
pub struct VerifyUserEmailChangeResponse {
    /// The email_token to be used in [ChorusUser::modify](crate::instance::ChorusUser::modify)
    #[serde(rename = "token")]
    pub email_token: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// Query string parameters for the route GET /users/{user.id}/profile
/// ([crate::types::User::get_profile])
///
/// See <https://docs.discord.sex/resources/user#get-user-profile>
pub struct GetUserProfileSchema {
    /// Whether to include the mutual guilds between the current user.
    ///
    /// If unset it will default to true
    pub with_mutual_guilds: Option<bool>,
    /// Whether to include the mutual friends between the current user.
    ///
    /// If unset it will default to false
    pub with_mutual_friends: Option<bool>,
    /// Whether to include the number of mutual friends between the current user
    ///
    /// If unset it will default to false
    pub with_mutual_friends_count: Option<bool>,
    /// The guild id to get the user's member profile in, if any.
    ///
    /// Note:
    ///
    /// when you click on a user in the member list in the discord client, a request is sent with
    /// this property set to the selected guild id.
    ///
    /// This makes the request include fields such as guild_member and guild_member_profile
    pub guild_id: Option<Snowflake>,
    /// The role id to get the user's application role connection metadata in, if any.
    pub connections_role_id: Option<Snowflake>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Internal type for the [crate::instance::ChorusUser::get_pomelo_suggestions] endpoint.
///
/// See <https://docs.discord.sex/resources/user#get-pomelo-suggestions>
pub(crate) struct GetPomeloSuggestionsReturn {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Internal type for the [crate::instance::ChorusUser::get_pomelo_eligibility] endpoint.
///
/// See <https://docs.discord.sex/resources/user#get-pomelo-eligibility>
pub(crate) struct GetPomeloEligibilityReturn {
    pub taken: bool,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
/// Query string parameters for the route GET /users/@me/mentions
/// ([crate::instance::ChorusUser::get_recent_mentions])
///
/// See <https://docs.discord.sex/resources/user#get-recent-mentions>
pub struct GetRecentMentionsSchema {
    /// Only fetch messages before this message id
    ///
    /// Due to the nature of snowflakes, this can be easily used to fetch
    /// messages before a certain timestamp
    pub before: Option<Snowflake>,
    /// Max number of messages to return
    ///
    /// Should be between 1 and 100.
    ///
    /// If unset the limit is 25 messages
    pub limit: Option<u8>,
    /// Limit messages to a specific guild
    pub guild_id: Option<Snowflake>,
    /// Whether to include role mentions.
    ///
    /// If unset the server assumes true
    pub roles: Option<bool>,
    /// Whether to include @everyone and @here mentions.
    ///
    /// If unset the server assumes true
    pub everyone: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Internal type for the [crate::instance::ChorusUser::create_harvest] endpoint.
// (koza): imo it's nicer if the user can just pass a vec, instead of having to bother with
// a specific type
///
/// See <https://docs.discord.sex/resources/user#create-user-harvest>
pub(crate) struct CreateUserHarvestSchema {
    pub backends: Option<Vec<HarvestBackendType>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Internal type for the [crate::instance::ChorusUser::set_user_note] endpoint.
///
/// See <https://docs.discord.sex/resources/user#modify-user-note>
pub(crate) struct ModifyUserNoteSchema {
    pub note: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Query string parameters for the route GET /connections/{connection.type}/authorize
/// ([crate::instance::ChorusUser::authorize_connection])
///
/// See <https://docs.discord.sex/resources/user#authorize-user-connection>
pub struct AuthorizeConnectionSchema {
    /// The type of two-way link ([TwoWayLinkType]) to create
    pub two_way_link_type: Option<TwoWayLinkType>,
    /// The device code to use for the two-way link
    pub two_way_user_code: Option<String>,
    /// If this is a continuation of a previous authorization
    pub continuation: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Internal type for the [crate::instance::ChorusUser::authorize_connection] endpoint.
///
/// See <https://docs.discord.sex/resources/user#authorize-user-connection>
pub(crate) struct AuthorizeConnectionReturn {
    pub url: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Json schema for the route POST /connections/{connection.type}/callback ([crate::instance::ChorusUser::create_connection_callback]).
///
/// See <https://docs.discord.sex/resources/user#create-user-connection-callback>
pub struct CreateConnectionCallbackSchema {
    /// The authorization code for the connection
    pub code: String,
    /// The "state" used to authorize a connection
    // TODO: what is this?
    pub state: String,
    pub two_way_link_code: Option<String>,
    pub insecure: Option<bool>,
    pub friend_sync: Option<bool>,
    /// Additional parameters used for OpenID Connect
    // FIXME: Is this correct? in other connections additional info
    // is provided like this, only being string - string
    pub openid_params: Option<HashMap<String, String>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Json schema for the route PUT /users/@me/connections/contacts/{connection.id} ([crate::instance::ChorusUser::create_contact_sync_connection]).
///
/// See <https://docs.discord.sex/resources/user#create-contact-sync-connection>
pub struct CreateContactSyncConnectionSchema {
    /// The username of the connection account
    pub name: String,
    /// Whether to sync friends over the connection
    pub friend_sync: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Json schema for the route PATCH /users/@me/connections/{connection.type}/{connection.id} ([crate::instance::ChorusUser::modify_connection]).
///
/// Note: not all connection types support all parameters.
///
/// See <https://docs.discord.sex/resources/user#modify-user-connection>
pub struct ModifyConnectionSchema {
    /// The connection account's username
    ///
    /// Note: We have not found which connection this could apply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether activities related to this connection will be shown in presence
    ///
    /// e.g. on a Spotify connection, "Display Spotify as your status"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_activity: Option<bool>,

    /// Whether or not to sync friends from this connection
    ///
    /// Note: we have not found which connections this can apply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friend_sync: Option<bool>,

    /// Whether to show additional metadata on the user's profile
    ///
    /// e.g. on a Steam connection, "Display details on profile"
    /// (number of games, items, member since)
    ///
    /// on a Twitter connection, number of posts / followers, member since
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_visibility: Option<bool>,

    /// Whether to show the connection on the user's profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
/// Internal type for the [crate::instance::ChorusUser::get_connection_access_token] endpoint.
///
/// See <https://docs.discord.sex/resources/user#get-user-connection-access-token>
pub(crate) struct GetConnectionAccessTokenReturn {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Return type for the [crate::instance::ChorusUser::get_user_affinities] endpoint.
///
/// See <https://docs.discord.sex/resources/user#get-user-affinities>
pub struct UserAffinities {
    pub user_affinities: Vec<UserAffinity>,
    // FIXME: Is this also a UserAffinity vec?
    // Also, no idea what this means
    pub inverse_user_affinities: Vec<UserAffinity>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
/// Return type for the [crate::instance::ChorusUser::get_guild_affinities] endpoint.
///
/// See <https://docs.discord.sex/resources/user#get-guild-affinities>
pub struct GuildAffinities {
    pub guild_affinities: Vec<GuildAffinity>,
}
