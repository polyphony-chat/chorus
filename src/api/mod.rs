//! All of the API's endpoints.
pub use channels::messages::*;
pub use guilds::*;
pub use invites::*;
pub use policies::instance::instance::*;
pub use policies::instance::ratelimits::*;
pub use users::*;

pub mod auth;
pub mod channels;
pub mod guilds;
pub mod invites;
pub mod policies;
pub mod users;
