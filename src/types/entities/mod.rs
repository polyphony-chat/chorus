// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use application::*;
pub use attachment::*;
pub use audit_log::*;
pub use auto_moderation::*;
pub use channel::*;
pub use config::*;
pub use emoji::*;
pub use guild::*;
pub use guild_member::*;
pub use integration::*;
pub use invite::*;
pub use message::*;
pub use ratelimits::*;
pub use relationship::*;
pub use role::*;
pub use security_key::*;
pub use stage_instance::*;
pub use sticker::*;
pub use team::*;
pub use template::*;
pub use user::*;
pub use user_settings::*;
pub use voice_state::*;
pub use webhook::*;

use crate::gateway::Shared;
#[cfg(feature = "client")]
use crate::gateway::Updateable;

#[cfg(feature = "client")]
use crate::gateway::GatewayHandle;

#[cfg(feature = "client")]
use async_trait::async_trait;

#[cfg(feature = "client")]
use std::fmt::Debug;
#[cfg(feature = "client")]
use std::sync::{Arc, RwLock};

mod application;
mod attachment;
mod audit_log;
mod auto_moderation;
mod channel;
mod config;
mod emoji;
mod guild;
mod guild_member;
mod integration;
mod invite;
mod message;
mod ratelimits;
mod relationship;
mod role;
mod security_key;
mod stage_instance;
mod sticker;
mod team;
mod template;
mod user;
mod user_settings;
mod voice_state;
mod webhook;

#[cfg(feature = "client")]
#[async_trait(?Send)]
pub trait Composite<T: Updateable + Clone + Debug> {
    async fn watch_whole(self, gateway: &GatewayHandle) -> Self;

    async fn option_observe_fn(
        value: Option<Shared<T>>,
        gateway: &GatewayHandle,
    ) -> Option<Shared<T>>
    where
        T: Composite<T> + Debug,
    {
        if let Some(value) = value {
            let value = value.clone();
            Some(gateway.observe(value).await)
        } else {
            None
        }
    }

    async fn option_vec_observe_fn(
        value: Option<Vec<Shared<T>>>,
        gateway: &GatewayHandle,
    ) -> Option<Vec<Shared<T>>>
    where
        T: Composite<T>,
    {
        if let Some(value) = value {
            let mut vec = Vec::new();
            for component in value.into_iter() {
                vec.push(gateway.observe(component).await);
            }
            Some(vec)
        } else {
            None
        }
    }

    async fn value_observe_fn(value: Shared<T>, gateway: &GatewayHandle) -> Shared<T>
    where
        T: Composite<T>,
    {
        gateway.observe(value).await
    }

    async fn vec_observe_fn(value: Vec<Shared<T>>, gateway: &GatewayHandle) -> Vec<Shared<T>>
    where
        T: Composite<T>,
    {
        let mut vec = Vec::new();
        for component in value.into_iter() {
            vec.push(gateway.observe(component).await);
        }
        vec
    }
}

pub trait IntoShared {
    /// Uses [`Shared`] to provide an ergonomic alternative to `Arc::new(RwLock::new(obj))`.
    ///
    /// [`Shared<Self>`] can then be observed using the [`Gateway`], turning the underlying
    /// `dyn Composite<Self>` into a self-updating struct, which is a tracked variant of a chorus
    /// entity struct, updating its' held information when new information concerning itself arrives
    /// over the [`Gateway`] connection, reducing the need for expensive network-API calls.
    fn into_shared(self) -> Shared<Self>;
}

impl<T: Sized> IntoShared for T {
    fn into_shared(self) -> Shared<Self> {
        Arc::new(RwLock::new(self))
    }
}
