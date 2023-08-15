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

use crate::gateway::{GatewayHandle, Updateable};
use async_trait::async_trait;
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

#[async_trait]
pub trait Composite<T: Updateable + Clone> {
    async fn watch_whole(self, gateway: &GatewayHandle) -> Self;

    async fn option_observe_fn(
        value: Option<Arc<RwLock<T>>>,
        gateway: &GatewayHandle,
    ) -> Option<Arc<RwLock<T>>> {
        if let Some(value) = value {
            Some(gateway.observe_and_get(value).await)
        } else {
            None
        }
    }

    async fn option_vec_observe_fn(
        value: Option<Vec<Arc<RwLock<T>>>>,
        gateway: &GatewayHandle,
    ) -> Option<Vec<Arc<RwLock<T>>>> {
        if let Some(value) = value {
            let mut vec = Vec::new();
            for component in value.into_iter() {
                vec.push(gateway.observe_and_get(component).await);
            }
            Some(vec)
        } else {
            None
        }
    }

    async fn value_observe_fn(value: Arc<RwLock<T>>, gateway: &GatewayHandle) -> Arc<RwLock<T>> {
        gateway.observe_and_get(value).await
    }

    async fn vec_observe_fn(
        value: Vec<Arc<RwLock<T>>>,
        gateway: &GatewayHandle,
    ) -> Vec<Arc<RwLock<T>>> {
        let mut vec = Vec::new();
        for component in value.into_iter() {
            vec.push(gateway.observe_and_get(component).await);
        }
        vec
    }
}
