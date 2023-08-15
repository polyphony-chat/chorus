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

use crate::gateway::Updateable;
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

pub(crate) trait Composite<T: Updateable> {
    fn watch_whole(self) -> Self;
    fn option_observe_fn(value: Option<Arc<RwLock<T>>>) -> Option<Arc<RwLock<T>>> {
        // Perform your logic here...
        value
    }

    fn option_vec_observe_fn(value: Option<Vec<Arc<RwLock<T>>>>) -> Option<Vec<Arc<RwLock<T>>>> {
        // Perform your logic here...
        value
    }

    fn value_observe_fn(value: Arc<RwLock<T>>) -> Arc<RwLock<T>> {
        // Perform your logic here...
        value
    }

    fn vec_observe_fn(value: Vec<Arc<RwLock<T>>>) -> Vec<Arc<RwLock<T>>> {
        // Perform your logic here...
        value
    }
}
