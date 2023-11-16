use super::*;

#[async_trait]
impl HeartbeatHandlerCapable<WebSocketStream<MaybeTlsStream<TcpStream>>> for HeartbeatHandler {
    fn new(
        heartbeat_interval: Duration,
        websocket_tx: Arc<
            Mutex<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tokio_tungstenite::tungstenite::Message,
                >,
            >,
        >,
        kill_rc: tokio::sync::broadcast::Receiver<()>,
    ) -> HeartbeatHandler {
        let (send, receive) = tokio::sync::mpsc::channel(32);
        let kill_receive = kill_rc.resubscribe();

        let handle: JoinHandle<()> = task::spawn(async move {
            HeartbeatHandler::heartbeat_task(
                websocket_tx,
                heartbeat_interval,
                receive,
                kill_receive,
            )
            .await;
        });

        Self {
            heartbeat_interval,
            send,
            handle,
        }
    }

    fn get_send(&self) -> &Sender<HeartbeatThreadCommunication> {
        &self.send
    }

    fn get_heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }
}
