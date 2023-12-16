//! Defines voice raw udp socket handling

use std::{net::SocketAddr, sync::Arc};

use log::{debug, info, trace, warn};
use tokio::{net::UdpSocket, sync::Mutex};

use crypto_secretbox::{
    aead::Aead, cipher::generic_array::GenericArray, KeyInit, XSalsa20Poly1305,
};

use discortp::{
    demux::{demux, Demuxed},
    discord::{IpDiscovery, IpDiscoveryPacket, IpDiscoveryType, MutableIpDiscoveryPacket},
    Packet,
};

use super::voice_data::VoiceData;

/// Handle to a voice udp connection
///
/// Can be safely cloned and will still correspond to the same connection.
#[derive(Debug, Clone)]
pub struct UdpHandle {
    /// Ip discovery data we received on init
    pub ip_discovery: IpDiscovery,
    socket: Arc<UdpSocket>,
}

#[derive(Debug)]
pub struct UdpHandler {
    data: Arc<Mutex<VoiceData>>,
    socket: Arc<UdpSocket>,
}

impl UdpHandler {
    pub async fn spawn(
        data_reference: Arc<Mutex<VoiceData>>,
        url: SocketAddr,
        ssrc: u32,
    ) -> UdpHandle {
        // Bind with a port number of 0, so the os assigns this listener a port
        let udp_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

        udp_socket.connect(url).await.unwrap();

        // First perform ip discovery
        let ip_discovery = IpDiscovery {
            pkt_type: IpDiscoveryType::Request,
            ssrc,
            length: 70,
            address: Vec::new(),
            port: 0,
            payload: Vec::new(),
        };

        let mut buf: Vec<u8> = Vec::new();

        let size = IpDiscoveryPacket::minimum_packet_size() + 64;

        for _i in 0..size {
            buf.push(0);
        }

        // TODO: Make this not panic everything
        let mut ip_discovery_packet =
            MutableIpDiscoveryPacket::new(&mut buf).expect("Mangled ip discovery packet");

        ip_discovery_packet.populate(&ip_discovery);

        let data = ip_discovery_packet.packet();

        info!("VUDP: Sending Ip Discovery {:?}", &data);

        udp_socket.send(data).await.unwrap();

        info!("VUDP: Sent packet discovery request");

        // Handle the ip discovery response
        let receieved_size = udp_socket.recv(&mut buf).await.unwrap();
        info!(
            "VUDP: Receiving messsage: {:?} - (expected {} vs real {})",
            buf.clone(),
            size,
            receieved_size
        );

        let receieved_ip_discovery = IpDiscoveryPacket::new(&buf).unwrap();

        info!(
            "VUDP: Received ip discovery!!! {:?}",
            receieved_ip_discovery
        );

        let socket = Arc::new(udp_socket);

        let mut handler = UdpHandler {
            data: data_reference,
            socket: socket.clone(),
        };

        // Now we can continuously check for messages in a different task
        tokio::spawn(async move {
            handler.listen_task().await;
        });

        let ip_discovery = IpDiscovery {
            pkt_type: receieved_ip_discovery.get_pkt_type(),
            length: receieved_ip_discovery.get_length(),
            ssrc: receieved_ip_discovery.get_ssrc(),
            address: receieved_ip_discovery.get_address(),
            port: receieved_ip_discovery.get_port(),
            payload: Vec::new(),
        };

        UdpHandle {
            ip_discovery,
            socket,
        }
    }

    /// The main listen task;
    ///
    /// Receives udp messages and parses them.
    pub async fn listen_task(&mut self) {
        loop {
            let mut buf: Vec<u8> = Vec::new();

            // FIXME: is there a better way to do this?
            for _i in 0..1_000 {
                buf.push(0);
            }
            let msg = self.socket.recv(&mut buf).await;
            if let Ok(size) = msg {
                self.handle_message(&buf[0..size]).await;
                continue;
            }

            warn!("VUDP: Voice UDP is broken, closing connection");
            break;
        }
    }

    /// Handles a message buf
    async fn handle_message(&self, buf: &[u8]) {
        let parsed = demux(buf);

        match parsed {
            Demuxed::Rtp(rtp) => {
                let ciphertext = buf[12..buf.len()].to_vec();
                trace!(
                    "VUDP: Parsed packet as rtp! {:?}; data: {:?}",
                    rtp,
                    ciphertext
                );

                let data_lock = self.data.lock().await;

                let session_description_result = data_lock.session_description.clone();

                if session_description_result.is_none() {
                    warn!("VUDP: Received encyrpted voice data, but no encryption key, CANNOT DECRYPT!");
                    return;
                }

                let session_description = session_description_result.unwrap();

                let nonce;

                let mut rtp_header = buf[0..12].to_vec();

                match session_description.encryption_mode {
                    crate::types::VoiceEncryptionMode::Xsalsa20Poly1305 => {
                        // The header is only 12 bytes, but the nonce has to be 24
                        // This actually works mind you, and anything else doesn't
                        for _i in 0..12 {
                            rtp_header.push(0);
                        }

                        nonce = GenericArray::from_slice(&rtp_header);
                    }
                    _ => {
                        unimplemented!();
                    }
                }

                let key = GenericArray::from_slice(&session_description.secret_key);

                let decryptor = XSalsa20Poly1305::new(key);

                let decryption_result = decryptor.decrypt(nonce, ciphertext.as_ref());

                if let Err(decryption_error) = decryption_result {
                    warn!(
                        "VUDP: Failed to decypt voice data! ({:?})",
                        decryption_error
                    );
                    return;
                }

                let decrypted = decryption_result.unwrap();

                info!("VUDP: SUCCESSFULLY DECRYPTED VOICE DATA!!! {:?}", decrypted);
            }
            Demuxed::Rtcp(rtcp) => {
                trace!("VUDP: Parsed packet as rtcp! {:?}", rtcp);
            }
            Demuxed::FailedParse(e) => {
                trace!("VUDP: Failed to parse packet: {:?}", e);
            }
            Demuxed::TooSmall => {
                unreachable!()
            }
        }
    }
}
