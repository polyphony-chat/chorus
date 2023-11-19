use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, tungstenite, Connector, MaybeTlsStream, WebSocketStream,
};

use super::GatewayMessage;
use crate::errors::GatewayError;

#[derive(Debug, Clone)]
pub struct WebSocketBackend;

// These could be made into inherent associated types when that's stabilized
pub type WsSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>;
pub type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
pub type WsMessage = tungstenite::Message;
pub type WsError = tungstenite::Error;

impl WebSocketBackend {
    // When impl_trait_in_assoc_type gets stabilized, this would just be = impl Sink<Self::Message>

    pub async fn new(
        websocket_url: &str,
    ) -> Result<(WsSink, WsStream), crate::errors::GatewayError> {
        let mut roots = rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs().expect("could not load platform certs")
        {
            roots.add(&rustls::Certificate(cert.0)).unwrap();
        }
        let (websocket_stream, _) = match connect_async_tls_with_config(
            websocket_url,
            None,
            false,
            Some(Connector::Rustls(
                rustls::ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(roots)
                    .with_no_client_auth()
                    .into(),
            )),
        )
        .await
        {
            Ok(websocket_stream) => websocket_stream,
            Err(e) => {
                return Err(GatewayError::CannotConnect {
                    error: e.to_string(),
                })
            }
        };

        Ok(websocket_stream.split())
    }
}

impl From<GatewayMessage> for tungstenite::Message {
    fn from(message: GatewayMessage) -> Self {
        Self::Text(message.0)
    }
}

impl From<tungstenite::Message> for GatewayMessage {
    fn from(value: tungstenite::Message) -> Self {
        Self(value.to_string())
    }
}
