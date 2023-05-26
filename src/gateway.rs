use crate::errors::ObserverError;
use crate::gateway::events::Events;
use crate::types;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use native_tls::TlsConnector;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time;
use tokio::time::Instant;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async_tls_with_config, Connector, WebSocketStream};

#[derive(Debug)]
/**
Represents a handle to a Gateway connection. A Gateway connection will create observable
[`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
implemented [Types] with the trait [`WebSocketEvent`]
Using this handle you can also send Gateway Events directly.
*/
pub struct GatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<Events>>,
    pub websocket_tx: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
}

impl GatewayHandle {
    /// Sends json to the gateway with an opcode
    async fn send_json_event(&self, op: u8, to_send: serde_json::Value) {
        let gateway_payload = types::GatewayPayload {
            op,
            d: Some(to_send),
            s: None,
            t: None,
        };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();

        let message = tokio_tungstenite::tungstenite::Message::text(payload_json);

        self.websocket_tx.lock().await.send(message).await.unwrap();
    }

    /// Sends an identify event to the gateway
    pub async fn send_identify(&self, to_send: types::GatewayIdentifyPayload) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Identify..");

        self.send_json_event(2, to_send_value).await;
    }

    /// Sends a resume event to the gateway
    pub async fn send_resume(&self, to_send: types::GatewayResume) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Resume..");

        self.send_json_event(6, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    pub async fn send_update_presence(&self, to_send: types::PresenceUpdate) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Presence Update..");

        self.send_json_event(3, to_send_value).await;
    }

    /// Sends a Request Guild Members to the server
    pub async fn send_request_guild_members(&self, to_send: types::GatewayRequestGuildMembers) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Request Guild Members..");

        self.send_json_event(8, to_send_value).await;
    }

    /// Sends a Request Guild Members to the server
    pub async fn send_update_voice_state(&self, to_send: types::GatewayVoiceStateUpdate) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        println!("GW: Sending Voice State Update..");

        self.send_json_event(4, to_send_value).await;
    }
}

pub struct Gateway {
    pub events: Arc<Mutex<Events>>,
    heartbeat_handler: Option<HeartbeatHandler>,
    pub websocket_tx: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
}

impl Gateway {
    pub async fn new(
        websocket_url: String,
    ) -> Result<GatewayHandle, tokio_tungstenite::tungstenite::Error> {
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
            Err(e) => return Err(e),
        };

        let (ws_tx, mut ws_rx) = ws_stream.split();

        let shared_tx = Arc::new(Mutex::new(ws_tx));

        let mut gateway = Gateway {
            events: Arc::new(Mutex::new(Events::default())),
            heartbeat_handler: None,
            websocket_tx: shared_tx.clone(),
        };

        let shared_events = gateway.events.clone();

        // Wait for the first hello and then spawn both tasks so we avoid nested tasks
        // This automatically spawns the heartbeat task, but from the main thread
        let msg = ws_rx.next().await.unwrap().unwrap();
        let gateway_payload: types::GatewayPayload =
            serde_json::from_str(msg.to_text().unwrap()).unwrap();

        if gateway_payload.op != 10 {
            println!("Recieved non hello on gateway init, what is happening?");
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                tokio_tungstenite::tungstenite::error::ProtocolError::InvalidOpcode(
                    gateway_payload.op,
                ),
            ));
        }

        println!("GW: Received Hello");

        let gateway_hello: types::HelloData =
            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
        gateway.heartbeat_handler = Some(HeartbeatHandler::new(
            gateway_hello.heartbeat_interval,
            shared_tx.clone(),
        ));

        // Now we can continously check for messages in a different task, since we aren't going to receive another hello
        task::spawn(async move {
            loop {
                let msg = ws_rx.next().await;
                if msg.as_ref().is_some() {
                    let msg_unwrapped = msg.unwrap().unwrap();
                    gateway.handle_event(msg_unwrapped).await;
                };
            }
        });

        return Ok(GatewayHandle {
            url: websocket_url.clone(),
            events: shared_events,
            websocket_tx: shared_tx.clone(),
        });
    }

    /// This handles a message as a websocket event and updates its events along with the events' observers
    pub async fn handle_event(&mut self, msg: tokio_tungstenite::tungstenite::Message) {
        if msg.to_string() == String::new() {
            return;
        }

        let msg_string = msg.to_string();

        let gateway_payload: types::GatewayPayload = serde_json::from_str(&msg_string).unwrap();

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
                        let _data: types::GatewayReady =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                    }
                    "RESUMED" => {}
                    "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {}
                    "AUTO_MODERATION_RULE_CREATE" => {}
                    "AUTO_MODERATION_RULE_UPDATE" => {}
                    "AUTO_MODERATION_RULE_DELETE" => {}
                    "AUTO_MODERATION_ACTION_EXECUTION" => {}
                    "CHANNEL_CREATE" => {
                        let channel: types::Channel =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::ChannelCreate { channel };
                        self.events
                            .lock()
                            .await
                            .channel
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_UPDATE" => {
                        let channel: types::Channel =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::ChannelUpdate { channel };
                        self.events
                            .lock()
                            .await
                            .channel
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_DELETE" => {
                        let channel: types::Channel =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::ChannelDelete { channel };
                        self.events
                            .lock()
                            .await
                            .channel
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "CHANNEL_PINS_UPDATE" => {
                        let new_data: types::ChannelPinsUpdate =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .channel
                            .pins_update
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_CREATE" => {
                        let thread: types::Channel =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::ThreadCreate { thread };
                        self.events
                            .lock()
                            .await
                            .thread
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_UPDATE" => {
                        let thread: types::Channel =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::ThreadUpdate { thread };
                        self.events
                            .lock()
                            .await
                            .thread
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_DELETE" => {
                        let thread: types::Channel =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::ThreadDelete { thread };
                        self.events
                            .lock()
                            .await
                            .thread
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_LIST_SYNC" => {
                        let new_data: types::ThreadListSync =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .list_sync
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_MEMBER_UPDATE" => {
                        let new_data: types::ThreadMemberUpdate =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .member_update
                            .update_data(new_data)
                            .await;
                    }
                    "THREAD_MEMBERS_UPDATE" => {
                        let new_data: types::ThreadMembersUpdate =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .thread
                            .members_update
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_CREATE" => {
                        let new_data: types::GuildCreate =
                            serde_json::from_str(&msg_string).unwrap();
                        self.events
                            .lock()
                            .await
                            .guild
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "GUILD_UPDATE" => {}
                    "GUILD_DELETE" => {
                        let _new_data: types::UnavailableGuild =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                    }
                    "GUILD_AUDIT_LOG_ENTRY_CREATE" => {}
                    "GUILD_BAN_ADD" => {
                        let _new_data: types::GuildBanAdd =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                    }
                    "GUILD_BAN_REMOVE" => {
                        let _new_data: types::GuildBanRemove =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                    }
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
                        let new_data: types::MessageCreate =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .create
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_UPDATE" => {
                        let new_data: types::MessageUpdate =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_DELETE" => {
                        let new_data: types::MessageDelete =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .delete
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_DELETE_BULK" => {
                        let new_data: types::MessageDeleteBulk =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .delete_bulk
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_ADD" => {
                        let new_data: types::MessageReactionAdd =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_add
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_REMOVE" => {
                        let new_data: types::MessageReactionRemove =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_remove
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_REMOVE_ALL" => {
                        let new_data: types::MessageReactionRemoveAll =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_remove_all
                            .update_data(new_data)
                            .await;
                    }
                    "MESSAGE_REACTION_REMOVE_EMOJI" => {
                        let new_data: types::MessageReactionRemoveEmoji =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .message
                            .reaction_remove_emoji
                            .update_data(new_data)
                            .await;
                    }
                    "PRESENCE_UPDATE" => {
                        let new_data: types::PresenceUpdate =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .user
                            .presence_update
                            .update_data(new_data)
                            .await;
                    }
                    "STAGE_INSTANCE_CREATE" => {}
                    "STAGE_INSTANCE_UPDATE" => {}
                    "STAGE_INSTANCE_DELETE" => {}
                    // Not documented in discord docs, I assume this isnt for bots / apps but is for users?
                    "SESSIONS_REPLACE" => {}
                    "TYPING_START" => {
                        let new_data: types::TypingStartEvent =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        self.events
                            .lock()
                            .await
                            .user
                            .typing_start_event
                            .update_data(new_data)
                            .await;
                    }
                    "USER_UPDATE" => {
                        let user: types::User =
                            serde_json::from_value(gateway_payload.d.unwrap()).unwrap();
                        let new_data = types::UserUpdate { user };
                        self.events
                            .lock()
                            .await
                            .user
                            .update
                            .update_data(new_data)
                            .await;
                    }
                    "VOICE_STATE_UPDATE" => {}
                    "VOICE_SERVER_UPDATE" => {}
                    "WEBHOOKS_UPDATE" => {}
                    _ => {
                        panic!("Invalid gateway event ({})", &gateway_payload_t)
                    }
                }
            }
            // Heartbeat
            // We received a heartbeat from the server
            1 => {}
            // Reconnect
            7 => {
                todo!()
            }
            // Invalid Session
            9 => {
                todo!()
            }
            // Hello
            // Starts our heartbeat
            // We should have already handled this in gateway init
            10 => {
                panic!("Recieved hello when it was unexpected");
            }
            // Heartbeat ACK
            11 => {
                println!("GW: Received Heartbeat ACK");
            }
            2 | 3 | 4 | 6 | 8 => {
                panic!(
                    "Received Gateway op code that's meant to be sent, not received ({})",
                    gateway_payload.op
                )
            }
            _ => {
                panic!("Received Invalid Gateway op code ({})", gateway_payload.op)
            }
        }

        // If we have an active heartbeat thread and we received a seq number we should let it know
        if gateway_payload.s.is_some() {
            if self.heartbeat_handler.is_some() {
                let heartbeat_communication = HeartbeatThreadCommunication {
                    op: gateway_payload.op,
                    d: gateway_payload.s.unwrap(),
                };

                self.heartbeat_handler
                    .as_mut()
                    .unwrap()
                    .tx
                    .send(heartbeat_communication)
                    .await
                    .unwrap();
            }
        }
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
    pub fn new(
        heartbeat_interval: u128,
        websocket_tx: Arc<
            Mutex<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tokio_tungstenite::tungstenite::Message,
                >,
            >,
        >,
    ) -> HeartbeatHandler {
        let (tx, mut rx) = mpsc::channel(32);

        task::spawn(async move {
            let mut last_heartbeat: Instant = time::Instant::now();
            let mut last_seq_number: Option<u64> = None;

            loop {
                // If we received a seq number update, use that as the last seq number
                let hb_communication: Result<HeartbeatThreadCommunication, TryRecvError> =
                    rx.try_recv();
                if hb_communication.is_ok() {
                    last_seq_number = Some(hb_communication.unwrap().d);
                }

                if last_heartbeat.elapsed().as_millis() > heartbeat_interval {
                    println!("GW: Sending Heartbeat..");

                    let heartbeat = types::GatewayHeartbeat {
                        op: 1,
                        d: last_seq_number,
                    };

                    let heartbeat_json = serde_json::to_string(&heartbeat).unwrap();

                    let msg = tokio_tungstenite::tungstenite::Message::text(heartbeat_json);

                    websocket_tx.lock().await.send(msg).await.unwrap();

                    last_heartbeat = time::Instant::now();
                }
            }
        });

        Self {
            heartbeat_interval,
            tx,
        }
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
    d: u64,
}

/**
Trait which defines the behaviour of an Observer. An Observer is an object which is subscribed to
an Observable. The Observer is notified when the Observable's data changes.
In this case, the Observable is a [`GatewayEvent`], which is a wrapper around a WebSocketEvent.
 */
pub trait Observer<T: types::WebSocketEvent>: std::fmt::Debug {
    fn update(&self, data: &T);
}

/** GatewayEvent is a wrapper around a WebSocketEvent. It is used to notify the observers of a
change in the WebSocketEvent. GatewayEvents are observable.
*/

#[derive(Default, Debug)]
pub struct GatewayEvent<T: types::WebSocketEvent> {
    observers: Vec<Arc<Mutex<dyn Observer<T> + Sync + Send>>>,
    pub event_data: T,
    pub is_observed: bool,
}

impl<T: types::WebSocketEvent> GatewayEvent<T> {
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
    pub fn subscribe(
        &mut self,
        observable: Arc<Mutex<dyn Observer<T> + Sync + Send>>,
    ) -> Option<ObserverError> {
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
    pub fn unsubscribe(&mut self, observable: Arc<Mutex<dyn Observer<T> + Sync + Send>>) {
        // .retain()'s closure retains only those elements of the vector, which have a different
        // pointer value than observable.
        // The usage of the debug format to compare the generic T of observers is quite stupid, but the only thing to compare between them is T and if T == T they are the same
        // anddd there is no way to do that without using format
        self.observers
            .retain(|obs| !(format!("{:?}", obs) == format!("{:?}", &observable)));
        self.is_observed = !self.observers.is_empty();
    }

    /**
    Updates the GatewayEvent's data and notifies the observers.
    */
    async fn update_data(&mut self, new_event_data: T) {
        self.event_data = new_event_data;
        self.notify().await;
    }

    /**
    Notifies the observers of the GatewayEvent.
    */
    async fn notify(&self) {
        for observer in &self.observers {
            observer.lock().await.update(&self.event_data);
        }
    }
}

mod events {
    use super::*;
    #[derive(Default, Debug)]
    pub struct Events {
        pub message: Message,
        pub user: User,
        pub channel: Channel,
        pub thread: Thread,
        pub guild: Guild,
        pub gateway_identify_payload: GatewayEvent<types::GatewayIdentifyPayload>,
        pub gateway_resume: GatewayEvent<types::GatewayResume>,
    }

    #[derive(Default, Debug)]
    pub struct Message {
        pub create: GatewayEvent<types::MessageCreate>,
        pub update: GatewayEvent<types::MessageUpdate>,
        pub delete: GatewayEvent<types::MessageDelete>,
        pub delete_bulk: GatewayEvent<types::MessageDeleteBulk>,
        pub reaction_add: GatewayEvent<types::MessageReactionAdd>,
        pub reaction_remove: GatewayEvent<types::MessageReactionRemove>,
        pub reaction_remove_all: GatewayEvent<types::MessageReactionRemoveAll>,
        pub reaction_remove_emoji: GatewayEvent<types::MessageReactionRemoveEmoji>,
    }

    #[derive(Default, Debug)]
    pub struct User {
        pub update: GatewayEvent<types::UserUpdate>,
        pub presence_update: GatewayEvent<types::PresenceUpdate>,
        pub typing_start_event: GatewayEvent<types::TypingStartEvent>,
    }

    #[derive(Default, Debug)]
    pub struct Channel {
        pub create: GatewayEvent<types::ChannelCreate>,
        pub update: GatewayEvent<types::ChannelUpdate>,
        pub delete: GatewayEvent<types::ChannelDelete>,
        pub pins_update: GatewayEvent<types::ChannelPinsUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct Thread {
        pub create: GatewayEvent<types::ThreadCreate>,
        pub update: GatewayEvent<types::ThreadUpdate>,
        pub delete: GatewayEvent<types::ThreadDelete>,
        pub list_sync: GatewayEvent<types::ThreadListSync>,
        pub member_update: GatewayEvent<types::ThreadMemberUpdate>,
        pub members_update: GatewayEvent<types::ThreadMembersUpdate>,
    }

    #[derive(Default, Debug)]
    pub struct Guild {
        pub create: GatewayEvent<types::GuildCreate>,
        /*pub update: GatewayEvent<ThreadCreate>,
        pub delete: GatewayEvent<ThreadCreate>,
        pub audit_log_entry_create: GatewayEvent<ThreadCreate>,
        pub ban_add: GatewayEvent<ThreadCreate>,
        pub ban_remove: GatewayEvent<ThreadCreate>,
        pub emojis_update: GatewayEvent<ThreadCreate>,
        pub stickers_update: GatewayEvent<ThreadCreate>,
        pub integrations_update: GatewayEvent<ThreadCreate>,
        pub member_add: GatewayEvent<ThreadCreate>,
        pub member_remove: GatewayEvent<ThreadCreate>,
        pub member_update: GatewayEvent<ThreadCreate>,
        pub members_chunk: GatewayEvent<ThreadCreate>,
        pub role_create: GatewayEvent<ThreadCreate>,
        pub role_update: GatewayEvent<ThreadCreate>,
        pub role_delete: GatewayEvent<ThreadCreate>,
        pub role_scheduled_event_create: GatewayEvent<ThreadCreate>,
        pub role_scheduled_event_update: GatewayEvent<ThreadCreate>,
        pub role_scheduled_event_delete: GatewayEvent<ThreadCreate>,
        pub role_scheduled_event_user_add: GatewayEvent<ThreadCreate>,
        pub role_scheduled_event_user_remove: GatewayEvent<ThreadCreate>,*/
    }
}

#[cfg(test)]
mod example {
    use super::*;

    #[derive(Debug)]
    struct Consumer;
    impl Observer<types::GatewayResume> for Consumer {
        fn update(&self, data: &types::GatewayResume) {
            println!("{}", data.token)
        }
    }

    #[tokio::test]
    async fn test_observer_behaviour() {
        let mut event = GatewayEvent::new(types::GatewayResume {
            token: "start".to_string(),
            session_id: "start".to_string(),
            seq: "start".to_string(),
        });

        let new_data = types::GatewayResume {
            token: "token_3276ha37am3".to_string(),
            session_id: "89346671230".to_string(),
            seq: "3".to_string(),
        };

        let consumer = Consumer;
        let arc_mut_consumer = Arc::new(Mutex::new(consumer));

        event.subscribe(arc_mut_consumer.clone());

        event.notify().await;

        event.update_data(new_data).await;

        let second_consumer = Consumer;
        let arc_mut_second_consumer = Arc::new(Mutex::new(second_consumer));

        match event.subscribe(arc_mut_second_consumer.clone()) {
            None => assert!(false),
            Some(err) => println!("You cannot subscribe twice: {}", err),
        }

        event.unsubscribe(arc_mut_consumer.clone());

        match event.subscribe(arc_mut_second_consumer.clone()) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_gateway_establish() {
        let _gateway = Gateway::new("ws://localhost:3001/".to_string())
            .await
            .unwrap();
    }
}
