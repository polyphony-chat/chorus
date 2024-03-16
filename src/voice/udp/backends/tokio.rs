// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::net::SocketAddr;

use crate::errors::VoiceUdpError;

#[derive(Debug, Clone)]
pub struct TokioBackend;

pub type TokioSocket = tokio::net::UdpSocket;

impl TokioBackend {
    pub async fn connect(url: SocketAddr) -> Result<TokioSocket, VoiceUdpError> {
        // Bind with a port number of 0, so the os assigns this listener a port
        let udp_socket_result = TokioSocket::bind("0.0.0.0:0").await;

        if let Err(e) = udp_socket_result {
            return Err(VoiceUdpError::CannotBind {
                error: format!("{:?}", e),
            });
        }

        let udp_socket = udp_socket_result.unwrap();

        let connection_result = udp_socket.connect(url).await;

        if let Err(e) = connection_result {
            return Err(VoiceUdpError::CannotConnect {
                error: format!("{:?}", e),
            });
        }

        Ok(udp_socket)
    }
}
