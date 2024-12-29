// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use custom_error::custom_error;
use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, connect_async_with_config, tungstenite, Connector,
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::{
    gateway::{GatewayCommunication, GatewayMessage, RawGatewayMessage},
    types::CloseCode,
};

#[derive(Debug, Clone, Copy)]
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
        let websocket_url_parsed =
            Url::parse(websocket_url).map_err(|_| TungsteniteBackendError::TungsteniteError {
                error: tungstenite::error::Error::Url(
                    tungstenite::error::UrlError::UnsupportedUrlScheme,
                ),
            })?;
        if websocket_url_parsed.scheme() == "ws" {
            let (websocket_stream, _) =
                match connect_async_with_config(websocket_url, None, false).await {
                    Ok(websocket_stream) => websocket_stream,
                    Err(e) => return Err(TungsteniteBackendError::TungsteniteError { error: e }),
                };

            Ok(websocket_stream.split())
        } else if websocket_url_parsed.scheme() == "wss" {
            let root_store = rustls::RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.into(),
            };

            let config = rustls::ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth();

            let connector = Connector::Rustls(Arc::new(config));

            let (websocket_stream, _) =
                match connect_async_tls_with_config(websocket_url, None, false, Some(connector))
                    .await
                {
                    Ok(websocket_stream) => websocket_stream,
                    Err(e) => return Err(TungsteniteBackendError::TungsteniteError { error: e }),
                };

            Ok(websocket_stream.split())
        } else {
            Err(TungsteniteBackendError::TungsteniteError {
                error: tungstenite::error::Error::Url(
                    tungstenite::error::UrlError::UnsupportedUrlScheme,
                ),
            })
        }
    }
}

impl From<GatewayMessage> for tungstenite::Message {
    fn from(message: GatewayMessage) -> Self {
        Self::Text(message.0.into())
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
            RawGatewayMessage::Text(text) => tungstenite::Message::Text(text.into()),
            RawGatewayMessage::Bytes(bytes) => tungstenite::Message::Binary(bytes.into()),
        }
    }
}

impl From<tungstenite::Message> for GatewayCommunication {
    fn from(value: tungstenite::Message) -> Self {
        match value {
            tungstenite::Message::Binary(bytes) => {
                GatewayCommunication::Message(RawGatewayMessage::Bytes(bytes.to_vec()))
            }
            tungstenite::Message::Text(text) => {
                GatewayCommunication::Message(RawGatewayMessage::Text(text.to_string()))
            }
            tungstenite::Message::Close(close_frame) => {
                if close_frame.is_none() {
                    return GatewayCommunication::Error(CloseCode::UnknownError);
                }

                let close_code = u16::from(close_frame.unwrap().code);

                GatewayCommunication::Error(
                    CloseCode::try_from(close_code).unwrap_or(CloseCode::UnknownError),
                )
            }
            _ => GatewayCommunication::Error(CloseCode::UnknownError),
        }
    }
}
