//! Defines voice raw udp socket handling

use self::voice_udp_events::VoiceUDPEvents;

use super::crypto;

use std::{net::SocketAddr, sync::Arc};

use log::{debug, info, trace, warn};
use tokio::{
    net::UdpSocket,
    sync::{Mutex, RwLock},
};

use crypto_secretbox::{
    aead::Aead, cipher::generic_array::GenericArray, KeyInit, XSalsa20Poly1305,
};

use discortp::{
    demux::{demux, Demuxed},
    discord::{IpDiscovery, IpDiscoveryPacket, IpDiscoveryType, MutableIpDiscoveryPacket},
    rtcp::report::{ReceiverReport, SenderReport},
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
    pub events: Arc<Mutex<VoiceUDPEvents>>,
    socket: Arc<UdpSocket>,
    pub data: Arc<RwLock<VoiceData>>,
}

impl UdpHandle {
    /// Constructs and sends encoded opus rtp data.
    ///
    /// Automatically makes an [RtpPacket](discorrtp::rtp::RtpPacket), encrypts it and sends it.
    pub async fn send_opus_data(&self, timestamp: u32, payload: Vec<u8>) {
        let ssrc = self.data.read().await.ready_data.clone().unwrap().ssrc.clone();
        let sequence_number = self.data.read().await.last_sequence_number.clone().wrapping_add(1);
        self.data.write().await.last_sequence_number = sequence_number;

        let payload_len = payload.len();

        let rtp_data = discortp::rtp::Rtp {
            // Always the same
            version: 2,
            padding: 0,
            extension: 0,
            csrc_count: 0,
            csrc_list: Vec::new(),
            marker: 0,
            payload_type: discortp::rtp::RtpType::Dynamic(120),
            // Actually variable
            sequence: sequence_number.into(),
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
        let mut buffer = self.encrypt_rtp_packet_payload(&packet).await;
        let new_packet = discortp::rtp::MutableRtpPacket::new(&mut buffer).unwrap();
        self.send_encrypted_rtp_packet(new_packet.consume_to_immutable())
            .await;
    }

    /// Encrypts an unencrypted rtp packet, returning a copy of the packet's bytes with an
    /// encrypted payload
    pub async fn encrypt_rtp_packet_payload(
        &self,
        packet: &discortp::rtp::MutableRtpPacket<'_>,
    ) -> Vec<u8> {
        let payload = packet.payload();

        let session_description_result = self.data.read().await.session_description.clone();

        if session_description_result.is_none() {
            // FIXME: Make this function reutrn a result with a proper error type for these kinds
            // of functions
            panic!("Trying to encrypt packet but no key provided yet");
        }

        let session_description = session_description_result.unwrap();

        let nonce_bytes = crypto::get_xsalsa20_poly1305_nonce(packet.packet());
        let nonce = GenericArray::from_slice(&nonce_bytes);

        let key = GenericArray::from_slice(&session_description.secret_key);

        let encryptor = XSalsa20Poly1305::new(key);

        let encryption_result = encryptor.encrypt(nonce, payload);

        if encryption_result.is_err() {
            // FIXME: See above fixme
            panic!("Encryption error");
        }

        let encrypted_payload = encryption_result.unwrap();

        // We need to allocate a new buffer, since the old one is too small for our new encrypted
        // data
        let mut new_buffer = packet.packet().to_vec();

        let buffer_size = encrypted_payload.len() + RTP_HEADER_SIZE as usize;

        // Fill the buffer
        while new_buffer.len() <= buffer_size {
            new_buffer.push(0);
        }

        new_buffer
    }

    /// Sends an (already encrypted) rtp packet to the connection.
    pub async fn send_encrypted_rtp_packet(&self, packet: discortp::rtp::RtpPacket<'_>) {
        let raw_bytes = packet.packet();

        self.socket.send(raw_bytes).await.unwrap();

        debug!("VUDP: Sent rtp packet!");
    }
}

#[derive(Debug)]
pub struct UdpHandler {
    events: Arc<Mutex<VoiceUDPEvents>>,
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

        let events = VoiceUDPEvents::default();
        let shared_events = Arc::new(Mutex::new(events));

        let mut handler = UdpHandler {
            events: shared_events.clone(),
            data: data_reference.clone(),
            socket: socket.clone(),
        };

        // Now we can continuously check for messages in a different task
        tokio::spawn(async move {
            handler.listen_task().await;
        });

        UdpHandle {
            events: shared_events,
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
            let result = self.socket.recv(&mut buf).await;
            if let Ok(size) = result {
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
                trace!("VUDP: Parsed packet as rtp!");

                let session_description_result = self.data.read().await.session_description.clone();

                if session_description_result.is_none() {
                    warn!("VUDP: Received encyrpted voice data, but no encryption key, CANNOT DECRYPT!");
                    return;
                }

                let session_description = session_description_result.unwrap();

                let nonce_bytes;

                match session_description.encryption_mode {
                    crate::types::VoiceEncryptionMode::Xsalsa20Poly1305 => {
                        nonce_bytes = crypto::get_xsalsa20_poly1305_nonce(rtp.packet());
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

                debug!("VUDP: Successfully decrypted voice data!");

                let rtp_with_decrypted_data = discortp::rtp::Rtp {
                    ssrc: rtp.get_ssrc(),
                    marker: rtp.get_marker(),
                    version: rtp.get_version(),
                    padding: rtp.get_padding(),
                    sequence: rtp.get_sequence(),
                    extension: rtp.get_extension(),
                    timestamp: rtp.get_timestamp(),
                    csrc_list: rtp.get_csrc_list(),
                    csrc_count: rtp.get_csrc_count(),
                    payload_type: rtp.get_payload_type(),
                    payload: decrypted,
                };

                self.events
                    .lock()
                    .await
                    .rtp
                    .notify(rtp_with_decrypted_data)
                    .await;
            }
            Demuxed::Rtcp(rtcp) => {
                trace!("VUDP: Parsed packet as rtcp!");

                let rtcp_data;

                match rtcp {
                    discortp::rtcp::RtcpPacket::KnownType(knowntype) => {
                        rtcp_data = discortp::rtcp::Rtcp::KnownType(knowntype);
                    }
                    discortp::rtcp::RtcpPacket::SenderReport(senderreport) => {
                        rtcp_data = discortp::rtcp::Rtcp::SenderReport(SenderReport {
                            payload: senderreport.payload().to_vec(),
                            padding: senderreport.get_padding(),
                            version: senderreport.get_version(),
                            ssrc: senderreport.get_ssrc(),
                            pkt_length: senderreport.get_pkt_length(),
                            packet_type: senderreport.get_packet_type(),
                            rx_report_count: senderreport.get_rx_report_count(),
                        });
                    }
                    discortp::rtcp::RtcpPacket::ReceiverReport(receiverreport) => {
                        rtcp_data = discortp::rtcp::Rtcp::ReceiverReport(ReceiverReport {
                            payload: receiverreport.payload().to_vec(),
                            padding: receiverreport.get_padding(),
                            version: receiverreport.get_version(),
                            ssrc: receiverreport.get_ssrc(),
                            pkt_length: receiverreport.get_pkt_length(),
                            packet_type: receiverreport.get_packet_type(),
                            rx_report_count: receiverreport.get_rx_report_count(),
                        });
                    }
                    _ => {
                        unreachable!();
                    }
                }

                self.events.lock().await.rtcp.notify(rtcp_data).await;
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

pub mod voice_udp_events {

    use discortp::{rtcp::Rtcp, rtp::Rtp};

    use crate::{gateway::GatewayEvent, types::WebSocketEvent};

    impl WebSocketEvent for Rtp {}
    impl WebSocketEvent for Rtcp {}

    #[derive(Debug)]
    pub struct VoiceUDPEvents {
        pub rtp: GatewayEvent<Rtp>,
        pub rtcp: GatewayEvent<Rtcp>,
    }

    impl Default for VoiceUDPEvents {
        fn default() -> Self {
            Self {
                rtp: GatewayEvent::new(),
                rtcp: GatewayEvent::new(),
            }
        }
    }
}
