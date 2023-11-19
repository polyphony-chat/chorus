pub mod gateway;
pub mod handle;
pub mod heartbeat;
pub mod message;

#[cfg(not(wasm))]
pub mod backend_tungstenite;
#[cfg(not(wasm))]
use backend_tungstenite::*;

pub use gateway::*;
pub use handle::GatewayHandle;
use heartbeat::*;
pub use message::*;

use crate::errors::GatewayError;
use crate::types::{Snowflake, WebSocketEvent};

use async_trait::async_trait;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::sleep_until;

use futures_util::SinkExt;
use futures_util::StreamExt;
use log::{info, trace, warn};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::Instant;

// Gateway opcodes
/// Opcode received when the server dispatches a [crate::types::WebSocketEvent]
const GATEWAY_DISPATCH: u8 = 0;
/// Opcode sent when sending a heartbeat
const GATEWAY_HEARTBEAT: u8 = 1;
/// Opcode sent to initiate a session
///
/// See [types::GatewayIdentifyPayload]
const GATEWAY_IDENTIFY: u8 = 2;
/// Opcode sent to update our presence
///
/// See [types::GatewayUpdatePresence]
const GATEWAY_UPDATE_PRESENCE: u8 = 3;
/// Opcode sent to update our state in vc
///
/// Like muting, deafening, leaving, joining..
///
/// See [types::UpdateVoiceState]
const GATEWAY_UPDATE_VOICE_STATE: u8 = 4;
/// Opcode sent to resume a session
///
/// See [types::GatewayResume]
const GATEWAY_RESUME: u8 = 6;
/// Opcode received to tell the client to reconnect
const GATEWAY_RECONNECT: u8 = 7;
/// Opcode sent to request guild member data
///
/// See [types::GatewayRequestGuildMembers]
const GATEWAY_REQUEST_GUILD_MEMBERS: u8 = 8;
/// Opcode received to tell the client their token / session is invalid
const GATEWAY_INVALID_SESSION: u8 = 9;
/// Opcode received when initially connecting to the gateway, starts our heartbeat
///
/// See [types::HelloData]
const GATEWAY_HELLO: u8 = 10;
/// Opcode received to acknowledge a heartbeat
const GATEWAY_HEARTBEAT_ACK: u8 = 11;
/// Opcode sent to get the voice state of users in a given DM/group channel
///
/// See [types::CallSync]
const GATEWAY_CALL_SYNC: u8 = 13;
/// Opcode sent to get data for a server (Lazy Loading request)
///
/// Sent by the official client when switching to a server
///
/// See [types::LazyRequest]
const GATEWAY_LAZY_REQUEST: u8 = 14;

pub type ObservableObject = dyn Send + Sync + Any;

/// An entity type which is supposed to be updateable via the Gateway. This is implemented for all such types chorus supports, implementing it for your own types is likely a mistake.
pub trait Updateable: 'static + Send + Sync {
    fn id(&self) -> Snowflake;
}

/// Trait which defines the behavior of an Observer. An Observer is an object which is subscribed to
/// an Observable. The Observer is notified when the Observable's data changes.
/// In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
/// Note that `Debug` is used to tell `Observer`s apart when unsubscribing.
#[async_trait]
pub trait Observer<T>: Sync + Send + std::fmt::Debug {
    async fn update(&self, data: &T);
}

/// GatewayEvent is a wrapper around a WebSocketEvent. It is used to notify the observers of a
/// change in the WebSocketEvent. GatewayEvents are observable.
#[derive(Default, Debug)]
pub struct GatewayEvent<T: WebSocketEvent> {
    observers: Vec<Arc<dyn Observer<T>>>,
}

impl<T: WebSocketEvent> GatewayEvent<T> {
    /// Returns true if the GatewayEvent is observed by at least one Observer.
    pub fn is_observed(&self) -> bool {
        !self.observers.is_empty()
    }

    /// Subscribes an Observer to the GatewayEvent.
    pub fn subscribe(&mut self, observable: Arc<dyn Observer<T>>) {
        self.observers.push(observable);
    }

    /// Unsubscribes an Observer from the GatewayEvent.
    pub fn unsubscribe(&mut self, observable: &dyn Observer<T>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        // The usage of the debug format to compare the generic T of observers is quite stupid, but the only thing to compare between them is T and if T == T they are the same
        // anddd there is no way to do that without using format
        let to_remove = format!("{:?}", observable);
        self.observers
            .retain(|obs| format!("{:?}", obs) != to_remove);
    }

    /// Notifies the observers of the GatewayEvent.
    async fn notify(&self, new_event_data: T) {
        for observer in &self.observers {
            observer.update(&new_event_data).await;
        }
    }
}

#[cfg(test)]
mod example {
    use crate::types;

    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

    #[derive(Debug)]
    struct Consumer {
        _name: String,
        events_received: AtomicI32,
    }

    #[async_trait]
    impl Observer<types::GatewayResume> for Consumer {
        async fn update(&self, _data: &types::GatewayResume) {
            self.events_received.fetch_add(1, Relaxed);
        }
    }

    #[tokio::test]
    async fn test_observer_behavior() {
        let mut event = GatewayEvent::default();

        let new_data = types::GatewayResume {
            token: "token_3276ha37am3".to_string(),
            session_id: "89346671230".to_string(),
            seq: "3".to_string(),
        };

        let consumer = Arc::new(Consumer {
            _name: "first".into(),
            events_received: 0.into(),
        });
        event.subscribe(consumer.clone());

        let second_consumer = Arc::new(Consumer {
            _name: "second".into(),
            events_received: 0.into(),
        });
        event.subscribe(second_consumer.clone());

        event.notify(new_data.clone()).await;
        event.unsubscribe(&*consumer);
        event.notify(new_data).await;

        assert_eq!(consumer.events_received.load(Relaxed), 1);
        assert_eq!(second_consumer.events_received.load(Relaxed), 2);
    }
}
