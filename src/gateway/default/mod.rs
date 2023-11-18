pub mod gateway;
pub mod handle;
pub mod heartbeat;

use super::*;
pub use gateway::*;
pub use handle::*;
use tokio_tungstenite::tungstenite::Message;

use crate::errors::GatewayError;

use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use log::{info, warn};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task;
use tokio::task::JoinHandle;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async_tls_with_config, Connector, WebSocketStream};

impl crate::gateway::MessageCapable for tokio_tungstenite::tungstenite::Message {
    fn as_string(&self) -> Option<String> {
        match self {
            Message::Text(text) => Some(text.clone()),
            _ => None,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Message::Text(text) => text.is_empty(),
            Message::Binary(bytes) => bytes.is_empty(),
            _ => false,
        }
    }

    fn as_bytes(&self) -> Option<Vec<u8>> {
        match self {
            Message::Binary(bytes) => Some(bytes.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
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
