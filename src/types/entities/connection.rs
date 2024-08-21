use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A 3rd party service connection to a user's account.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#connection-object>
// TODO: Should (could) this type be Updateable and Composite?
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Connection {
    /// The id of the account on the 3rd party service
    #[serde(rename = "id")]
    pub connected_account_id: String,

    #[serde(rename = "type")]
    pub connection_type: ConnectionType,

    /// The username of the connection account
    pub name: String,

    /// If the connection is verified
    pub verified: bool,

    /// Service specific metadata about the connection / connected account
    // FIXME: Is there a better type? As far as I see the value is always encoded as a string
    pub metadata: Option<HashMap<String, String>>,
    pub metadata_visibility: ConnectionVisibilityType,

    /// If the connection if revoked
    pub revoked: bool,

    // TODO: Add integrations
    pub friend_sync: bool,

    /// Whether activities related to this connection will be shown in presence
    pub show_activity: bool,

    /// Whether this connection has a corresponding 3rd party OAuth2 token
    pub two_way_link: bool,

    pub visibility: ConnectionVisibilityType,

    /// The access token for the connection account
    ///
    /// Note: not included when fetching a user's connections via OAuth2
    pub access_token: Option<String>,
}

/// A partial / public [Connection] type.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#partial-connection-structure>
// FIXME: Should (could) this type also be Updateable and Composite?
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PublicConnection {
    /// The id of the account on the 3rd party service
    #[serde(rename = "id")]
    pub connected_account_id: String,

    #[serde(rename = "type")]
    pub connection_type: ConnectionType,

    /// The username of the connection account
    pub name: String,

    /// If the connection is verified
    pub verified: bool,

    /// Service specific metadata about the connection / connected account
    // FIXME: Is there a better type? As far as I see the value is always encoded as a string
    pub metadata: Option<HashMap<String, String>>,
}

impl From<Connection> for PublicConnection {
    fn from(value: Connection) -> Self {
        Self {
            connected_account_id: value.connected_account_id,
            connection_type: value.connection_type,
            name: value.name,
            verified: value.verified,
            metadata: value.metadata,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
/// A type of connection; the service the connection is for
///
/// Note: this is subject to change, and the enum is likely non-exhaustive
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#connection-type>
pub enum ConnectionType {
    #[serde(rename = "amazon-music")]
    AmazonMusic,
    /// Battle.net
    BattleNet,
    /// Bungie.net
    Bungie,
    /// Discord?'s contact sync
    ///
    /// (Not returned in Get User Profile or when fetching connections)
    Contacts,
    Crunchyroll,
    /// Note: spacebar only
    Discord,
    Domain,
    Ebay,
    EpicGames,
    Facebook,
    GitHub,
    Instagram,
    LeagueOfLegends,
    PayPal,
    /// Playstation network
    Playstation,
    Reddit,
    Roblox,
    RiotGames,
    /// Samsung Galaxy
    ///
    /// Users can no longer add this service
    Samsung,
    Spotify,
    /// Users can no longer add this service
    Skype,
    Steam,
    TikTok,
    Twitch,
    Twitter,
    Xbox,
    YouTube,
}

impl Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AmazonMusic => f.write_str("Amazon Music"),
            Self::BattleNet => f.write_str("Battle.net"),
            Self::Bungie => f.write_str("Bungie.net"),
            Self::Ebay => f.write_str("eBay"),
            Self::EpicGames => f.write_str("Epic Games"),
            Self::LeagueOfLegends => f.write_str("League of Legends"),
            Self::Playstation => f.write_str("PlayStation Network"),
            Self::RiotGames => f.write_str("Riot Games"),
            Self::Samsung => f.write_str("Samsung Galaxy"),
            _ => f.write_str(format!("{:?}", self).as_str()),
        }
    }
}

impl ConnectionType {
    /// Returns an vector of all the connection types
    // API note: this could be an array, but it is subject to change.
    pub fn vector() -> Vec<ConnectionType> {
        vec![
            ConnectionType::AmazonMusic,
            ConnectionType::BattleNet,
            ConnectionType::Bungie,
            ConnectionType::Contacts,
            ConnectionType::Crunchyroll,
            ConnectionType::Discord,
            ConnectionType::Domain,
            ConnectionType::Ebay,
            ConnectionType::EpicGames,
            ConnectionType::Facebook,
            ConnectionType::GitHub,
            ConnectionType::Instagram,
            ConnectionType::LeagueOfLegends,
            ConnectionType::PayPal,
            ConnectionType::Playstation,
            ConnectionType::Reddit,
            ConnectionType::RiotGames,
            ConnectionType::Samsung,
            ConnectionType::Spotify,
            ConnectionType::Skype,
            ConnectionType::Steam,
            ConnectionType::TikTok,
            ConnectionType::Twitch,
            ConnectionType::Twitter,
            ConnectionType::Xbox,
            ConnectionType::YouTube,
        ]
    }

    /// Returns an vector of all the connection types available on discord
    pub fn discord_vector() -> Vec<ConnectionType> {
        vec![
            ConnectionType::AmazonMusic,
            ConnectionType::BattleNet,
            ConnectionType::Bungie,
            ConnectionType::Contacts,
            ConnectionType::Crunchyroll,
            ConnectionType::Domain,
            ConnectionType::Ebay,
            ConnectionType::EpicGames,
            ConnectionType::Facebook,
            ConnectionType::GitHub,
            ConnectionType::Instagram,
            ConnectionType::LeagueOfLegends,
            ConnectionType::PayPal,
            ConnectionType::Playstation,
            ConnectionType::Reddit,
            ConnectionType::RiotGames,
            ConnectionType::Samsung,
            ConnectionType::Spotify,
            ConnectionType::Skype,
            ConnectionType::Steam,
            ConnectionType::TikTok,
            ConnectionType::Twitch,
            ConnectionType::Twitter,
            ConnectionType::Xbox,
            ConnectionType::YouTube,
        ]
    }

    /// Returns an vector of all the connection types available on spacebar
    pub fn spacebar_vector() -> Vec<ConnectionType> {
        vec![
            ConnectionType::BattleNet,
            ConnectionType::Discord,
            ConnectionType::EpicGames,
            ConnectionType::Facebook,
            ConnectionType::GitHub,
            ConnectionType::Reddit,
            ConnectionType::Spotify,
            ConnectionType::Twitch,
            ConnectionType::Twitter,
            ConnectionType::Xbox,
            ConnectionType::YouTube,
        ]
    }
}

#[derive(
    Serialize_repr, Deserialize_repr, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// # Reference
/// See <https://docs.discord.sex/resources/user#visibility-type>
pub enum ConnectionVisibilityType {
    /// Invisible to everyone except the user themselves
    None = 0,
    /// Visible to everyone
    Everyone = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "lowercase")]
/// A type of two-way connection link
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#two-way-link-type>
pub enum TwoWayLinkType {
    /// The connection is linked via web
    Web,
    /// The connection is linked via mobile
    Mobile,
    /// The connection is linked via desktop
    Desktop,
}

impl Display for TwoWayLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Defines a subreddit as fetched through a Reddit connection.
///
/// # Reference
/// See <https://docs.discord.sex/resources/user#subreddit-structure>
pub struct ConnectionSubreddit {
    /// The subreddit's internal id, e.g. "t5_388p4"
    pub id: String,
    /// How many reddit users follow the subreddit
    pub subscribers: usize,
    /// The subreddit's relative url, e.g. "/r/discordapp/"
    pub url: String,
}
