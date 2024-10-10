// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use apierror::*;
pub use audit_log::*;
pub use auth::*;
pub use channel::*;
pub use guild::*;
pub use message::*;
pub use relationship::*;
pub use role::*;
pub use user::*;
pub use invites::*;
pub use voice_state::*;
pub use instance::*;

mod apierror;
mod audit_log;
mod auth;
mod channel;
mod guild;
mod message;
mod relationship;
mod role;
mod user;
mod invites;
mod voice_state;
mod instance;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericSearchQueryWithLimit {
    pub query: String,
    pub limit: Option<u16>,
}
