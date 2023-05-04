use std::sync::Arc;
use std::thread;

use crate::api::types::*;
use crate::api::WebSocketEvent;
use crate::errors::ObserverError;
use crate::gateway::events::Events;
use crate::URLBundle;

use futures_util::stream::{FilterMap, SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::StreamExt;
use native_tls::TlsConnector;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use tokio::io;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task;
use tokio_tungstenite::tungstenite::error::UrlError;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, connect_async_tls_with_config};
use tokio_tungstenite::{Connector, MaybeTlsStream};

/**
Represents a Gateway connection. A Gateway connection will create observable
[`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
implemented [Types] with the trait [`WebSocketEvent`]
*/
pub struct Gateway<'a> {
    pub url: String,
    pub token: String,
    pub events: Events<'a>,
    websocket: WebSocketConnection,
}

impl<'a> Gateway<'a> {
    pub async fn new(
        websocket_url: String,
        token: String,
    ) -> Result<Gateway<'a>, tokio_tungstenite::tungstenite::Error> {
        return Ok(Gateway {
            url: websocket_url.clone(),
            token,
            events: Events::default(),
            websocket: WebSocketConnection::new(websocket_url).await,
        });
    }

    /// This function reads all messages from the gateway's websocket and updates its events along with the events' observers
    pub async fn update_events(&mut self) {
        while let Some(msg) = self.websocket.rx.lock().await.recv().await {
            let gateway_payload: GatewayPayload = serde_json::from_str(msg.to_text().unwrap()).unwrap();

            // See https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
            match gateway_payload.op {
                // Dispatch
                // An event was dispatched, we need to look at the gateway event name t
                0 => {
                    let gateway_payload_t = gateway_payload.t.unwrap();

                    // See https://discord.com/developers/docs/topics/gateway-events#receive-events
                    match gateway_payload_t.as_str() {
                        "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {}
                        "AUTO_MODERATION_RULE_CREATE" => {}
                        "AUTO_MODERATION_RULE_UPDATE" => {}
                        "AUTO_MODERATION_RULE_DELETE" => {}
                        "AUTO_MODERATION_ACTION_EXECUTION" => {}
                        "CHANNEL_CREATE" => {}
                        "CHANNEL_UPDATE" => {}
                        "CHANNEL_DELETE" => {}
                        "CHANNEL_PINS_UPDATE" => {}
                        "THREAD_CREATE" => {}
                        "THREAD_UPDATE" => {}
                        "THREAD_DELETE" => {}
                        "THREAD_LIST_SYNC" => {}
                        "THREAD_MEMBER_UPDATE" => {}
                        "THREAD_MEMBERS_UPDATE" => {}
                        "GUILD_CREATE" => {}
                        "GUILD_UPDATE" => {}
                        "GUILD_DELETE" => {}
                        "GUILD_AUDIT_LOG_ENTRY_CREATE" => {}
                        "GUILD_BAN_ADD" => {}
                        "GUILD_BAN_REMOVE" => {}
                        "GUILD_EMOJIS_UPDATE" => {}
                        "GUILD_STICKERS_UPDATE" => {}
                        "GUILD_INTEGRATIONS_UPDATE" => {}
                        "GUILD_MEMBER_ADD" => {}
                        "GUILD_MEMBER_REMOVE" => {}
                        "GUILD_MEMBER_UPDATE" => {}
                        "GUILD_MEMBERS_CHUNK" => {}
                        "GUILD_ROLE_CREATE" => {}
                        "GUILD_ROLE_UPDATE" => {}
                        "GUILD_ROLE_DELETE" => {}
                        "GUILD_SCHEDULED_EVENT_CREATE" => {}
                        "GUILD_SCHEDULED_EVENT_UPDATE" => {}
                        "GUILD_SCHEDULED_EVENT_DELETE" => {}
                        "GUILD_SCHEDULED_EVENT_USER_ADD" => {}
                        "GUILD_SCHEDULED_EVENT_USER_REMOVE" => {}
                        "INTEGRATION_CREATE" => {}
                        "INTEGRATION_UPDATE" => {}
                        "INTEGRATION_DELETE" => {}
                        "INTERACTION_CREATE" => {}
                        "INVITE_CREATE" => {}
                        "INVITE_DELETE" => {}
                        "MESSAGE_CREATE" => {}
                        "MESSAGE_UPDATE" => {}
                        "MESSAGE_DELETE" => {}
                        "MESSAGE_DELETE_BULK" => {}
                        "MESSAGE_REACTION_ADD" => {}
                        "MESSAGE_REACTION_REMOVE" => {}
                        "MESSAGE_REACTION_REMOVE_ALL" => {}
                        "MESSAGE_REACTION_REMOVE_EMOJI" => {}
                        "PRESENCE_UPDATE" => {}
                        "STAGE_INSTANCE_CREATE" => {}
                        "STAGE_INSTANCE_UPDATE" => {}
                        "STAGE_INSTANCE_DELETE" => {}
                        "TYPING_START" => {}
                        "USER_UPDATE" => {}
                        "VOICE_STATE_UPDATE" => {}
                        "VOICE_SERVER_UPDATE" => {}
                        "WEBHOOKS_UPDATE" => {}
                        _ => {panic!("Invalid gateway event ({})", &gateway_payload_t)}
                    }
                }
                // Heartbeat
                // We received a heartbeat from the server
                1 => {
                    let gateway_heartbeat: GatewayHeartbeat = serde_json::from_str(gateway_payload.d.unwrap().as_ref()).unwrap();
                }
                // Reconnect
                7 => {todo!()}
                // Invalid Session
                9 => {todo!()}
                // Hello
                // Should start our heartbeat
                10 => {
                    let gateway_hello: GatewayHello = serde_json::from_str(gateway_payload.d.unwrap().as_ref()).unwrap();
                }
                // Heartbeat ACK
                11 => {
                    let gateway_hb_ack: GatewayHeartbeatAck = serde_json::from_str(gateway_payload.d.unwrap().as_ref()).unwrap();
                }
                2 | 3 | 4 | 6 | 8 => {panic!("Received Gateway op code that's meant to be sent, not received ({})", gateway_payload.op)}
                _ => {panic!("Received Invalid Gateway op code ({})", gateway_payload.op)}
            }
        }
    }
}

struct WebSocketConnection {
    rx: Arc<Mutex<Receiver<tokio_tungstenite::tungstenite::Message>>>,
    tx: Arc<Mutex<Sender<tokio_tungstenite::tungstenite::Message>>>,
}

impl<'a> WebSocketConnection {
    async fn new(websocket_url: String) -> WebSocketConnection {
        let parsed_url = Url::parse(&URLBundle::parse_url(websocket_url.clone())).unwrap();
        /*if parsed_url.scheme() != "ws" && parsed_url.scheme() != "wss" {
            return Err(tokio_tungstenite::tungstenite::Error::Url(
                UrlError::UnsupportedUrlScheme,
            ));
        }*/

        let (mut channel_write, mut channel_read): (
            Sender<tokio_tungstenite::tungstenite::Message>,
            Receiver<tokio_tungstenite::tungstenite::Message>,
        ) = channel(32);

        let shared_channel_write = Arc::new(Mutex::new(channel_write));
        let clone_shared_channel_write = shared_channel_write.clone();
        let shared_channel_read = Arc::new(Mutex::new(channel_read));
        let clone_shared_channel_read = shared_channel_read.clone();

        task::spawn(async move {
            let (mut ws_stream, _) = match connect_async_tls_with_config(
                &websocket_url,
                None,
                Some(Connector::NativeTls(
                    TlsConnector::builder().build().unwrap(),
                )),
            )
            .await
            {
                Ok(ws_stream) => ws_stream,
                Err(_) => return, /*return Err(tokio_tungstenite::tungstenite::Error::Io(
                                      io::ErrorKind::ConnectionAborted.into(),
                                  ))*/
            };

            let (mut write_tx, mut write_rx) = ws_stream.split();

            while let Some(msg) = shared_channel_read.lock().await.recv().await {
                write_tx.send(msg).await.unwrap();
            }

            let event = while let Some(msg) = write_rx.next().await {
                shared_channel_write
                    .lock()
                    .await
                    .send(msg.unwrap())
                    .await
                    .unwrap();
            };
        });

        WebSocketConnection {
            tx: clone_shared_channel_write,
            rx: clone_shared_channel_read,
        }
    }
}

/**
Trait which defines the behaviour of an Observer. An Observer is an object which is subscribed to
an Observable. The Observer is notified when the Observable's data changes.
In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
 */
pub trait Observer<T: WebSocketEvent> {
    fn update(&self, data: &T);
}

/** GatewayEvent is a wrapper around a WebSocketEvent. It is used to notify the observers of a
change in the WebSocketEvent. GatewayEvents are observable.
*/

#[derive(Default)]
pub struct GatewayEvent<'a, T: WebSocketEvent> {
    observers: Vec<&'a dyn Observer<T>>,
    pub event_data: T,
    pub is_observed: bool,
}

impl<'a, T: WebSocketEvent> GatewayEvent<'a, T> {
    fn new(event_data: T) -> Self {
        Self {
            is_observed: false,
            observers: Vec::new(),
            event_data,
        }
    }

    /**
    Returns true if the GatewayEvent is observed by at least one Observer.
    */
    pub fn is_observed(&self) -> bool {
        self.is_observed
    }

    /**
    Subscribes an Observer to the GatewayEvent. Returns an error if the GatewayEvent is already
    observed.
    # Errors
    Returns an error if the GatewayEvent is already observed.
    Error type: [`ObserverError::AlreadySubscribedError`]
    */
    pub fn subscribe(&mut self, observable: &'a dyn Observer<T>) -> Option<ObserverError> {
        if self.is_observed {
            return Some(ObserverError::AlreadySubscribedError);
        }
        self.is_observed = true;
        self.observers.push(observable);
        None
    }

    /**
    Unsubscribes an Observer from the GatewayEvent.
    */
    pub fn unsubscribe(&mut self, observable: &'a dyn Observer<T>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        self.observers.retain(|obs| !std::ptr::eq(*obs, observable));
        self.is_observed = !self.observers.is_empty();
        return;
    }

    /**
    Updates the GatewayEvent's data and notifies the observers.
    */
    fn update_data(&mut self, new_event_data: T) {
        self.event_data = new_event_data;
        self.notify();
    }

    /**
    Notifies the observers of the GatewayEvent.
    */
    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.event_data);
        }
    }
}

mod events {
    use super::*;
    #[derive(Default)]
    pub struct Events<'a> {
        pub message: Message<'a>,
        pub user: User<'a>,
        pub gateway_identify_payload: GatewayEvent<'a, GatewayIdentifyPayload>,
        pub gateway_resume: GatewayEvent<'a, GatewayResume>,
    }

    #[derive(Default)]
    pub struct Message<'a> {
        pub create: GatewayEvent<'a, MessageCreate>,
        pub update: GatewayEvent<'a, MessageUpdate>,
        pub delete: GatewayEvent<'a, MessageDelete>,
        pub delete_bulk: GatewayEvent<'a, MessageDeleteBulk>,
        pub reaction_add: GatewayEvent<'a, MessageReactionAdd>,
        pub reaction_remove: GatewayEvent<'a, MessageReactionRemove>,
        pub reaction_remove_all: GatewayEvent<'a, MessageReactionRemoveAll>,
        pub reaction_remove_emoji: GatewayEvent<'a, MessageReactionRemoveEmoji>,
    }

    #[derive(Default)]
    pub struct User<'a> {
        pub presence_update: GatewayEvent<'a, PresenceUpdate>,
        pub typing_start_event: GatewayEvent<'a, TypingStartEvent>,
    }
}

#[cfg(test)]
mod example {
    use super::*;
    use crate::api::types::GatewayResume;

    struct Consumer;
    impl Observer<GatewayResume> for Consumer {
        fn update(&self, data: &GatewayResume) {
            println!("{}", data.token)
        }
    }

    #[test]
    fn test_observer_behaviour() {
        let mut event = GatewayEvent::new(GatewayResume {
            token: "start".to_string(),
            session_id: "start".to_string(),
            seq: "start".to_string(),
        });

        let new_data = GatewayResume {
            token: "token_3276ha37am3".to_string(),
            session_id: "89346671230".to_string(),
            seq: "3".to_string(),
        };

        let consumer = Consumer;

        event.subscribe(&consumer);

        event.notify();

        event.update_data(new_data);

        let second_consumer = Consumer;

        match event.subscribe(&second_consumer) {
            None => assert!(false),
            Some(err) => println!("You cannot subscribe twice: {}", err),
        }
    }

    #[tokio::test]
    async fn test_gateway() {
        let gateway = Gateway::new("ws://localhost:3001/".to_string(), "none".to_string())
            .await
            .unwrap();
    }
}
