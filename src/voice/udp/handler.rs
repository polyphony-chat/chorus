// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
use tokio::sync::{Mutex, RwLock};

use super::UdpBackend;
use super::UdpSocket;

use super::RTP_HEADER_SIZE;
use crate::errors::VoiceUdpError;
use crate::types::VoiceEncryptionMode;
use crate::voice::crypto::get_xsalsa20_poly1305_lite_nonce;
use crate::voice::crypto::get_xsalsa20_poly1305_nonce;
use crate::voice::crypto::get_xsalsa20_poly1305_suffix_nonce;
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
    /// Spawns a new UDP handler and performs IP discovery.
    ///
    /// Mutates the given data_reference with the IP discovery data.
    pub async fn spawn(
        data_reference: Arc<RwLock<VoiceData>>,
        url: SocketAddr,
        ssrc: u32,
    ) -> Result<UdpHandle, VoiceUdpError> {
        let udp_socket = UdpBackend::connect(url).await?;

        // First perform ip discovery
        let ip_discovery = IpDiscovery {
            pkt_type: IpDiscoveryType::Request,
            ssrc,
            length: 70,
            address: Vec::new(),
            port: 0,
            payload: Vec::new(),
        };

        // Minimum size with an empty Address value, + 64 bytes for the actual address size
        let size = IpDiscoveryPacket::minimum_packet_size() + 64;

        let mut buf: Vec<u8> = vec![0; size];

        // Safety: expect is justified here, since this is an error which should never happen.
        // If this errors, the code at fault is the buffer size calculation.
        let mut ip_discovery_packet =
            MutableIpDiscoveryPacket::new(&mut buf).expect("Mangled ip discovery packet creation buffer, something is very wrong. Please open an issue on the chorus github: https://github.com/polyphony-chat/chorus/issues/new");

        ip_discovery_packet.populate(&ip_discovery);

        let data = ip_discovery_packet.packet();

        debug!("VUDP: Sending Ip Discovery {:?}", &data);

        let send_res = udp_socket.send(data).await;
        if let Err(e) = send_res {
            return Err(VoiceUdpError::BrokenSocket {
                error: format!("{:?}", e),
            });
        }

        debug!("VUDP: Sent packet discovery request");

        // Handle the ip discovery response
        let received_size_or_err = udp_socket.recv(&mut buf).await;

        if let Err(e) = received_size_or_err {
            return Err(VoiceUdpError::BrokenSocket {
                error: format!("{:?}", e),
            });
        }

        let receieved_ip_discovery = IpDiscoveryPacket::new(&buf).expect("Could not make ipdiscovery packet from received data, something is very wrong. Please open an issue on the chorus github: https://github.com/polyphony-chat/chorus/issues/new");

        debug!(
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

        Ok(UdpHandle {
            events: shared_events,
            socket,
            data: data_reference,
        })
    }

    /// The main listen task;
    ///
    /// Receives UDP messages and parses them.
    async fn listen_task(&mut self) {
        loop {
            // FIXME: is there a max size for these packets?
            // Allocating 512 bytes seems a bit extreme
            //
            // Update: see <https://stackoverflow.com/questions/58097580/rtp-packet-maximum-size>
            // > "The RTP standard does not set a maximum size.."
            //
            // The theoretical max for this buffer would be 1458 bytes, but that is imo
            // unreasonable to allocate for every message.
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
                trace!("VUDP: Parsed packet as rtp! {:?}", buf);

                let decryption_result = self.decrypt_rtp_packet_payload(&rtp).await;

                if let Err(err) = decryption_result {
                    match err {
                        VoiceUdpError::NoKey => {
                            warn!("VUDP: Received encyrpted voice data, but no encryption key, CANNOT DECRYPT!");
                            return;
                        }
                        VoiceUdpError::FailedDecryption => {
                            warn!("VUDP: Failed to decrypt voice data!");
                            return;
                        }
                        _ => {
                            error!("VUDP: Failed to decrypt voice data: {}", err);
                            return;
                        }
                    }
                }

                let decrypted = decryption_result.unwrap();

                trace!("VUDP: Successfully decrypted voice data!");

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

    /// Decrypts an encrypted rtp packet, returning a decrypted copy of the packet's payload
    /// bytes.
    ///
    /// # Errors
    /// If we have not received an encryption key, this returns a [VoiceUdpError::NoKey] error.
    ///
    /// If the decryption fails, this returns a [VoiceUdpError::FailedDecryption].
    pub async fn decrypt_rtp_packet_payload(
        &self,
        rtp: &discortp::rtp::RtpPacket<'_>,
    ) -> Result<Vec<u8>, VoiceUdpError> {
        let packet_bytes = rtp.packet();

        let mut ciphertext: Vec<u8> =
            packet_bytes[(RTP_HEADER_SIZE as usize)..packet_bytes.len()].to_vec();

        let session_description_result = self.data.read().await.session_description.clone();

        // We are trying to decrypt, but have not received SessionDescription yet,
        // which contains the secret key
        if session_description_result.is_none() {
            return Err(VoiceUdpError::NoKey);
        }

        let session_description = session_description_result.unwrap();

        let nonce_bytes = match session_description.encryption_mode {
            VoiceEncryptionMode::Xsalsa20Poly1305 => get_xsalsa20_poly1305_nonce(packet_bytes),
            VoiceEncryptionMode::Xsalsa20Poly1305Suffix => {
                // Remove the suffix from the ciphertext
                ciphertext = ciphertext[0..ciphertext.len() - 24].to_vec();
                get_xsalsa20_poly1305_suffix_nonce(packet_bytes)
            }
            // Note: Rtpsize is documented by userdoccers to be the same, yet decryption
            // doesn't work.
            //
            // I have no idea how Rtpsize works.
            VoiceEncryptionMode::Xsalsa20Poly1305Lite => {
                // Remove the suffix from the ciphertext
                ciphertext = ciphertext[0..ciphertext.len() - 4].to_vec();
                get_xsalsa20_poly1305_lite_nonce(packet_bytes)
            }
            _ => {
                error!(
                    "This voice encryption mode ({:?}) is not yet implemented.",
                    session_description.encryption_mode
                );
                return Err(VoiceUdpError::EncryptionModeNotImplemented {
                    encryption_mode: format!("{:?}", session_description.encryption_mode),
                });
            }
        };

        let key = GenericArray::from_slice(&session_description.secret_key);

        let decryption_result;

        if session_description.encryption_mode.is_xsalsa20_poly1305() {
            let nonce = GenericArray::from_slice(&nonce_bytes);

            let decryptor = XSalsa20Poly1305::new(key);

            decryption_result = decryptor.decrypt(nonce, ciphertext.as_ref());
        }
        // Note: currently unused because I have no idea what the AeadAes256Gcm nonce is
        /*else if session_description.encryption_mode.is_aead_aes256_gcm() {
            let nonce = GenericArray::from_slice(&nonce_bytes);

            let decryptor = Aes256Gcm::new(key);

            decryption_result = decryptor.decrypt(nonce, ciphertext.as_ref());

        }*/
        else {
            error!(
                "This voice encryption mode ({:?}) is not yet implemented.",
                session_description.encryption_mode
            );
            return Err(VoiceUdpError::EncryptionModeNotImplemented {
                encryption_mode: format!("{:?}", session_description.encryption_mode),
            });
        }

        // Note: this may seem like we are throwing away valuable error handling data,
        // but the decryption error provides no extra info.
        if decryption_result.is_err() {
            return Err(VoiceUdpError::FailedDecryption);
        }

        Ok(decryption_result.unwrap())
    }
}
