use super::{events::Events, *};
use crate::types::{self, Composite};

#[async_trait(?Send)]
impl
    GatewayHandleCapable<
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        WebSocketStream<MaybeTlsStream<TcpStream>>,
    > for DefaultGatewayHandle
{
    async fn send_json_event(&self, op_code: u8, to_send: serde_json::Value) {
        self.send_json_event(op_code, to_send).await
    }

    async fn observe<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Arc<RwLock<T>>,
    ) -> Arc<RwLock<T>> {
        self.observe(object).await
    }

    async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}

/// Represents a handle to a Gateway connection. A Gateway connection will create observable
/// [`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
/// implemented types with the trait [`WebSocketEvent`]
/// Using this handle you can also send Gateway Events directly.
#[derive(Debug, Clone)]
pub struct DefaultGatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<Events>>,
    pub websocket_send: Arc<
        Mutex<
            SplitSink<
                WebSocketStream<MaybeTlsStream<TcpStream>>,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
    /// Tells gateway tasks to close
    pub(super) kill_send: tokio::sync::broadcast::Sender<()>,
    pub(crate) store: GatewayStore,
}

impl DefaultGatewayHandle {
    async fn send_json_event(&self, op_code: u8, to_send: serde_json::Value) {
        let gateway_payload = types::GatewaySendPayload {
            op_code,
            event_data: Some(to_send),
            sequence_number: None,
        };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();

        let message = tokio_tungstenite::tungstenite::Message::text(payload_json);

        self.websocket_send
            .lock()
            .await
            .send(message)
            .await
            .unwrap();
    }

    pub async fn observe<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Arc<RwLock<T>>,
    ) -> Arc<RwLock<T>> {
        let mut store = self.store.lock().await;
        let id = object.read().unwrap().id();
        if let Some(channel) = store.get(&id) {
            let object = channel.clone();
            drop(store);
            object
                .read()
                .unwrap()
                .downcast_ref::<T>()
                .unwrap_or_else(|| {
                    panic!(
                        "Snowflake {} already exists in the store, but it is not of type T.",
                        id
                    )
                });
            let ptr = Arc::into_raw(object.clone());
            // SAFETY:
            // - We have just checked that the typeid of the `dyn Any ...` matches that of `T`.
            // - This operation doesn't read or write any shared data, and thus cannot cause a data race
            // - The reference count is not being modified
            let downcasted = unsafe { Arc::from_raw(ptr as *const RwLock<T>).clone() };
            let object = downcasted.read().unwrap().clone();

            let watched_object = object.watch_whole(self).await;
            *downcasted.write().unwrap() = watched_object;
            downcasted
        } else {
            let id = object.read().unwrap().id();
            let object = object.read().unwrap().clone();
            let object = object.clone().watch_whole(self).await;
            let wrapped = Arc::new(RwLock::new(object));
            store.insert(id, wrapped.clone());
            wrapped
        }
    }
}
