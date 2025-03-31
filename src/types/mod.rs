// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! All the types, entities, events and interfaces of the Spacebar API.

#[cfg(feature = "client")]
use std::sync::{Arc, RwLock};

pub use config::*;
pub use entities::*;
pub use events::*;
pub use interfaces::*;
pub use schema::*;
pub use utils::*;

mod config;
#[cfg(feature = "backend")]
mod database;
mod entities;
pub mod errors;
pub mod events;
mod interfaces;
mod schema;
mod utils;

/// A type alias for [`Arc<RwLock<T>>`], used to make the public facing API concerned with
/// Composite structs more ergonomic.
/// ## Note
///
/// While `T` does not have to implement `Composite` to be used with `Shared`,
/// the primary use of `Shared` is with types that implement `Composite`.
///
/// When the `client` feature is disabled, this does nothing (same as just `T`),
/// since `Composite` structures are disabled.
#[cfg(feature = "client")]
pub type Shared<T> = Arc<RwLock<T>>;
#[cfg(not(feature = "client"))]
pub type Shared<T> = T;
#[cfg(feature = "backend")]
pub use backend::*;

#[cfg(feature = "backend")]
pub mod backend {
    use std::collections::HashMap;
    use std::sync::Arc;

    use pubserve::Publisher;
    use tokio::sync::RwLock;

    use crate::types::Snowflake;

    use super::utils::events::Event;
    pub type SharedEventPublisher = Arc<RwLock<Publisher<Event>>>;
    pub type EventPublisherMap = HashMap<Snowflake, SharedEventPublisher>;
    pub type SharedEventPublisherMap = Arc<RwLock<EventPublisherMap>>;
    pub type WebSocketReceive =
        futures::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>;
    pub type WebSocketSend = futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        tokio_tungstenite::tungstenite::Message,
    >;

    pub async fn eq_shared_event_publisher(
        a: &SharedEventPublisher,
        b: &SharedEventPublisher,
    ) -> bool {
        let a = a.read().await;
        let b = b.read().await;
        *a == *b
    }
}
