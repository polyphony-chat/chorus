//! Defines voice raw udp socket handling

use super::crypto;

use std::{net::SocketAddr, sync::Arc};

use log::{debug, info, trace, warn};
use tokio::{net::UdpSocket, sync::RwLock};

use crypto_secretbox::{
    aead::Aead, cipher::generic_array::GenericArray, KeyInit, XSalsa20Poly1305,
};

use discortp::{
    demux::{demux, Demuxed},
    discord::{IpDiscovery, IpDiscoveryPacket, IpDiscoveryType, MutableIpDiscoveryPacket},
    Packet,
};

use super::voice_data::VoiceData;

/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#voice-packet-structure>
/// This always adds up to 12
const RTP_HEADER_SIZE: u8 = 12;

/// Handle to a voice udp connection
///
/// Can be safely cloned and will still correspond to the same connection.
#[derive(Debug, Clone)]
pub struct UdpHandle {
    socket: Arc<UdpSocket>,
    pub data: Arc<RwLock<VoiceData>>,
}

impl UdpHandle {
    /// Constructs and sends encoded opus rtp data.
    ///
    /// Automatically makes an [RtpPacket](discorrtp::rtp::RtpPacket), encrypts it and sends it.
    pub async fn send_opus_data(&self, sequence: u16, timestamp: u32, payload: Vec<u8>) {
        let data_lock = self.data.read().await;
        let ssrc = data_lock.ready_data.clone().unwrap().ssrc;

        let payload_len = payload.len();

        let rtp_data = discortp::rtp::Rtp {
            // Always the same
            version: 2,
            padding: 0,
            extension: 1,
            csrc_count: 0,
            csrc_list: Vec::new(),
            marker: 0,
            payload_type: discortp::rtp::RtpType::Dynamic(120),
            // Actually variable
            sequence: sequence.into(),
            timestamp: timestamp.into(),
            ssrc,
            payload,
        };

        let mut buffer = Vec::new();

        let buffer_size = payload_len + RTP_HEADER_SIZE as usize;

        // Fill the buffer
        for _i in 0..buffer_size {
            buffer.push(0);
        }

        let mut rtp_packet = discortp::rtp::MutableRtpPacket::new(&mut buffer).unwrap();
        rtp_packet.populate(&rtp_data);

        self.send_rtp_packet(rtp_packet).await;
    }

    /// Encrypts and sends and rtp packet.
    pub async fn send_rtp_packet(&self, packet: discortp::rtp::MutableRtpPacket<'_>) {
        let mut mutable_packet = packet;
        self.encrypt_rtp_packet(&mut mutable_packet).await;
        self.send_encrypted_rtp_packet(mutable_packet.consume_to_immutable())
            .await;
    }

    /// Encrypts an unecnrypted rtp packet, mutating its payload.
    pub async fn encrypt_rtp_packet(&self, packet: &mut discortp::rtp::MutableRtpPacket<'_>) {
        let payload = packet.payload();

        let data_lock = self.data.read().await;

        let session_description_result = data_lock.session_description.clone();

        if session_description_result.is_none() {
            // FIXME: Make this function reutrn a result with a proper error type for these kinds
            // of functions
            panic!("Trying to encrypt packet but no key provided yet");
        }

        let session_description = session_description_result.unwrap();

        let nonce_bytes = crypto::get_xsalsa20_poly1305_nonce(packet.to_immutable());
        let nonce = GenericArray::from_slice(&nonce_bytes);

        let key = GenericArray::from_slice(&session_description.secret_key);

        let encryptor = XSalsa20Poly1305::new(key);

        let encryption_result = encryptor.encrypt(nonce, payload);

        if encryption_result.is_err() {
            // FIXME: See above fixme
            panic!("Encryption error");
        }

        let encrypted_payload = encryption_result.unwrap();

        packet.set_payload(&encrypted_payload);
    }

    /// Sends an (already encrypted) rtp packet to the connection.
    pub async fn send_encrypted_rtp_packet(&self, packet: discortp::rtp::RtpPacket<'_>) {
        let raw_bytes = packet.packet();

        self.socket.send(raw_bytes).await.unwrap();
    }
}

#[derive(Debug)]
pub struct UdpHandler {
    pub data: Arc<RwLock<VoiceData>>,
    socket: Arc<UdpSocket>,
}

impl UdpHandler {
    /// Spawns a new udp handler and performs ip discovery.
    ///
    /// Mutates the given data_reference with the ip discovery data.
    pub async fn spawn(
        data_reference: Arc<RwLock<VoiceData>>,
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

        let ip_discovery = IpDiscovery {
            pkt_type: receieved_ip_discovery.get_pkt_type(),
            length: receieved_ip_discovery.get_length(),
            ssrc: receieved_ip_discovery.get_ssrc(),
            address: receieved_ip_discovery.get_address(),
            port: receieved_ip_discovery.get_port(),
            payload: Vec::new(),
        };

        let mut data_reference_lock = data_reference.write().await;
        data_reference_lock.ip_discovery = Some(ip_discovery);
        drop(data_reference_lock);

        let socket = Arc::new(udp_socket);

        let mut handler = UdpHandler {
            data: data_reference.clone(),
            socket: socket.clone(),
        };

        // Now we can continuously check for messages in a different task
        tokio::spawn(async move {
            handler.listen_task().await;
        });

        UdpHandle {
            socket,
            data: data_reference,
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

                let data_lock = self.data.read().await;

                let session_description_result = data_lock.session_description.clone();

                if session_description_result.is_none() {
                    warn!("VUDP: Received encyrpted voice data, but no encryption key, CANNOT DECRYPT!");
                    return;
                }

                let session_description = session_description_result.unwrap();

                let nonce_bytes;

                match session_description.encryption_mode {
                    crate::types::VoiceEncryptionMode::Xsalsa20Poly1305 => {
                        nonce_bytes = crypto::get_xsalsa20_poly1305_nonce(rtp);
                    }
                    _ => {
                        unimplemented!();
                    }
                }

                let nonce = GenericArray::from_slice(&nonce_bytes);

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
