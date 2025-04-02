// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(deprecated)] // Since Opcode variants marked as deprecated are being used here, we need to suppress the warnings about them being deprecated

pub mod backends;
pub mod events;
pub mod gateway;
pub mod handle;
pub mod heartbeat;
pub mod message;
pub mod observers;
pub mod options;

pub use backends::*;
pub use gateway::*;
pub use handle::*;
use heartbeat::*;
pub use message::*;
pub use observers::*;
pub use options::*;

use crate::errors::GatewayError;
use crate::types::Snowflake;

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::sync::Mutex;

pub type ObservableObject = dyn Send + Sync + Any;

/// Note: this is a reexport of [pubserve::Subscriber],
/// exported not to break the public api and make development easier
pub use pubserve::Subscriber as Observer;

/// An entity type which is supposed to be updateable via the Gateway. This is implemented for all such types chorus supports, implementing it for your own types is likely a mistake.
pub trait Updateable: 'static + Send + Sync {
    fn id(&self) -> Snowflake;
}
