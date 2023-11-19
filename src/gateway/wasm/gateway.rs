use std::sync::Arc;
use std::u8;

use super::events::Events;
use super::*;
use futures_util::StreamExt;
use tokio::task;
use tokio_stream::StreamExt;
use ws_stream_wasm::*;

use crate::types::{self, GatewayReceivePayload, WebSocketEvent};
#[derive(Debug)]
pub struct WasmGateway {
    events: Arc<Mutex<Events>>,
    heartbeat_handler: HeartbeatHandler,
    websocket_send: Arc<Mutex<SplitSink<WsStream, WsMessage>>>,
    websocket_receive: SplitStream<WsStream>,
    store: GatewayStore,
    url: String,
}

#[async_trait]
impl GatewayCapable<WsMessage, WsStream> for WasmGateway {
    fn get_events(&self) -> Arc<Mutex<Events>> {
        self.events.clone()
    }

    fn get_websocket_send(&self) -> Arc<Mutex<SplitSink<WsStream, WsMessage>>> {
        self.websocket_send.clone()
    }

    fn get_store(&self) -> GatewayStore {
        self.store.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    async fn spawn<G: GatewayHandleCapable<WsMessage, WsStream>>(
        websocket_url: String,
    ) -> Result<G, GatewayError> {
        let (_, mut websocket_stream) = match WsMeta::connect(websocket_url.clone(), None).await {
            Ok(ws) => Ok(ws),
            Err(e) => Err(GatewayError::CannotConnect {
                error: e.to_string(),
            }),
        }?;

        let (kill_send, mut _kill_receive) = tokio::sync::broadcast::channel::<()>(16);
        let (websocket_send, mut websocket_receive) = websocket_stream.split();
        let shared_websocket_send = Arc::new(Mutex::new(websocket_send));

        let msg = match websocket_receive.next().await {
            Some(msg) => match msg {
                WsMessage::Text(text) => Ok(text),
                WsMessage::Binary(vec) => Err(GatewayError::NonHelloOnInitiate {
                    opcode: vec.into_iter().next().unwrap_or(u8::MIN),
                }),
            },
            None => Err(GatewayError::CannotConnect {
                error: "No 'Hello' message received!".to_string(),
            }),
        }?;

        let payload: GatewayReceivePayload = match serde_json::from_str(msg.as_str()) {
            Ok(msg) => Ok(msg),
            Err(_) => Err(GatewayError::Decode),
        }?;
        if payload.op_code != GATEWAY_HELLO {
            return Err(GatewayError::NonHelloOnInitiate {
                opcode: payload.op_code,
            });
        };

        info!("GW: Received Hello");

        let gateway_hello: types::HelloData =
            serde_json::from_str(payload.event_data.unwrap().get()).unwrap();

        let events = Events::default();
        let shared_events: Arc<Mutex<Events>> = Arc::new(Mutex::new(events));
        let store: GatewayStore = Arc::new(Mutex::new(HashMap::new()));

        let mut gateway = WasmGateway {
            events: shared_events.clone(),
            heartbeat_handler: todo!(),
            websocket_send: shared_websocket_send.clone(),
            websocket_receive,
            store: store.clone(),
            url: websocket_url.clone(),
        };

        task::spawn_local(async move {
            gateway.gateway_listen_task().await;
        });

        Ok(G::new(
            websocket_url.clone(),
            shared_events.clone(),
            shared_websocket_send.clone(),
            kill_send.clone(),
            store,
        ))
    }
}

impl WasmGateway {
    async fn gateway_listen_task(&mut self) {
        todo!()
    }
}
