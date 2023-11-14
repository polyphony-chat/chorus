use super::{event::Events, *};
use crate::types::{self, Composite};

/// Represents a handle to a Gateway connection. A Gateway connection will create observable
/// [`GatewayEvents`](GatewayEvent), which you can subscribe to. Gateway events include all currently
/// implemented types with the trait [`WebSocketEvent`]
/// Using this handle you can also send Gateway Events directly.
#[derive(Debug, Clone)]
pub struct GatewayHandle {
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

impl GatewayHandle {
    /// Sends json to the gateway with an opcode
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

    /// Recursively observes and updates all updateable fields on the struct T. Returns an object `T`
    /// with all of its observable fields being observed.
    pub async fn observe_and_into_inner<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Arc<RwLock<T>>,
    ) -> T {
        let channel = self.observe(object.clone()).await;
        let object = channel.read().unwrap().clone();
        object
    }

    /// Sends an identify event to the gateway
    pub async fn send_identify(&self, to_send: types::GatewayIdentifyPayload) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Identify..");

        self.send_json_event(GATEWAY_IDENTIFY, to_send_value).await;
    }

    /// Sends a resume event to the gateway
    pub async fn send_resume(&self, to_send: types::GatewayResume) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Resume..");

        self.send_json_event(GATEWAY_RESUME, to_send_value).await;
    }

    /// Sends an update presence event to the gateway
    pub async fn send_update_presence(&self, to_send: types::UpdatePresence) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Update Presence..");

        self.send_json_event(GATEWAY_UPDATE_PRESENCE, to_send_value)
            .await;
    }

    /// Sends a request guild members to the server
    pub async fn send_request_guild_members(&self, to_send: types::GatewayRequestGuildMembers) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Request Guild Members..");

        self.send_json_event(GATEWAY_REQUEST_GUILD_MEMBERS, to_send_value)
            .await;
    }

    /// Sends an update voice state to the server
    pub async fn send_update_voice_state(&self, to_send: types::UpdateVoiceState) {
        let to_send_value = serde_json::to_value(to_send).unwrap();

        trace!("GW: Sending Update Voice State..");

        self.send_json_event(GATEWAY_UPDATE_VOICE_STATE, to_send_value)
            .await;
    }

    /// Sends a call sync to the server
    pub async fn send_call_sync(&self, to_send: types::CallSync) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Call Sync..");

        self.send_json_event(GATEWAY_CALL_SYNC, to_send_value).await;
    }

    /// Sends a Lazy Request
    pub async fn send_lazy_request(&self, to_send: types::LazyRequest) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Lazy Request..");

        self.send_json_event(GATEWAY_LAZY_REQUEST, to_send_value)
            .await;
    }

    /// Closes the websocket connection and stops all gateway tasks;
    ///
    /// Esentially pulls the plug on the gateway, leaving it possible to resume;
    pub async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}
