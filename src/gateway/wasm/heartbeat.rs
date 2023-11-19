use tokio::task::{self, JoinHandle};
use ws_stream_wasm::*;

use super::*;

#[async_trait]
impl HeartbeatHandlerCapable<WsMessage, WsStream> for HeartbeatHandler {
    fn get_send(&self) -> &Sender<HeartbeatThreadCommunication> {
        &self.send
    }

    fn get_heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }

    fn new(
        heartbeat_interval: Duration,
        websocket_tx: Arc<Mutex<SplitSink<WsStream, WsMessage>>>,
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
}
