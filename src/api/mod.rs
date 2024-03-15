// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! All of the API's endpoints.

#![allow(unused_imports)]
pub use channels::messages::*;
pub use guilds::*;
pub use invites::*;
pub use policies::instance::instance::*;
pub use users::*;

pub mod auth;
pub mod channels;
pub mod guilds;
pub mod invites;
pub mod policies;
pub mod users;
