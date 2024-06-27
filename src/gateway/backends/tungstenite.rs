// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use custom_error::custom_error;
use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, tungstenite, Connector, MaybeTlsStream, WebSocketStream,
};

use crate::gateway::{GatewayMessage, RawGatewayMessage};

#[derive(Debug, Clone)]
pub struct TungsteniteBackend;

// These could be made into inherent associated types when that's stabilized
pub type TungsteniteSink =
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>;
pub type TungsteniteStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

custom_error! {
    pub TungsteniteBackendError
    FailedToLoadCerts{error: std::io::Error} = "failed to load platform native certs: {error}",
    TungsteniteError{error: tungstenite::error::Error} = "encountered a tungstenite error: {error}",
}

impl TungsteniteBackend {
    pub async fn connect(
        websocket_url: &str,
    ) -> Result<(TungsteniteSink, TungsteniteStream), TungsteniteBackendError> {
        let mut roots = rustls::RootCertStore::empty();
        let certs = rustls_native_certs::load_native_certs();

        if let Err(e) = certs {
            log::error!("Failed to load platform native certs! {:?}", e);
            return Err(TungsteniteBackendError::FailedToLoadCerts { error: e });
        }

        for cert in certs.unwrap() {
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
                return Err(TungsteniteBackendError::TungsteniteError {
                    error: e,
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

impl From<RawGatewayMessage> for tungstenite::Message {
    fn from(message: RawGatewayMessage) -> Self {
        match message {
            RawGatewayMessage::Text(text) => tungstenite::Message::Text(text),
            RawGatewayMessage::Bytes(bytes) => tungstenite::Message::Binary(bytes),
        }
    }
}

impl From<tungstenite::Message> for RawGatewayMessage {
    fn from(value: tungstenite::Message) -> Self {
        match value {
            tungstenite::Message::Binary(bytes) => RawGatewayMessage::Bytes(bytes),
            tungstenite::Message::Text(text) => RawGatewayMessage::Text(text),
            _ => RawGatewayMessage::Text(value.to_string()),
        }
    }
}
