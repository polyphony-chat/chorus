use std::{net::SocketAddr, sync::Arc};

use crypto_secretbox::aead::Aead;
use crypto_secretbox::cipher::generic_array::GenericArray;
use crypto_secretbox::KeyInit;
use crypto_secretbox::XSalsa20Poly1305;

use discortp::demux::Demuxed;
use discortp::discord::{
    IpDiscovery, IpDiscoveryPacket, IpDiscoveryType, MutableIpDiscoveryPacket,
};
use discortp::rtcp::report::ReceiverReport;
use discortp::rtcp::report::SenderReport;
use discortp::{demux::demux, Packet};
use tokio::{
    net::UdpSocket,
    sync::{Mutex, RwLock},
};

use crate::voice::crypto::get_xsalsa20_poly1305_nonce;
use super::RTP_HEADER_SIZE;
use crate::voice::voice_data::VoiceData;

use super::{events::VoiceUDPEvents, UdpHandle};

use log::*;

#[derive(Debug)]
/// The main UDP struct, which handles receiving, parsing and decrypting the rtp packets
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

        let size = IpDiscoveryPacket::minimum_packet_size() + 64;

        let mut buf: Vec<u8> = vec![0; size];

        // TODO: Make this not panic everything
        // Actually, if this panics, something is very, very wrong
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
    async fn listen_task(&mut self) {
        loop {
            // FIXME: is there a max size for these packets?
            // Allocating 512 bytes seems a bit extreme
            let mut buf: Vec<u8> = vec![0; 512];

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
                let ciphertext = buf[(RTP_HEADER_SIZE as usize)..buf.len()].to_vec();
                trace!("VUDP: Parsed packet as rtp!");

                let session_description_result = self.data.read().await.session_description.clone();

                if session_description_result.is_none() {
                    warn!("VUDP: Received encyrpted voice data, but no encryption key, CANNOT DECRYPT!");
                    return;
                }

                let session_description = session_description_result.unwrap();

                let nonce_bytes = match session_description.encryption_mode {
                    crate::types::VoiceEncryptionMode::Xsalsa20Poly1305 => {
                        get_xsalsa20_poly1305_nonce(rtp.packet())
                    }
                    _ => {
                        unimplemented!();
                    }
                };

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

                let rtcp_data = match rtcp {
                    discortp::rtcp::RtcpPacket::KnownType(knowntype) => {
                        discortp::rtcp::Rtcp::KnownType(knowntype)
                    }
                    discortp::rtcp::RtcpPacket::SenderReport(senderreport) => {
                        discortp::rtcp::Rtcp::SenderReport(SenderReport {
                            payload: senderreport.payload().to_vec(),
                            padding: senderreport.get_padding(),
                            version: senderreport.get_version(),
                            ssrc: senderreport.get_ssrc(),
                            pkt_length: senderreport.get_pkt_length(),
                            packet_type: senderreport.get_packet_type(),
                            rx_report_count: senderreport.get_rx_report_count(),
                        })
                    }
                    discortp::rtcp::RtcpPacket::ReceiverReport(receiverreport) => {
                        discortp::rtcp::Rtcp::ReceiverReport(ReceiverReport {
                            payload: receiverreport.payload().to_vec(),
                            padding: receiverreport.get_padding(),
                            version: receiverreport.get_version(),
                            ssrc: receiverreport.get_ssrc(),
                            pkt_length: receiverreport.get_pkt_length(),
                            packet_type: receiverreport.get_packet_type(),
                            rx_report_count: receiverreport.get_rx_report_count(),
                        })
                    }
                    _ => {
                        unreachable!();
                    }
                };

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
