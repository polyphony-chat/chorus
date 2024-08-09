use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::Snowflake;

#[cfg(feature = "client")]
use crate::gateway::Updateable;

// FIXME: Should this type be Composite?
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// A user's data harvest.
///
/// # Reference
///
/// See <https://docs.discord.sex/resources/user#harvest-object>
pub struct Harvest {
    pub harvest_id: Snowflake,
    /// The id of the user being harvested
    pub user_id: Snowflake,
    pub status: HarvestStatus,
    /// The time the harvest was created
    pub created_at: DateTime<Utc>,
    /// The time the harvest was last polled
    pub polled_at: Option<DateTime<Utc>>,
    /// The time the harvest was completed
    pub completed_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "client")]
impl Updateable for Harvest {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Snowflake {
        self.harvest_id
    }
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Default,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Copy,
    PartialOrd,
    Ord,
)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Current status of a [Harvest]
///
/// See <https://docs.discord.sex/resources/user#harvest-status> and <https://docs.discord.sex/resources/user#harvest-object>
pub enum HarvestStatus {
    /// The harvest is queued and has not been started
    Queued = 0,
    /// The harvest is currently running / being processed
    Running = 1,
    /// The harvest has failed
    Failed = 2,
    /// The harvest has been completed successfully
    Completed = 3,
    #[default]
    Unknown = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// A type of backend / service a harvest can be requested for.
///
/// See <https://docs.discord.sex/resources/user#harvest-backend-type> and <https://support.discord.com/hc/en-us/articles/360004957991-Your-Discord-Data-Package>
pub enum HarvestBackendType {
    /// All account information;
    Accounts,
    /// Actions the user has taken;
    ///
    /// Represented as "Your Activity" in the discord client
    Analytics,
    /// First-party embedded activity information;
    ///
    /// e.g.: Chess in the Park, Checkers in the Park, Poker Night 2.0;
    /// Sketch Heads, Watch Together, Letter League, Land-io, Know What I Meme
    Activities,
    /// The user's messages
    Messages,
    /// Official Discord programes;
    ///
    /// e.g.: Partner, HypeSquad, Verified Server
    Programs,
    /// Guilds the user is a member of;
    Servers,
}
