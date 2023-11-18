use super::*;

#[async_trait]
impl
    HeartbeatHandlerCapable<
        tokio_tungstenite::tungstenite::Message,
        WebSocketStream<MaybeTlsStream<TcpStream>>,
    > for HeartbeatHandler
{
    fn get_send(&self) -> &Sender<HeartbeatThreadCommunication> {
        &self.send
    }

    fn get_heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }
}
