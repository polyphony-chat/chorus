use std::sync::Arc;

use crate::api::types::*;
use crate::api::WebSocketEvent;
use crate::errors::ObserverError;
use crate::gateway::events::Events;

use futures_util::SinkExt;
use futures_util::StreamExt;
use futures_util::stream::SplitSink;
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task;
use tokio::time;
use tokio::time::Instant;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{WebSocketStream, Connector, connect_async_tls_with_config};

/**
Represents a Gateway connection. A Gateway connection will create observable
[`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
implemented [Types] with the trait [`WebSocketEvent`]
*/
pub struct Gateway<'a> {
    pub url: String,
    pub events: Events<'a>,
    websocket: WebSocketConnection,
    heartbeat_handler: Option<HeartbeatHandler>
}

impl<'a> Gateway<'a> {
    pub async fn new(
        websocket_url: String,
    ) -> Result<Gateway<'a>, tokio_tungstenite::tungstenite::Error> {
        return Ok(Gateway {
            url: websocket_url.clone(),
            events: Events::default(),
            websocket: WebSocketConnection::new(websocket_url).await,
            heartbeat_handler: None,
        });
    }

    /// This function reads all messages from the gateway's websocket and updates its events along with the events' observers
    pub async fn update_events(&mut self) {
        
        while let Ok(msg) = self.websocket.rx.lock().await.try_recv() {

            if msg.to_string() == String::new() {
                continue;
            }

            let gateway_payload: GatewayPayload = serde_json::from_str(msg.to_text().unwrap()).unwrap();

            // See https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes
            match gateway_payload.op {
                // Dispatch
                // An event was dispatched, we need to look at the gateway event name t
                0 => {
                    let gateway_payload_t = gateway_payload.t.unwrap();

                    println!("GW: Received {}..", gateway_payload_t);

                    // See https://discord.com/developers/docs/topics/gateway-events#receive-events
                    match gateway_payload_t.as_str() {
                        "READY" => {
                            let data: GatewayReady = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        }
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
                        "MESSAGE_CREATE" => {
                            let new_data: MessageCreate = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.create.update_data(new_data);
                        }
                        "MESSAGE_UPDATE" => {
                            let new_data: MessageUpdate = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.update.update_data(new_data);
                        }
                        "MESSAGE_DELETE" => {
                            let new_data: MessageDelete = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.delete.update_data(new_data);
                        }
                        "MESSAGE_DELETE_BULK" => {
                            let new_data: MessageDeleteBulk = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.delete_bulk.update_data(new_data);
                        }
                        "MESSAGE_REACTION_ADD" => {
                            let new_data: MessageReactionAdd = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.reaction_add.update_data(new_data);
                        }
                        "MESSAGE_REACTION_REMOVE" => {
                            let new_data: MessageReactionRemove = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.reaction_remove.update_data(new_data);
                        }
                        "MESSAGE_REACTION_REMOVE_ALL" => {
                            let new_data: MessageReactionRemoveAll = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.reaction_remove_all.update_data(new_data);
                        }
                        "MESSAGE_REACTION_REMOVE_EMOJI" => {
                            let new_data: MessageReactionRemoveEmoji= serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.message.reaction_remove_emoji.update_data(new_data);
                        }
                        "PRESENCE_UPDATE" => {
                            let new_data: PresenceUpdate = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.user.presence_update.update_data(new_data);
                        }
                        "STAGE_INSTANCE_CREATE" => {}
                        "STAGE_INSTANCE_UPDATE" => {}
                        "STAGE_INSTANCE_DELETE" => {}
                        // Not documented in discord docs, I assume this isnt for bots / apps but is for users?
                        "SESSIONS_REPLACE" => {}
                        "TYPING_START" => {
                            let new_data: TypingStartEvent = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                            self.events.user.typing_start_event.update_data(new_data);
                        }
                        "USER_UPDATE" => {}
                        "VOICE_STATE_UPDATE" => {}
                        "VOICE_SERVER_UPDATE" => {}
                        "WEBHOOKS_UPDATE" => {}
                        _ => {panic!("Invalid gateway event ({})", &gateway_payload_t)}
                    }
                }
                // Heartbeat
                // We received a heartbeat from the server
                1 => {}
                // Reconnect
                7 => {todo!()}
                // Invalid Session
                9 => {todo!()}
                // Hello
                // Starts our heartbeat
                10 => {
                    println!("GW: Received Hello");
                    let gateway_hello: HelloData = serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                    self.heartbeat_handler = Some(HeartbeatHandler::new(gateway_hello.heartbeat_interval, self.websocket.tx.clone()));
                }
                // Heartbeat ACK
                11 => {
                    println!("GW: Received Heartbeat ACK");
                }
                2 | 3 | 4 | 6 | 8 => {panic!("Received Gateway op code that's meant to be sent, not received ({})", gateway_payload.op)}
                _ => {panic!("Received Invalid Gateway op code ({})", gateway_payload.op)}
            }

            // If we have an active heartbeat thread and we received a seq number we should let it know
            if gateway_payload.s.is_some() {
                if self.heartbeat_handler.is_some() {

                    let heartbeat_communication = HeartbeatThreadCommunication { op: gateway_payload.op, d: gateway_payload.s.unwrap() };

                    self.heartbeat_handler.as_mut().unwrap().tx.send(heartbeat_communication).await.unwrap();
                }
            }
        }
    }

    /// Sends json to the gateway with an opcode
    async fn send_json_event(&self, op: u8, to_send: serde_json::Value) {

        let gateway_payload = GatewayPayload { op, d: Some(to_send), s: None, t: None };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();

        let message = tokio_tungstenite::tungstenite::Message::text(payload_json);

        self.websocket.tx.lock().await.send(message).await.unwrap();
    }

    /// Sends an identify event to the gateway
    pub async fn send_identify(&self, to_send: GatewayIdentifyPayload) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Identify..");

        self.send_json_event(2, to_send_value).await;
    }

    /// Sends a resume event to the gateway
    pub async fn send_resume(&self, to_send: GatewayResume) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Resume..");

        self.send_json_event(6, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    pub async fn send_update_presence(&self, to_send: PresenceUpdate) {

        let to_send_value = serde_json::to_value(&to_send).unwrap();

        self.send_json_event(3, to_send_value).await;
    }
}

/**
Handles sending heartbeats to the gateway in another thread
*/
struct HeartbeatHandler {
    /// The heartbeat interval in milliseconds
    heartbeat_interval: u128,
    tx: Sender<HeartbeatThreadCommunication>,
}

impl HeartbeatHandler {
    pub fn new(heartbeat_interval: u128, websocket_tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>>>) -> HeartbeatHandler {
        let (tx, mut rx) = mpsc::channel(32);

        task::spawn(async move {
            let mut last_heartbeat: Instant = time::Instant::now();
            let mut last_seq_number: Option<u64> = None;

            loop {

                // If we received a seq number update, use that as the last seq number
                let hb_communication: Result<HeartbeatThreadCommunication, TryRecvError> = rx.try_recv();
                if hb_communication.is_ok() {
                    last_seq_number = Some(hb_communication.unwrap().d);
                }

                if last_heartbeat.elapsed().as_millis() > heartbeat_interval {

                    println!("GW: Sending Heartbeat..");

                    let heartbeat = GatewayHeartbeat {
                        op: 1,
                        d: last_seq_number
                    };

                    let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                    let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                    websocket_tx.lock()
                    .await
                    .send(msg)
                    .await
                    .unwrap();

                    last_heartbeat = time::Instant::now();
                }
            }
        });

        Self { heartbeat_interval, tx }
    }
}

/**
Used to communicate with the main thread.
Either signifies a sequence number update or a received heartbeat ack
*/
#[derive(Clone, Copy, Debug)]
struct HeartbeatThreadCommunication {
    /// An opcode for the communication we received
    op: u8,
    /// The sequence number we got from discord
    d: u64
}

struct WebSocketConnection {
    rx: Arc<Mutex<Receiver<tokio_tungstenite::tungstenite::Message>>>,
    tx: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Message>>>,
}

impl<'a> WebSocketConnection {
    async fn new(websocket_url: String) -> WebSocketConnection {

        let (receive_channel_write, receive_channel_read): (
            Sender<tokio_tungstenite::tungstenite::Message>,
            Receiver<tokio_tungstenite::tungstenite::Message>,
        ) = channel(32);

        let shared_receive_channel_read = Arc::new(Mutex::new(receive_channel_read));

        let (ws_stream, _) = match connect_async_tls_with_config(
            &websocket_url,
            None,
            Some(Connector::NativeTls(
                TlsConnector::builder().build().unwrap(),
            )),
        )
        .await
        {
            Ok(ws_stream) => ws_stream,
            Err(e) => panic!("{:?}", e),
        };

        let (ws_tx, mut ws_rx) = ws_stream.split();

        task::spawn(async move {
            loop {

                // Write received messages to the receive channel
                let msg = ws_rx.next().await;
                if msg.as_ref().is_some() {
                    let msg_unwrapped = msg.unwrap().unwrap();
                    receive_channel_write
                        .send(msg_unwrapped)
                        .await
                        .unwrap();
                };
            }
        });

        WebSocketConnection {
            tx: Arc::new(Mutex::new(ws_tx)),
            rx: shared_receive_channel_read,
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
        let mut gateway = Gateway::new("ws://localhost:3001/".to_string())
            .await
            .unwrap();
        gateway.send_identify(GatewayIdentifyPayload::default()).await;
        loop {
            gateway.update_events().await;
        }
    }
}
